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
