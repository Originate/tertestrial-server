require! {
  'chalk' : {bold, cyan, dim}
  './command-runner' : CommandRunner
  './config-file' : ConfigFile
  'fs'
  './helpers/reset-terminal'
  './helpers/run-mode-checker' : runs-in-foreground
  'interpret'
  'liftoff' : Liftoff
  '../package.json' : pkg
  './pipe-listener' : PipeListener
  './setup-wizard'
  'update-notifier'
}

update-notifier({pkg}).notify!

Tertestrial = new Liftoff name: 'tertestrial', config-name: 'tertestrial', extensions: interpret.extensions
  ..launch {}, (env) ->

    if process.argv.length is 3 and process.argv[2] is 'setup'
      setup-wizard!
      return

    reset-terminal!
    console.log dim "Tertestrial server #{pkg.version}\n"
    if runs-in-foreground!
      console.log "#{bold 'ctrl-c'} to exit"
    else
      console.log "to exit, run #{cyan 'fg'}, then hit #{bold '[ctrl-c]'}\n"

    config = new ConfigFile env.config-path
    command-runner = new CommandRunner config
    pipe-listener = new PipeListener
      ..on 'command-received', command-runner.run-command
      ..listen ->
        console.log '\nrunning'
