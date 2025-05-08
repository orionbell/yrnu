# Yrnu global
The `yrnu` global provides many useful functions for executing, saving and managing configurations
### Executing configuration on local or remote machines
#### run
Runs the giving configuration on a remote machine
**Arguments:** 
- host - `String`
- config - `String`
- auth - `Table` (for more details see below) - *optional*
- port - `Number` - *optional*

##### Auth table
The auth table specifies how to authenticate against the remote machine using SSH.
There are 4 ways to authenticate, And those are:
- **Arguments**: By providing the `username` and `password` as arguments to the `Auth table`.
- **Stdin**: By asking the user directly for `username` and `password` (This is the default behavior when auth table not passed).
- **Key pairs**: By providing the `Path` to the `public` and `private` keys.
- **Agent**: By providing an agent name to the `Auth table`.
###### Fields
To authenticate using the `Arguments` way use:
- username - `String`
- password - `String`
To authenticate using the `Key pairs` way use:
- username - `String`
- keys - `Table`
  - private - `String`
  - public - `String`
To authenticate using the `Agent` way use:
- agent - `String`

**Returns:** `Table` - in case of a single command it returns a `result` (see below) table, in case of multiple commands it returns an array of `result` tables one for each command

##### Result table
###### Fields
- success - `Bool`
- status_code - `Number`
- output - `String`
- stdout - `String`
- stderr - `String`
##### Example
```lua
```
---
#### exec
Runs the giving configuration on the local machine
**Arguments:** 
- config - `String`
- options - `Table` (for more details see below) - *optional*
##### Options table
- shell - `String` - the shell to use to run the commands
- stdin - `String` - string to pass to the command as the stdin
**Returns:** `Table` - in case of a single command it returns a `result` (see below) table, in case of multiple commands it returns an array of `result` tables one for each command

##### Example
```lua
```
### Matching specific syntax using Regex
#### reg_match
### Serializing and Deserializing data
#### serialize
#### deserialize

