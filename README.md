# Yrnu

Yrnu is a `Rust` program that provides simple and easy lua interface for - but not limited to - automating network administrators and cyber spaciliest tasks.

> [!NOTE]
> This project is under the GPLv3 license,
> 
> For more permissive licenses please contact me

### current program stage
- [x] address module ([wiki](https://github.com/orionbell/yrnu/wiki/core))
- [ ] config module - currently in development
- [ ] packet module
- [ ] server module

### modules
- address - tools to handle IP and Mac addresses as well as Network definitions
- packet - tools to construct varius types of network traffic messages as well as sniffing packets
- config - tools to configure network devices automaticly using ssh
- server - tools to spawn varius types of servers 

### Examples
#### core functionality
```lua
local ip_str = "10.0.1.255"
if IpAddress.is_valid(ip_str) then
    local ip = IpAddress.new(ip_str)
    print(ip) -- 10.0.1.255 is version 4 broadcast address
    local oct = ip:get_octats()
    for i in pairs(oct) do
        print(oct[i]) -- 10 0 1 255
    end
end
local mask = Mask.from_prefix(22)
print(IpKind.get_broadcast("10.0.0.0",mask)) -- 10.0.3.255 is version 4 broadcast address
```
#### A simple router configuration
This example uses the [netdev_config](https://github.com/orionbell/netdev_config) yrnu plugin
```lua
local rtr = router()
rtr:set_hostname("RTR-NYC-FL1")
rtr:set_secret("#bestpa$$")
rtr:set_password("d0ntus3pa44")
rtr:set_password_enc(true)
rtr:set_enable_ipv6(true)
rtr:add_ospf({ pid = 1 })
local id = IpAddress.new("1.1.1.1")
print(id.kind)
print(id.version)
rtr.ospf:set_router_id(id)
print(rtr:config())
yrnu.run("192.168.1.254", rtr:config())
```
Output
```
public
version 4
hostname RTR-NYC-FL1
enable secret #bestpa$$
enable password d0ntus3pa44
ipv6 unicast-routing
service password-encryption
router ospf 1
router-id 1.1.1.1
```
### Shell Usage

- Opening interactive lua interpreter: `yrnu`
- Running a lua script: `yrnu script.lua`
- Send 5 icmp packets: `yrnu packet send icmp -n 5`
- Sniff the next 10 packets and save them as pcap file: `yrnu packet sniff -n 10 --save`
- Start interactive dialog to configure STP remotely: `yrnu config -i switch stp -r 192.168.1.12`
- Start an HTTP server and serve the files under giving directory: `yrnu server http --dir ./src`
- etc.

### Core features

- Sending custom network traffic
- Constructing network device configurations and auto configure them using ssh
- Spawn varius types of servers
- Sniff packets

### Contributions
I am by no means a senior Rust developer, contributions are more than welcome

