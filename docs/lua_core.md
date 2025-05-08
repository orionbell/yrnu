# Core Utils
The core Utils provides basic tools for handling Ip and Mac addresses, 
defining networks and getting network interface information.
> [!tip] Note
> All the fields and the properites are ***Read Only*** unless `writeable` is specified
## IpVersion
The `IpVersion` global present the two types of IP address versions

### Fields
- `V4`
- `V6`
##### Examples
```lua
v4 = IpAddress.V4
v6 = IpAddress["V6"]
print(v4) -- version 4
print(v6) -- version 6
```
### Functions
#### is_v4
Checks if a giving address is a valid Ipv4 address

Arguments:
- address - `string`

Returns: `bool` - `true` if valid Ipv4 `else` otherwise

##### Examples
```lua
if IpVersion.is_v4("192.168.1.1") then
    print("Address is valid Ipv4")
end
```
#### is_v6
Checks if a giving address is a valid Ipv6 address

Arguments:
- address - `string`

Returns: `bool` - `true` if valid Ipv6 `else` otherwise

##### Examples
```lua
if IpVersion.is_v6("2001:db8::8a2e:370:7334") then
    print("Address is valid Ipv4")
end
```

## IpKind
The `IpKind` global present the various kinds of IP addresses

### Fields
- `public`
- `private`
- `loopback`
- `linkLocal`
- `apipa`
- `uniqeLocal`
- `uniqeGlobal`
- `broadcast`
- `netid`
- `multicast`
- `unspecified`
---
### Functions
---
#### `is_public`
Checks if a giving address is a public Ipv4 address

**Arguments:** 
- address - `string`

**Returns:** `boolean`, `true` if public, `false` otherwise

---
#### `is_private`
Checks if a giving address is a private Ipv4 address

**Arguments:** 
- address - `string`

**Returns:** `boolean`, `true` if private, `false` otherwise

---
#### `is_loopback`
Checks if a giving address is a loopback Ipv4 address

**Arguments:** 
- address - `string`

**Returns:** `boolean`, `true` if loopback, `false` otherwise

---
#### `is_linklocal`
Checks if a giving address is a linklocal Ipv6 address

**Arguments:** 
- address - `string`

**Returns:** `boolean`, `true` if linklocal, `false` otherwise

---
#### `is_apipa`
Checks if a giving address is an apipa Ipv4 address

**Arguments:** 
- address - `string`

**Returns:** `boolean`, `true` if apipa, `false` otherwise

---
#### `is_multicast`
Checks if a giving address is a multicast address

**Arguments:** 
- address - `string`

**Returns:** `boolean`, `true` if multicast, `false` otherwise

---
#### `is_unspecified`
Checks if a giving address is a unspecified address

**Arguments:** 
- address - `string`

**Returns:** `boolean`, `true` if unspecified, `false` otherwise

---
#### `is_broadcast`
Checks if a giving address is a broadcast address

**Arguments:**
    - address - `string`,
    -  mask - `Mask`

**Returns:** `boolean`, `true` if is valid broadcast address in the mask range, `false` otherwise

---
#### `is_netid`
Checks if a giving address is a net id

**Arguments:** 
    - address - `string`,
    -  mask - `Mask`

**Returns:** `boolean`, `true` if is a net id in the mask range, `false` otherwise

---
#### `get_kind`
Gets the type of a giving address if valid

**Arguments:** 
- address - `string`

**Returns:** `IpKind` member if valid address otherwise `nil` would be returned

---
#### `get_broadcast`
Gets broadcast by a giving net id and a mask

**Arguments:**
    - netid - `string`,
    -  mask - `Mask`
**Returns:** `IpAddress` if valid net id otherwise `nil` would be returned

---
##### Examples
```lua
if IpKind.is_public("178.123.98.32") then
    print(IpKind.public)
end

print(IpKind.get_kind("2001:db8::8a2e:370:7334")) -- uniqe global
print(IpKind.get_broadcast("192.168.1.0", Mask("255.255.255.0"))) -- 192.168.1.255
```
## IpAddress
the `IpAddress` global present an IP address.

