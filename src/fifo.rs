// This file contains code to manage the FIFO pipe and read from it.

use super::signal::Signal;
use std::io::prelude::*;
use std::sync::Arc;

// A fifo pipe
#[derive(Debug)]
pub struct Pipe {
  pub filepath: std::path::PathBuf,
}

pub enum CreateOutcome {
  Ok(),
  AlreadyExists(String), // a pipe already exists at the given path
  OtherError(String),    // other error creating the pipe
}

impl Pipe {
  // creates the FIFO on the filesystem
  pub fn create(&self) -> CreateOutcome {
    match nix::unistd::mkfifo(&self.filepath, nix::sys::stat::Mode::S_IRWXU) {
      Ok(_) => CreateOutcome::Ok(),
      Err(err) => match err.as_errno() {
        None => panic!("cannot determine error code"),
        Some(err_code) => match err_code {
          nix::errno::Errno::EEXIST => CreateOutcome::AlreadyExists(self.path_str()),
          _ => CreateOutcome::OtherError(err.to_string()),
        },
      },
    }
  }

  pub fn delete(&self) {
    std::fs::remove_file(&self.filepath).expect("cannot delete pipe");
  }

  pub fn open(&self) -> std::io::BufReader<std::fs::File> {
    let file = std::fs::File::open(&self.filepath).unwrap();
    std::io::BufReader::new(file)
  }

  // provides the path of this pipe as a string
  pub fn path_str(&self) -> String {
    self.filepath.display().to_string()
  }
}

// constructs a fifo pipe in the current directory
pub fn in_dir(dirpath: &std::path::PathBuf) -> Pipe {
  Pipe {
    filepath: dirpath.join(".tertestrial.pipe"),
  }
}

pub fn listen(pipe: &Arc<Pipe>, sender: std::sync::mpsc::Sender<Signal>) {
  let pipe = Arc::clone(&pipe);
  std::thread::spawn(move || {
    loop {
      // TODO: don't create a new BufReader for each line
      for line in pipe.open().lines() {
        match line {
          Ok(text) => sender.send(Signal::Line(text)).unwrap(),
          Err(err) => {
            println!("error reading line: {}", err);
            sender.send(Signal::Exit).unwrap();
            break;
          }
        };
      }
    }
  });
}

//
// ----------------------------------------------------------------------------
//

#[test]
fn pipe_create_does_not_exist() -> Result<(), std::io::Error> {
  let temp_path = tempfile::tempdir().unwrap().into_path();
  let pipe = in_dir(&temp_path);
  match pipe.create() {
    CreateOutcome::Ok() => {}
    _ => panic!("cannot create pipe"),
  }
  let mut files = vec![];
  for file in std::fs::read_dir(&temp_path)? {
    files.push(file?.path());
  }
  let want = vec![pipe.filepath];
  assert_eq!(want, files);
  std::fs::remove_dir_all(&temp_path)?;
  Ok(())
}

#[test]
fn pipe_create_exists() -> Result<(), std::io::Error> {
  let temp_path = tempfile::tempdir().unwrap().into_path();
  let pipe = in_dir(&temp_path);
  match pipe.create() {
    CreateOutcome::Ok() => {}
    _ => panic!("cannot create first pipe"),
  }
  match pipe.create() {
    CreateOutcome::AlreadyExists(_) => {}
    CreateOutcome::Ok() => panic!("should not create second pipe"),
    CreateOutcome::OtherError(err) => panic!(err),
  }
  std::fs::remove_dir_all(&temp_path)?;
  Ok(())
}

#[test]
fn pipe_delete() -> Result<(), std::io::Error> {
  let temp_path = tempfile::tempdir().unwrap().into_path();
  let pipe = in_dir(&temp_path);
  match pipe.create() {
    CreateOutcome::Ok() => {}
    _ => panic!(),
  }
  pipe.delete();
  let mut files = vec![];
  for file in std::fs::read_dir(&temp_path)? {
    files.push(file?.path());
  }
  assert_eq!(0, files.len());
  std::fs::remove_dir_all(&temp_path).unwrap();
  Ok(())
}
