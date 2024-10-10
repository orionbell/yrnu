# Yrnu

Yrnu is a `Rust` crate that provides simple and easy api for automating network administrators and cyber spaciliest tasks.
> [!NOTE]
> This project is still in development and not usable yet.

### current program stage
- [x] address module
- [ ] packet module
- [ ] config module
- [ ] server module

### modules
- address - tools to handle IP and Mac addresses as well as Network definitions
- packet - tools to construct varius types of network traffic messages as well as sniffing packets
- config - tools to configure network devices automaticly using ssh
- server - tools to spawn varius types of servers 

### Examples
#### address module
```lua
local address = require('yrnu.address')
local ip = address.IpAddress('192.168.1.1')
local mac1 = address.MacAddress('ff:ee:ff:11:22:33')
local mac2 = address.MacAddress('ef:ee:ff:11:22:33')
local mask = address.Mask('255.255.255.128')
local net1 = address.Network('10.0.1.0/27')
local net2 = address.Network(ip,mask)

print(net1.contines(ip))        -- false
print(mac1 > mac2)              -- true
print(ip.type())                -- Private
print(ip.version())             -- V4
print(net2.broadcast())         -- 192.168.1.127
print(net2.broadcast().type())  -- Broadcast
print(mac2.eui64())             -- fe80::edee:ffff:fe11:2233
```

### Shell Usage

- Opening interactive interpreter: `yrnu`
- Running a lua file: `yrnu script.lua`
- Send 5 icmp packets: `yrnu -s icmp -n 5`
- Sniff the next 10 packets and save them as pcap file: `yrnu -S 10 --save`
- Start interactive dialog to configure STP remotely: `yrnu -c STP -t 192.168.1.1`
- Run command remotely (using ssh): `yrnu -r 'whoami'`
- Start an HTTP server and serve the files under giving directory: `yrnu --server http --dir ./src`
- etc.

### Core features

- Sending custom network traffic
- Constructing network device configurations and auto configure them using ssh
- Spawn varius types of servers
- Sniff packets
