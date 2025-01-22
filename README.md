# Yrnu

Yrnu is a `Rust` tool providing a simple and easy lua api for creating automation tools for network administrators and cyber spaciliest tasks.
> [!NOTE]
> This project is still in development and not usable yet.
> 
> Currently most of the development is done locally 

### current program stage
- [x] core module - Mostly Done
- [ ] config module - In Local Development
- [ ] packet module
- [ ] server module

### modules
- core   - tools to handle IP and Mac addresses as well as Network definitions
- packet - tools to construct varius types of network traffic messages as well as sniffing packets
- config - tools to configure network devices automaticly using ssh
- server - tools to spawn varius types of servers 

### Examples
#### address module
```lua
local ip_str = "10.0.1.254
if IpAddress.is_valid(ip_str) then
    local ip = IpAddress.new(ip_str)
    print(ip) -- 10.0.1.255 is version 4 private address
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
- Running a lua script: `yrnu script.lua`
- Send 5 icmp packets to `10.0.0.1` : `yrnu send icmp -n 5 -t 10.0.0.1`
- Sniff the next 10 packets and save them as pcap file: `yrnu sniff -n 10 -s`
- Configure EIGRP using yrnu: `yrnu config router -H RTR -s 12211 -p 443322 -E true -6 true --banner "^Hello World^" eigrp --as-number 132 --networks 192.168.1.0/24,10.0.0.0/22,1.1.1.0/16 --redistribute Static,rip,"ospf 1","eigrp 23" --passive-interfaces "gigabit 0/0/0","gigabit 0/1/0"`
- Run command remotely (using ssh): `yrnu config -r 'whoami'`
- Start an HTTP server and serve the files under giving directory: `yrnu server http -d ./src`
- etc.

### Core features

- Sending custom network traffic
- Constructing devices configurations and auto configure them using ssh
- Spawn varius types of servers
- Sniff packets
