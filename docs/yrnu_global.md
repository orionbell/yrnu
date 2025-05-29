# Yrnu global
The `yrnu` global provides many useful functions for executing, saving and managing configurations
## Executing configuration
### run
Runs the giving configuration on a remote machine
**Arguments:** 
- host - `String`
- config - `String`
- auth - `Table` (for more details see below) - *optional*
- port - `Number` - *optional*

#### Auth table
The auth table specifies how to authenticate against the remote machine using SSH.
There are 4 ways to authenticate, And those are:
- **Arguments**: By providing the `username` and `password` as arguments to the `Auth table`.
- **Stdin**: By asking the user directly for `username` and `password` (This is the default behavior when auth table not passed).
- **Key pairs**: By providing the `Path` to the `public` and `private` keys.
- **Agent**: By providing an agent name to the `Auth table`.
##### Fields
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

#### Result table
##### Fields
- success - `Bool`
- status_code - `Number`
- output - `String`
- stdout - `String`
- stderr - `String`
#### Example 1 (success)
```lua
host = "192.168.1.1"
cmd = "whoami"
keys = { public = Path("~/.ssh/id_rsa.pub"), private = Path("~/.ssh/id_rsa") }
auth = {
    username = "user",
    keys = keys
}
result = yrnu.run(host, cmd, auth)
print(result.success)       -- true
print(result.status_code)   -- 0
print(result.output)        -- user
```
#### Example 2 (failure)
```lua
host = "10.10.0.1"
cmd = "notexists"
auth = {
    username = "user",
    password = "Very$ecret"
}
result = yrnu.run(host, cmd, auth, 2222)
print(result.false)       -- true
print(result.status_code)   -- 127
print(result.output)        -- zsh:1: command not found: notexists
```
>[!NOTE]
> `output` would contain `stdout` on success and `stderr` on failure.
---
### exec
Runs the giving configuration on the local machine
**Arguments:** 
- config - `String`
- options - `Table` (for more details see below) - *optional*
#### Options table
- shell - `String` - the shell to use to run the commands
- stdin - `String` - string to pass to the command as the stdin
**Returns:** `Table` - in case of a single command it returns a `result` (see below) table, in case of multiple commands it returns an array of `result` tables one for each command

#### Example 1 (success)
```lua
result = yrnu.exec("whoami",{shell = "zsh"})
print(result.success)       -- true
print(result.status_code)   -- 0
print(result.output)        -- user
```
#### Example 2 (failure)
```lua
result = yrnu.exec("notexists")
print(result.success)       -- false
print(result.status_code)   -- 127
print(result.output)        -- sh: line 1: notexists: command not found
```
## Matching Regex
### match
Matches a giving string with a giving regular expression
**Arguments:** 
- regex - `String` - the regular expression string
- str - `String` - the string to match
**Returns:** `boolean` - `true` if `str` matches the `regex` regular expression, false otherwise.
#### Example
```lua
email_regex = [[^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$]]
if yrnu.match(email_reg, "test@example.com") then
    print("Valid")
end
```
## Serializing and Deserializing
### serialize
Serialize Lua `Table` to supported format.
Currently supported formats are: `json`, `yaml`, `toml`, `xml`, `csv`.
Some need specific format of `Table` in order to serialize them others can be serialize natively.
Currently the following formats are natively supported and don't need specific structure: `json`, `yaml`, `toml`.
**Arguments:** 
- table - `Table` - the table to serialize.
- fmt - `String` - the format to serialize to.
**Returns:** `String` or `nil` - if case of successfully serializing the serialize string would be returned otherwise `nil` would be returned.
#### Example 1 (native support)
```lua
data = {
    name = "tim",
    age = 21,
    is_cool = true,
    hobbies = {
        "reading",
        "video games",
        "drawing",
    }
}
print(yrnu.serialize(data, "json"))
print(yrnu.serialize(data, "yaml"))
print(yrnu.serialize(data, "toml"))
```
output:
```json
{
    "is_cool": true,
    "name": "tim",
    "hobbies": [
        "reading",
        "video games",
        "drawing"
    ],
    "age": 21
}
```
```yaml
---
is_cool: true
name: tim
hobbies:
  - reading
  - video games
  - drawing
age: 21
```
```toml
age = 21
hobbies = [
    "reading",
    "video games",
    "drawing",
]
is_cool = true
name = "tim"
```

