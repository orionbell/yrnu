# Yrnu APIs
For each argument or subcommand Yrnu creates 2 APIs, A `Lua` API and a `CLI` API that is why
before one should use a plugin he first most know which arguments or subcommands the plugin defines

The examples uses the simple `docker` plugin that been created in the previous section.
## Lua
When using the Lua API first we should initialize a new global instance, this being done by accessing a Lua global that present the plugin and running a function named after the global.
If `init` function is defined it would be used here.
```lua
local ctr = docker.container{ image = "ubuntu" }
```
Now each plugin has a set method named `set_{arg_name}`
```lua
ctr:set_name("ubuntu")
ctr:set_daemon(true)
ctr:set_ports{[443] = 443, [80] = 80}
```
When we done configuring the container we use the `config` method to get configure the container.
```lua
print(ctr:config())
-- output: docker run -d --name ubuntu -p "443:443" -p "80:80" ubuntu
```
But because the `config` method is also the `tostring` function of that table we could just print the table itself.
```lua
print(ctr)
-- output: docker run -d --name ubuntu -p "443:443" -p "80:80" ubuntu
local res = yrnu.exec(tostring(ctr))
print(res.status_code)
```
## CLI
The CLI usage is as you might expected, for each argument we specify the value and the config method is runs behind the scenes and the output is returned.
```sh 
yrnu config docker container -d -n ubuntu -i ubuntu -p "443:443","80:80"
root# output: docker run -d --name ubuntu -p "443:443" -p "80:80" ubuntu
```
In this example it is kind of useless to do it like so, but in more complex tasks this can be really productive and useful.
### Using the wizard
For each global there is also a wizard - a way to interactively provide the values for each argument, this can be used to really hide the complex procedures behind friendly wizard.
An example for this can be seen as something like this in the terminal:
```sh
root# yrnu config docker container -w
#image: ubuntu
#daemon: 
#name: ubuntu
#ports: "443:443", "80:80"

#docker run -d --name ubuntu -p "443:443" -p "80:80" ubuntu
```
