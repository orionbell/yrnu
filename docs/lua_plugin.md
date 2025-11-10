# Yrnu Plugins
The major strength of Yrnu is it's scalability, Creating plugin is a very straight forward process and it can be used to automate any task or app possible.
Yrnu provides a basic tools that can be utilized in order to complete complex tasks.

Plugin acts as a wrapper for an application or specified tasks or workflow, this done by defining arguments and using them in the `config` function.

Yrnu takes those arguments and expose API's to setting and validating them, the user only needs to set him arguments via one of the API's and run the `config` method that would
do the rest of the configuration.
## Plugin structure
Every plugin is simply a directory contains global Lua files (global in that context means in the root of that directory) those are the only files that would be parsed.
Here is a simple diagram example of plugin structure:
```
plugin_name/
    init.lua     < Special file for plugin settings
    global1.lua
    global2.lua
    subdir/
        file.lua < would not be parsed unless imported in the global files
```
## Defining a plugin global
Plugins are `Lua` files that returns specific kind of `table`

Here's a global file example:
```lua
-- global.lua
return {
    about = "about string",            -- optional
    preconfig = "preconfig",           -- optional
    postconfig = "postconfig",         -- optional
    init = function(self, table) end,  -- optional
    config = function(self) end,       
    args = {
        arg_name = { ... }
    },
    subcommands = {                    -- optional
        scmd_name = require("plugins.plugin_name.subdir.file")
    },
}
```
Let's break it one piece at a time.

- `about` - Optional text that would be printed when using that global via the CLI.
- `preconfig` - Optional text that would be added to every `config` before the `config` output
- `postconfig` - Same as `preconfig` but added after the `config` output
- `init` - Optional global constructor, takes the `self` (the config table) and optional params table for additional parameters if needed. 
- `config` - The global config function, takes only `self`.
- `args` - The arguments table, every field in this table should be argument definition.
- `subcommands` - The subcommands table, every field in this table is a subcommand (or sub config) for this global.
Subcommands defined exactly how globals defined but imported in this table instead of being in the root of the plugin and can be access only via this global in all the APIs.
## Defining a global argument
Arguments is the smallest component in the plugin and it plays as a small puzzle piece in the config puzzle.
Here's an argument definition example:
```lua
{
    index = 1,
    required = true,
    short = "a",
    long = "arg-name",
    action = "store-true",
    arg_type = "boolish"
    update = function(config, value)
        config["arg_name"] = value
    end,
    delimiter = ",",
    wizard = "turn on?: "
}

```
Again let's break it one piece at a time.

- `index` - wizard index ordering (lower is first) - specifies the order which the wizard would ask for the arguments (default is random).
- `required` - does this argument is required (defaults to `false`)
- `short` - the short CLI flag for this argument (defaults to the first arg name character)
- `help` - the argument help message
- `long` - long CLI flag for this argument (defaults to arg name replacing `_` with `-`)
- `possible_values` - list of argument possible "string" values
- `action` - this action would be taken automatically when using the CLI usage (more on that later).
    Supported actions are:
    - `store-true`
    - `store-false`
    - `store-count`
    - `store-table`

- `arg_type` - The argument type, this would be used to validate and give the right type to the update function (defaults to `string`).
    Supported types are:
    - `boolish` and  `bool` - `true` or `false` the only different between the two in the CLI, The `boolish` would accept `yes` or `on` as `true` and `no` or `off` as `false` but `bool` would accept only `true` or `false`
    - `int` and `uint` 64bit integer and unsigned 64 bit integer.
    - `real` - real numbers (a.k.a float)

    - `ip-address` - IP addresses
    - `mac-address` - MAC addresses
    - `mask` - Subnet masks
    - `network` - Network in the `netid/prefix` format
    - `interface` - valid network interface on the machine
    - `path` - File and Directories Paths
    - `url` - URLs
    - `string`