### Properties
- address - `string`
- version - `IpVersion`
- kind - `IpKind`

---
### Functions
#### `IpAddress`
Creates a new `IpAddress` instance

**Arguments:** 
- address - `string`

**Returns:** `IpAddress` - if valid address otherwise `nil` would be returned


##### Example
```lua
addr = IpAddress("10.0.0.12")
print(addr)          -- 10.0.0.12
print(addr.version)  -- version 4
print(addr.kind)     -- private
```
---

#### `is_valid`
Checks if a giving string is a valid IP address

**Arguments:** 
- address - `string`

**Returns:** `boolean`, `true` if valid, `false` otherwise

##### Example
```lua
if IpAddress.is_valid("2001:db8::8a2e:370:7334") or IpAddress.is_valid("1.2.3.4") then
    print("Valid")
end
```
---

#### `expend`
Expends a giving Ipv6 address

**Arguments:**
- address - `string`

**Returns:** `string` - The expended address string if valid, otherwise `nil` would be returned

##### Example
```lua
print(IpAddress.expend("2001:db8::8a2e:370:7334"))
-- 2001:0db8:0000:0000:0000:8a2e:0370:7334
print(IpAddress.expend("2001:db8:0:0:0:8a2e:0370:7334"))
-- 2001:0db8:0000:0000:0000:8a2e:0370:7334
```
---

#### `shorten`
Shorten a giving Ipv6 address

**Arguments:** 
- address - `string`

**Returns:** `string` - The shorten address string if valid, otherwise `nil` would be returned


##### Example
```lua
print(IpAddress.shorten("2001:0db8:0000:0000:0000:8a2e:0370:7334"))
-- 2001:db8::8a2e:370:7334
print(IpAddress.shorten("2001:0db8:0:0:0000:2e:370:7334"))
-- 2001:db8::8a2e:370:7334
```
---

#### `eui64`
Creates a linklocal address from a giving mac using the eui64 algorithm

**Arguments:** 
- mac - `MacAddress`

**Returns:** `IpAddress` - The Ipv6 address that been created using the EUI64 algorithem


##### Example
```lua
addr = IpAddress.eui64(MacAddress("C0:FF:EE:00:00:01"))
print(addr)         -- fe80::c2ff:eeff:fe00:1
print(addr.version) -- version 6
print(addr.kind)    -- linklocal
```
---

### Methods
#### `get_octets`
Get the octets of the address

**Arguments:** 
- self - `IpAddress`

**Returns:** `table` - Array of the address bytes

##### Example
```lua
addr = IpAddress("172.17.0.1")
for oct in pairs(addr:get_octets()) do print(oct) end
-- 172 17 0 1
```
---

#### `get_expended`
Get string representation of the expends Ipv6 address

**Arguments:** 
- self - `IpAddress`

**Returns:** `string` - The expended address string

##### Example
```lua
addr = IpAddress("2001:0db8:0000:0000:0000:8a2e:0370:7334")
print(addr) -- 2001:db8::8a2e:370:7334
print(addr:get_expended()) -- 2001:0db8:0000:0000:0000:8a2e:0370:7334
```

## Mask
The `Mask` global present a network mask.

### Properties
- prefix - `number`
- num_of_hosts - `number`
---
### Functions
#### `Mask`
Creates a new `Mask` instance

**Arguments:**
 - mask - `string`

**Returns:** `Mask` if valid mask otherwise `nil` would be returned

##### Example
```lua
mask = Mask("255.255.255.192")
print(mask)              -- 255.255.255.192
print(mask.prefix)       -- 26
print(mask.num_of_hosts) -- 62
```
---

#### `is_valid`
Checks if a giving mask is a valid network mask

**Arguments:** 
- mask - `string`

**Returns:** `boolean`, `true` if valid, `false` otherwise

##### Example
```lua
if Mask.is_valid("255.255.255.0") then 
    print("Valid") 
end
```
---

#### `from_prefix`
Creates a new `Mask` instance from a giving mask prefix

**Arguments:** 
- prefix - `number`

