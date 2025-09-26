local rtr = netdev_config.router()
io.write("hostname: ")
local hostname = io.read()
if #hostname ~= 0 then
    rtr:set_hostname(hostname)
end
io.write("secret: ")
local secret = io.read()
if #secret ~= 0 then
    rtr:set_secret(secret)
end
io.write("password: ")
local password = io.read()
if #password ~= 0 then
    rtr:set_password(password)
end
io.write("enable password encryption ?(y/n): ")
local passwd_enc = io.read()
if #passwd_enc ~= 0 then
    if string.match("yes", passwd_enc) then
        rtr:set_password_enc(true)
    elseif string.match("no", passwd_enc) then
        rtr:set_password_enc(false)
    end
end
io.write("enable ipv6 ?(y/n): ")
local ipv6 = io.read()
if #ipv6 ~= 0 then
    if string.match("yes", ipv6) then
        rtr:set_enable_ipv6(true)
    elseif string.match("no", ipv6) then
        rtr:set_enable_ipv6(false)
    end
end
io.write("add ospf ? (y/n): ")
local is_ospf = io.read()
local is_ospf = string.match("yes", is_ospf) and is_ospf ~= ""
if is_ospf then
    io.write("ospf pid: ")
    local pid = io.read("*n")
    rtr:add_ospf({ pid = pid })
    io.write("router id: ")
    io.read() -- Getting rid of the \n from the previous input
    local router_id = io.read()
    if IpVersion.is_v4(router_id) then
        local id = IpAddress(router_id)
        rtr.ospf:set_router_id(id)
    end
    io.write("add network: ")
    local net = io.read()
    local nets = {}
    while net ~= "" do
        table.insert(nets, net)
        io.write("add network: ")
        net = io.read()
    end
    if #nets ~= 0 then
        rtr.ospf:set_networks(nets)
    end
end
print(rtr)