#### Example 2 (XML)
The following 4 table formats are supported for `XML` serialization:
```lua
comment = { comment = "Hello World" }
-- "encoding" and "standalone" are optional
declaration = { version = 1.2, encoding = "utf-8", standalone = "yes" }
doctype = { doctype = "html" }
empty_tag = { name = "code", attributes = { id = "123321" }, self_closed = true }
any_other_tag = { name = "person", attributes = { attrib = 0 }, children = { "Hello", { comment = "test" }, { name = "test", is_empty = true } }}
print(yrnu.serialize({ comment, declartion, doctype, empty_tag, any_other_tag }, "xml"))
```
output:
```xml
<!--Hello World-->
<?xml version="1.2" encoding="utf-8" standalone="yes"?>
<!DOCTYPE html>
<code id="123321"/>
<person attrib="0">
    Hello
    <!--test-->
    <test/>
</person>
```

#### Example 3 (CSV)
The following 3 table formats are supported for `CSV` serialization. 
```lua
csv1 = { 
    -- each sub table is a row and each key is a column (should be the same across all rows)
    -- This way of serializing is not recommended,
    -- The reason being that the order which the column would be shown is undetermind.
    { col1 = "val1", col2 = "val2", col3 = "val3" },
    { col1 = "val4", col2 = "val5", col3 = "val6" },
}
csv2 = {
    { "col1", "col2", "col3" },
    { "val1", "val2", "val3" },
    { "val4", "val5", "val6" } 
}
csv3 = { 
    { "val1", "val2", "val3" },
    { "val4", "val5", "val6" }
}
print(yrnu.serialize(csv1, "csv"))
-- When not specifing the headers option the first row would be the headers
print(yrnu.serialize(csv2, "csv")) 
print(yrnu.serialize(csv3, "csv", { headers = { "col1", "col2", "col3" } }))
```
output:
```csv
col2,col1,col3
val2,val1,val3
val5,val4,val6
```
```csv
col1,col2,col3
val1,val2,val3
val4,val5,val6
```
```csv
col1,col2,col3
val1,val2,val3
val4,val5,val6
```
### deserialize
This function does - how the name my suggests - the reverse of the `serialize`, it takes supported formats strings 
and parse it to a Lua `Table`, exactly how you would pass it to the `serialize` function.

To demonstrate this look at the example below
```lua
json_str = [[
{
    "is_cool": true,
    "name": "tim",
    "hobbies": [
        "reading",
        "video games",
        "drawing"
    ],
    "age": 21
}
]]
xml_str = [[
<!--Hello World-->
<?xml version="1.2" encoding="utf-8" standalone="yes"?>
<!DOCTYPE html>
<code id="123321"/>
<person attrib="0">
    Hello
    <!--test-->
    <test/>
</person>
]]
csv_str = [[
col1,col2,col3
val1,val2,val3
val4,val5,val6
]]

print(yrnu.serialize(yrnu.deserialize(json_str, "json"), "json"))
print(yrnu.serialize(yrnu.deserialize(xml_str, "xml"), "xml"))
print(yrnu.serialize(yrnu.deserialize(csv_str, "csv"), "csv"))
```
output:
```json
{
    "name": "tim",
    "age": 21,
    "is_cool": true,
    "hobbies": [
        "reading",
        "video games",
        "drawing"
    ]
}
```
```xml
<!--Hello World-->
<?xml version="1.2" encoding="utf-8" standalone="yes"?>
<!DOCTYPE html>
<code id="123321"/>
<person attrib="0">
    Hello
    <!--test-->
    <test/>
</person>
```
```csv
col2,col3,col1
val2,val3,val1
val5,val6,val4
```
>[!NOTE]
> Notice that the column order has been changed, the reason being, for ease of use
> the `deserialize` function returns the first way of defining a `CSV` table.
> 
> For example: 
> ```lua
> csv_table = yrnu.deserialize("col1,col2,col3\nval1,val2,val3\nval4,val5,val6", "csv")
> print(csv_table[1].col1) -- val1
> print(csv_table[2].col3) -- val6
> ```