**Returns:** `Mask` if valid prefix otherwise `nil` would be returned

##### Example
```lua
mask = Mask.from_prefix(12)
print(mask)              -- 255.240.0.0
print(mask.num_of_hosts) -- 1048576
```
---

#### `get_prefix`
Gets the mask prefix from a giving mask

**Arguments:** 
- mask - `string`

**Returns:** `number` if valid mask otherwise `nil` would be returned

##### Example
```lua
print(Mask.get_prefix("255.240.0.0")) -- 12
```

### Methods
#### wildcard
Gets the wildcard representation from a giving mask

**Arguments:** 
- self - `Mask`

**Returns:** `string` - The wildcard address string

##### Example
```lua
print(Mask("255.240.0.0"):wildcard()) -- 0.15.255.255
```

## Network
The `Network` global present an IP network. 

### Properties
- broadcast - `IpAddress`
- netid - `IpAddress`
- mask - `Mask`
---

### Functions
#### `Network`
Creates a new `Network` instance

**Arguments:**
- netid - `IpAddress`
- mask - `Mask`

**Returns:** `Network` if valid net id with the mask range otherwise `nil` would be returned
##### Example
```lua
net = Network(IpAddress("192.168.1.0"), Mask.from_prefix(27))
print(net)           -- 192.168.1.0/27
print(net.id)     -- 192.168.1.0
print(net.broadcast) -- 192.168.1.31
```

---

#### `from`
Creates a new `Network` instance from string in the `{net_id}/{prefix}` format

**Arguments:** 
- net - `string`

**Returns:** `Network` if valid net id with the prefix range and in the `{net_id}/{prefix}` format otherwise `nil` would be returned

##### Example
```lua
net = Network.from("192.168.1.64/27")
```
---

### Methods
#### `contains`
Check if a giving `IpAddress` is part of the network

**Arguments:** 
- self - `Network`
- address - `IpAddress`

**Returns:** `boolean`, true if is part of the network false otherwise

##### Example
```lua
net = Network.from("192.168.1.64/27")
addr = IpAddress("192.168.1.65")
if net:contains(addr) then
    print(addr .. " is part of the network")
end
```
---

#### `contains_str`
Check if a giving string (if valid IP address) is part of the network 

**Arguments:**
- `self`
- address - `string`

**Returns:** `boolean`, true if is part of the network false otherwise

##### Example
```lua
net = Network.from("192.168.1.64/27")
if net:contains_str("192.168.1.65") then
    print(addr .. " is part of the network")
end
```
## MacAddress
The `MacAddress` global that present a mac address userdata.

### Properties
- address - `string`
- vendor - `string`
---
### Functions
#### `MacAddress`
Creates a new `MacAddress` instance

**Arguments:** 
- address - `string`

**Returns:** `MacAddress` if valid mac otherwise `nil` would be returned
##### Example
```lua
mac = MacAddress("54:ee:75:b0:11:34")
print(mac)        -- 54:EE:75:B0:11:34
print(mac.vendor) -- Wistron InfoComm(Kunshan)Co.,Ltd.
```
---
#### `is_valid`
Checks if a giving mac address is a valid

**Arguments:**
- address - `string`

**Returns:** `bool`, `true` if valid, `false` otherwise

##### Example
```lua
if MacAddress.is_valid("aa:bb:cc:11:22:33") then
    print("valid mac")
end
```
---
#### `as_bytes`
Get the parts of the mac address as bytes

**Arguments:** 
- self - `MacAddress`

**Returns:** `table` - array of the mac address bytes
##### Example
```lua
mac = MacAddress("01:23:45:21:43:65")
for i,part in pairs(mac:as_bytes()) do
    print(part) -- 1 35 69 33 67 101  
end
```
### Operators support
`==`,`>`,`>=`
can be used to compare between to giving `MacAddress`s
##### Example
```lua
mac1 = MacAddress("00:11:22:33:44:55:66")
mac2 = MacAddress("00:11:23:33:44:55:66")
mac3 = MacAddress("00:11:23:33:44:55:66")
print(mac1 > mac2)  -- false
print(mac2 >= mac3) -- true

```