- `update` - The setter method for this argument, this function should add the value that been given to the config table if valid (defaults to setter that sets that adds arg name field to the config table).
- `delimiter` - The delimiter would be used for distinguishing between values when using `store-table` action.
- `wizard` - The wizard question that would be asked for this argument (defaults to the arg name followed by `: `).
### Shorten ways to define an argument
Because almost every property has a default value there are 4 shorten ways to define an argument (instead of the formal table) in order to reduce boilerplate.
1. single character `string`
```lua
    arg_name = "a" 
    -- same as:
    -- arg_name = { short = "a" }
```
1. `string`
```lua
    arg_name = "this is an help message" 
    -- same as:
    -- arg_name = { help = "this is an help message" }
```
1. `boolean`
```lua
    arg_name = true 
    -- same as:
    -- arg_name = { required = true }
```
1. `function`
```lua
    arg_name = function(self, value)
        self["arg_name"] = value
    end
    -- same as:
    -- arg_name = { update = function(self, value) self["arg_name"] = value end }
```
## The plugin `init.lua` file
The plugin `init.lua` file is an optional file that can be used to configure the plugin behavior.
Let's look on a plugin `init.lua` file template:
```lua
return {
    description = "",
    public = {},
    private = {},
    dependencies = {
        programs = {},
        plugins = {},
    },
}
```
- `description` - a description about the plugin and usage (this mostly should be used as a TL;DR for the plugin help message)
- `public` - When using the `public` table only globals that would be specified in this table would be available through the plugin table the others would be used only internally by the plugin 
- `private` - Only globals that been specified in the `private` table would be inaccessible from the plugin table.
- `dependencies` - A table for specifying the plugin dependencies. If dependencies is written directly in the dependencies table (not in the `programs` nor `plugins` sub tables) it defaults to program.

## Plugin example
In this section we would build a simple `Docker` plugin.
The user would be able to:
1. Define the image and tag
2. Define optional name for the container
3. Define whether to run the container as a daemon
4. Add port forwarding

Let's begin!

First we will specify the bare metal.
Create a new directory named `Docker` and create a new file named `container.lua` with the following:
```lua
return {
    about = "Docker plugin",
    init = function(self, props)
        if props.image then
            self.image = props.image
            return self
        end
    end
    config = function(self) end,
    args = {},
}
```
This specifies the basic structure of the plugin, let's define the first argument,
this argument is the `image` argument.
```lua
args = {
    image = {
        required = true,
        short = "i",
        long = "image",
        update = function(self, value) end
    }
}
```
This defines a new argument in the global `container` named `image`, makes it required with `i` and `image` being the short and long CLI flags for this argument.
Before we would take a look on the `update` method it is important to notice that the `i` and `image` are the default ones so specifying them is for the sake of the example.

Now let's look on the `update` method.
The `update` function is the setter method that would be called when the user use the global API to assigned a new value for this argument.
The simplest `update` method definition (which is the default if not specified) is:
```lua
update = function(self, value)
    self["image"] = value
end
```
This only creates a new field with the argument name and sets it to the value, only type checking is done.
A lot of those definitions are the defaults so we can short it down to:
```lua
args = {
    image = true
}
```
Nice, let's continue, this time a little quicker.

```lua
args = {
    image = true,
    name = "specifies the container name",
    daemon = {
        arg_type = "boolish",
        action = "store-true",
    }
}
```
The last argument is a little more complex, so a more complex setup is needed.

We would want to enable the user to specify port forwarding in two ways.
1. Via giving a table of string in the format of `host:container`
2. By giving host port as the table key and the container as the value for that key
Examples for such tables are:
```lua
ports_str = {"443:443", "80:80"}
ports_num = {[443] = 443, [80] = 80}
```
But give the config table only the second type of table.
Okay let's start working, first let's define the easy stuff
```lua
ports = {
    action = "store-table",
    delimiter = "," -- The default when specifing store-table action
    update = function(self, values) end
}
```
We use the `store-table` action in order to specify that we want a list of values, 
and the delimiter is for the character that would be used in order the distinguish between values.
Though we specify it as a `,` we could skip that because this is the default thus not needed.

Let's move on to the `update` method.
```lua
update = function(self, values)
    local ports = {}
    local parts = {}
    for i, v in pairs(values) do
        if type(v) == "number" then
            ports[i] = v
        elseif type(v) == "string" then
        if yrnu.match("\\d:\\d", v) then
            for str in string.gmatch(v, "([^:]+)") do
                table.insert(parts, tonumber(str))
            end
            ports[parts[1]] = parts[2]
            parts = {}
        end
    end
    self.ports = ports
end
```
First we iterate over the values and checks whether the value is a `string` or a `number`.
If it's a number we assume it already in the right format. If it is a string we make sure it is in the right format
by using the built-in `match` function in the global `yrnu` global (see [Yrnu global](/yrnu_global.md) for more info).
If it is in the right format we split the string by the `:`, converting them to numbers and adding them to the table.

Okay we finished the argument definitions let's move on to the second part of this example.
Now we should define what the plugin does with the argument that been defined. this being done
by implementing the `config` method.
```lua
config = function(self)
    local conf = "docker run "
    if self.daemon then
        conf = conf .. "-d "
    end
    if self.name then
        conf = conf .. "--name " .. self.name .. " "
    end
    if self.ports then
        for i, v in pairs(self.ports) do
            conf = conf .. "-p " .. i .. ":" .. v .. " "
        end
    end
    return conf .. self.image
end
```
Here we slowly construct the `docker` command and return it.

This plugin is very minimal and simple but hopefully demonstrate the idea.
