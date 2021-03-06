require! {
  './helpers/error-message' : {abort}
  './helpers/file-type'
  fs
  path
  'prelude-ls' : {capitalize, map, obj-to-pairs}
  'remove-value'
  'require-uncached'
  'require-yaml'
  util
}


# Represents the Tertestrial config file
#
# Config files can be written in a variety of languages
# like JavaScript, CoffeeScript, LiveScript, etc
class ConfigFile

  (@config-path) ->
    | !@exists!  =>  abort 'cannot find configuration file'
    @actions = @content!.actions |> @_standardize-actions
    @_convert-regex @actions


  exists: ->
    try
      fs.stat-sync @config-path


  content: ->
    require-uncached @config-path


  _convert-regex: (action-sets) !->
    for action-set in action-sets
      for actionset-name, actions of action-set
        for action in actions
          for key, value of action.match
            action.match[key] = new RegExp value


  _standardize-actions: (actions) ->
    switch typeof! actions
      | 'Array'  =>  [name: 'default', matches: actions]
      | 'Object' =>  obj-to-pairs(actions) |> map ([name, matches]) -> {name, matches}
      | _        =>  abort "unknown action type: #{util.inspect actions, depth: null}"



module.exports = ConfigFile