## Interface

### Properties
- name - `string`
- index - `number`
- description - `string`
- mac - `MacAddress`
- ipv4 - `IpAddress`
- ipv6 - `IpAddress`
- mask - `Mask`
### Functions
#### `by_index`
Creates a new `Interface` instance as the local machine network interface with the giving index

**Arguments:** 
- index - `number`

**Returns:** `Interface` if interface with that index exists, otherwise `nil` would be returned
##### Example
```lua
-- 1 is the loopback interface index
inf = Interface.by_index(1)
print(inf)
-- ==== lo ====
-- index: 1
-- description: 
-- mac:  00:00:00:00:00:00
-- ipv4: 127.0.0.1                
-- ipv6: 1
-- mask: 255.0.0.0
```

#### `by_name`
Creates a new `Interface` instance as the local machine network interface with the giving name

**Arguments:** 
- name - `string`

**Returns:** `Interface` if interface with that name exists, otherwise `nil` would be returned
##### Example
```lua
-- 1 is the loopback interface index
inf = Interface.by_name("lo")
print(inf)
-- ==== lo ====
-- index: 1
-- description: 
-- mac:  00:00:00:00:00:00
-- ipv4: 127.0.0.1                
-- ipv6: 1
-- mask: 255.0.0.0
```
#### `all`
Gets a `Interface` instances array as all the local machine network interfaces

**Returns:** `table` - array of the availabe `Interface`'s in the current machine

## Path
The `Path` global present a file system path to a file or directory

### Properties
- name - `string` (`writeable`)
- extension - `string` (`writeable`)
- exists - `bool`
- is_file - `bool`
- is_dir - `bool`
- is_relative - `bool`
- is_symlink - `bool`
- parent - `Path`
- children - `table` - array of `Path`

---
### Methods
#### `push`
Adds a value to the path

**Arguments:** 
- self - `Path`
- value - `string`

##### Example
```lua
path = Path("/etc")
path:push("passwd")
print(path) -- /etc/passwd
```
---
#### `join`
Creates a copy of the path and adds to a giving value

**Arguments:** 
- self - `Path`
- value - `string`

**Returns:** `Path` - the modified path
##### Example
```lua
path = Path("/etc")
new_path = path:join("passwd")
print(path) -- /etc
print(new_path) -- /etc/passwd
```
---

## Url 
The `Url` global present a URL path

### Properties
- scheme - `string` (`writeable`)
- host - `string` (`writeable`)
- username - `string` (`writeable`)
- password - `string` (`writeable`)
- port - `number` (`writeable`)
- path - `string` (`writeable`)
- params - `string` (`writeable`)
- fragment - `string` (`writeable`)

---
### Methods
#### `join`
Tries to add `url` to `self` (such `self` is the base for `url`), returns it as a new Url instance if it successeds or `nil` otherwise  
**Arguments:** 
- self - `Url`
- url - `Url`

**Returns:** `Url` - the combained url
##### Example
```lua
url = Url("https://example.com/a/b/c")
print(url:)
```
---
#### `get_relative`
Returns the relative path of `self` relative to `url` as a `string`

**Arguments:** 
- self - `Url`
- url - `Url`

**Returns:** `string`
##### Example
```lua
url1 = Url("https://example.com/a/b/c")
url2 = Url("https://example.com/a/b/c/index.html")
print(url1:get_relative(url2))
```
---
#### `segments`
Returns the `Url` path parts as table

**Arguments:** 
- self - `Url`

**Returns:** `table` - array of the `Url` path segments
##### Example
```lua
url = Url("https://example.com/a/b/c")
for _, seg in pairs(url:segments()) do 
    print(seg) -- a b c
end
```
---
#### `get_params`
Returns the `Url` params as table

**Arguments:** 
- self - `Url`

**Returns:** `table` - the `Url` params as table
##### Example
```lua
url = Url("https://example.com/route?one=1&two=2")
for k, v in pairs(url:get_params()) do 
    print(k .. " = " .. v) -- one = 1 two = 2
end
```
