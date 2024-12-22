use super::LuaSetup;
use crate::core::*;
use mlua::{FromLua, Lua, MetaMethod, Result, UserData, UserDataMethods, Value};

impl FromLua for IpVersion {
    fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
        if let Some(value) = value.as_userdata() {
            let value = value.take::<IpVersion>()?;
            return Ok(value);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: "table",
            to: "IpVersion".to_string(),
            message: Some("convertion failed".to_string()),
        })
    }
}
impl UserData for IpVersion {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(format!("{}", this).to_string())
        });
    }
}
impl LuaSetup for IpVersion {
    fn setup(lua: &mlua::Lua) -> Result<()> {
        let versions_table = lua.create_table()?;
        versions_table.set("V4", IpVersion::V4)?;
        versions_table.set("V6", IpVersion::V6)?;
        versions_table.set(
            "is_v4",
            lua.create_function(|_, address: String| Ok(IpVersion::is_v4(address.as_str())))?,
        )?;
        versions_table.set(
            "is_v6",
            lua.create_function(|_, address: String| Ok(IpVersion::is_v6(address.as_str())))?,
        )?;
        let _ = lua.globals().set("IpVersion", versions_table);
        Ok(())
    }
}

impl FromLua for IpKind {
    fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
        if let Some(value) = value.as_userdata() {
            let value = value.borrow::<IpKind>()?;
            return Ok(value.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: "table",
            to: "IpKind".to_string(),
            message: Some("convertion failed".to_string()),
        })
    }
}
impl UserData for IpKind {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(format!("{}", this).to_string())
        });
    }
}
impl LuaSetup for IpKind {
    fn setup(lua: &mlua::Lua) -> Result<()> {
        let kinds_table = lua.create_table()?;
        kinds_table.set("Public", IpKind::Public)?;
        kinds_table.set("Private", IpKind::Private)?;
        kinds_table.set("Loopback", IpKind::Loopback)?;
        kinds_table.set("Linklocal", IpKind::Linklocal)?;
        kinds_table.set("Apipa", IpKind::Apipa)?;
        kinds_table.set("Uniqelocal", IpKind::Uniqelocal)?;
        kinds_table.set("Uniqeglobal", IpKind::Uniqeglobal)?;
        kinds_table.set("Broadcast", IpKind::Broadcast)?;
        kinds_table.set("Netid", IpKind::Netid)?;
        kinds_table.set("Multicast", IpKind::Multicast)?;
        kinds_table.set("Unspecified", IpKind::Unspecified)?;
        kinds_table.set(
            "is_public",
            lua.create_function(|_, address: String| Ok(IpKind::is_public(address.as_str())))?,
        )?;
        kinds_table.set(
            "is_private",
            lua.create_function(|_, address: String| Ok(IpKind::is_private(address.as_str())))?,
        )?;
        kinds_table.set(
            "is_loopback",
            lua.create_function(|_, address: String| Ok(IpKind::is_loopback(address.as_str())))?,
        )?;
        kinds_table.set(
            "is_linklocal",
            lua.create_function(|_, address: String| Ok(IpKind::is_linklocal(address.as_str())))?,
        )?;
        kinds_table.set(
            "is_apipa",
            lua.create_function(|_, address: String| Ok(IpKind::is_apipa(address.as_str())))?,
        )?;
        kinds_table.set(
            "is_multicast",
            lua.create_function(|_, address: String| Ok(IpKind::is_multicast(address.as_str())))?,
        )?;
        kinds_table.set(
            "is_unspecified",
            lua.create_function(|_, address: String| Ok(IpKind::is_unspecified(address.as_str())))?,
        )?;
        kinds_table.set(
            "get_kind",
            lua.create_function(
                |_, address: String| match IpKind::get_kind(address.as_str()) {
                    Ok(kind) => Ok(kind),
                    Err(_) => Err(mlua::Error::BadArgument {
                        to: None,
                        pos: 1,
                        name: None,
                        cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                            "Invalid Argument".to_string(),
                        )),
                    }),
                },
            )?,
        )?;
        kinds_table.set(
            "is_broadcast",
            lua.create_function(|_, (address, mask): (String, Mask)| {
                Ok(IpKind::is_broadcast(address.as_str(), &mask))
            })?,
        )?;
        kinds_table.set(
            "is_netid",
            lua.create_function(|_, (address, mask): (String, Mask)| {
                Ok(IpKind::is_netid(address.as_str(), &mask))
            })?,
        )?;
        kinds_table.set(
            "get_broadcast",
            lua.create_function(|_, (id, mask): (String, Mask)| {
                match IpKind::get_broadcast(id.as_str(), &mask) {
                    Ok(broadcast) => Ok(broadcast),
                    Err(_) => Err(mlua::Error::BadArgument {
                        to: None,
                        pos: 1,
                        name: None,
                        cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                            "Invalid Argument".to_string(),
                        )),
                    }),
                }
            })?,
        )?;
        let _ = lua.globals().set("IpKind", kinds_table);
        Ok(())
    }
}

impl FromLua for IpAddress {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        if let Some(value) = value.as_userdata() {
            let value = value.take::<IpAddress>()?;
            return Ok(value);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: "table",
            to: "IpAddress".to_string(),
            message: Some("convertion failed".to_string()),
        })
    }
}
impl UserData for IpAddress {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("address", |_, this| Ok(this.address().to_owned()));
        fields.add_field_method_get("version", |_, this| Ok(this.version().to_owned()));
        fields.add_field_method_get("kind", |_, this| Ok(this.kind().to_owned()));
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("octets", |_, this, ()| Ok(this.octets().clone()));
        methods.add_method("get_expended", |_, this, ()| Ok(this.get_expended()));
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{}", this)));
    }
}
impl LuaSetup for IpAddress {
    fn setup(lua: &mlua::Lua) -> Result<()> {
        let constructor =
            lua.create_function(
                |_, address: String| match IpAddress::new(address.as_str()) {
                    Ok(addr) => Ok(addr),
                    Err(_) => Err(mlua::Error::BadArgument {
                        to: Some("IpAddress.new".to_string()),
                        pos: 1,
                        name: Some("address".to_string()),
                        cause: std::sync::Arc::new(mlua::Error::RuntimeError(format!(
                            "Invalid Address: {}",
                            address
                        ))),
                    }),
                },
            )?;
        let ipaddress_table = lua.create_table()?;
        ipaddress_table.set("new", constructor)?;
        ipaddress_table.set(
            "is_valid",
            lua.create_function(|_, address: String| Ok(IpAddress::is_valid(address.as_str())))?,
        )?;
        _ = ipaddress_table.set(
            "expend",
            lua.create_function(
                |_, address: String| match IpAddress::expend(address.as_str()) {
                    Ok(addr) => Ok(addr),
                    Err(_) => Err(mlua::Error::BadArgument {
                        to: Some("IpAddress.expend".to_string()),
                        pos: 1,
                        name: Some("address".to_string()),
                        cause: std::sync::Arc::new(mlua::Error::RuntimeError(format!(
                            "Invalid Address: {}",
                            address
                        ))),
                    }),
                },
            )?,
        );
        _ = ipaddress_table.set(
            "shorten",
            lua.create_function(
                |_, address: String| match IpAddress::shorten(address.as_str()) {
                    Ok(addr) => Ok(addr),
                    Err(_) => Err(mlua::Error::BadArgument {
                        to: Some("IpAddress.shorten".to_string()),
                        pos: 1,
                        name: Some("address".to_string()),
                        cause: std::sync::Arc::new(mlua::Error::RuntimeError(format!(
                            "Invalid Address: {}",
                            address
                        ))),
                    }),
                },
            )?,
        );
        ipaddress_table.set(
            "eui64",
            lua.create_function(|_, address: MacAddress| Ok(IpAddress::eui64(&address)))?,
        )?;
        let _ = lua.globals().set("IpAddress", ipaddress_table);
        Ok(())
    }
}

impl FromLua for Mask {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        if let Some(value) = value.as_userdata() {
            let value = value.take::<Mask>()?;
            return Ok(value);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: "table",
            to: "Mask".to_string(),
            message: Some("convertion failed".to_string()),
        })
    }
}
impl UserData for Mask {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("mask", |_, this| Ok(this.mask().to_owned()));
        fields.add_field_method_get("prefix", |_, this| Ok(this.prefix().to_owned()));
        fields.add_field_method_get("num_of_hosts", |_, this| Ok(this.num_of_hosts().to_owned()));
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{}", this)));
    }
}
impl LuaSetup for Mask {
    fn setup(lua: &mlua::Lua) -> Result<()> {
        let constructor =
            lua.create_function(|_, mask: String| match Mask::new(mask.as_str()) {
                Ok(mask) => Ok(mask),
                Err(_) => Err(mlua::Error::BadArgument {
                    to: Some("Mask.new".to_string()),
                    pos: 1,
                    name: Some("mask".to_string()),
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                        "Invalid Argument".to_string(),
                    )),
                }),
            })?;
        let mask_table = lua.create_table()?;
        mask_table.set("new", constructor)?;
        mask_table.set(
            "is_valid",
            lua.create_function(|_, mask: String| Ok(Mask::is_valid(mask.as_str())))?,
        )?;
        mask_table.set(
            "from_prefix",
            lua.create_function(|_, prefix: u8| match Mask::from_prefix(prefix) {
                Ok(mask) => Ok(mask),
                Err(_) => Err(mlua::Error::BadArgument {
                    to: Some("Mask.from_prefix".to_string()),
                    pos: 1,
                    name: Some("prefix".to_string()),
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(format!(
                        "Invalid mask prefix: {}",
                        prefix
                    ))),
                }),
            })?,
        )?;
        mask_table.set(
            "get_prefix",
            lua.create_function(|_, mask: String| match Mask::get_prefix(mask.as_str()) {
                Ok(prefix) => Ok(prefix),
                Err(_) => Err(mlua::Error::BadArgument {
                    to: Some("Mask.get_prefix".to_string()),
                    pos: 1,
                    name: Some("mask".to_string()),
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(format!(
                        "Invalid mask: {}",
                        mask
                    ))),
                }),
            })?,
        )?;
        let _ = lua.globals().set("Mask", mask_table);
        Ok(())
    }
}

impl FromLua for Network {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        if let Some(value) = value.as_userdata() {
            let value = value.take::<Network>()?;
            return Ok(value);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: "table",
            to: "Network".to_string(),
            message: Some("convertion failed".to_string()),
        })
    }
}
impl UserData for Network {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("broadcast", |_, this| Ok(this.broadcast().to_owned()));
        fields.add_field_method_get("id", |_, this| Ok(this.netid().to_owned()));
        fields.add_field_method_get("mask", |_, this| Ok(this.mask().to_owned()));
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{}", this)));
        methods.add_method("contains", |_, this, addr: IpAddress| {
            Ok(this.contains(&addr))
        });
        methods.add_method(
            "contains_str",
            |_, this, address: String| match IpAddress::new(&address) {
                Ok(address) => Ok(this.contains(&address)),
                Err(_) => Err(mlua::Error::BadArgument {
                    to: Some("Network.containes".to_string()),
                    pos: 2,
                    name: Some("address".to_string()),
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(format!(
                        "Invalid address: {}",
                        address
                    ))),
                }),
            },
        );
    }
}
impl LuaSetup for Network {
    fn setup(lua: &mlua::Lua) -> Result<()> {
        let constructor = lua.create_function(|_, (netid, mask): (IpAddress, Mask)| {
            match Network::new(netid.clone(), mask.clone()) {
                Ok(net) => Ok(net),
                Err(_) => Err(mlua::Error::BadArgument {
                    to: Some("Network.new".to_string()),
                    pos: if IpKind::is_netid(&netid.address(), &mask) {
                        1
                    } else {
                        2
                    },
                    name: if IpKind::is_netid(&netid.address(), &mask) {
                        Some("netid".to_string())
                    } else {
                        Some("mask".to_string())
                    },
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(format!(
                        "Invalid {}: {}",
                        if IpKind::is_netid(&netid.address(), &mask) {
                            "net id"
                        } else {
                            "mask"
                        },
                        if IpKind::is_netid(&netid.address(), &mask) {
                            netid.address().to_owned()
                        } else {
                            format!("{} for net id {}", mask.mask(), &netid.address())
                        }
                    ))),
                }),
            }
        })?;
        let network_table = lua.create_table()?;
        network_table.set("new", constructor)?;
        network_table.set(
            "from",
            lua.create_function(|_, net: String| match Network::from_str(net.as_str()) {
                Ok(net) => Ok(net),
                Err(_) => Err(mlua::Error::BadArgument {
                    to: Some("from".to_string()),
                    pos: 1,
                    name: Some("net".to_string()),
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(format!(
                        "Invalid Network string {} use the \"netid/prefix\" format",
                        net
                    ))),
                }),
            })?,
        )?;
        let _ = lua.globals().set("Network", network_table);
        Ok(())
    }
}

impl FromLua for MacAddress {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        if let Some(value) = value.as_userdata() {
            let value = value.borrow::<MacAddress>()?;
            return Ok(value.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: "table",
            to: "MacAddress".to_string(),
            message: Some("convertion failed".to_string()),
        })
    }
}
impl UserData for MacAddress {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("address", |_, this| Ok(this.address()));
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{}", this)));
        methods.add_method("as_bytes", |_, this, ()| Ok(this.as_bytes().to_owned()));
        methods.add_meta_method(MetaMethod::Eq, |_, this, other: MacAddress| {
            Ok(this == &other)
        });
        methods.add_meta_method(MetaMethod::Lt, |_, this, other: MacAddress| {
            println!("{}", this < &other);
            Ok(this < &other)
        });
        methods.add_meta_method(MetaMethod::Le, |_, this, other: MacAddress| {
            println!("{}", this <= &other);
            Ok(this <= &other)
        });
    }
}
impl LuaSetup for MacAddress {
    fn setup(lua: &mlua::Lua) -> Result<()> {
        let constructor =
            lua.create_function(|_, address: String| match MacAddress::new(&address) {
                Ok(address) => Ok(address),
                Err(_) => Err(mlua::Error::BadArgument {
                    to: Some("MacAddress.new".to_string()),
                    pos: 1,
                    name: Some("address".to_string()),
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(format!(
                        "Invalid mac address: {}",
                        address
                    ))),
                }),
            })?;
        let macaddress_table = lua.create_table()?;
        macaddress_table.set("new", constructor)?;
        macaddress_table.set(
            "is_valid",
            lua.create_function(|_, address: String| Ok(MacAddress::is_valid(&address)))?,
        )?;
        let _ = lua.globals().set("MacAddress", macaddress_table);
        Ok(())
    }
}

impl FromLua for Interface {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        if let Some(value) = value.as_userdata() {
            let value = value.take::<Interface>()?;
            return Ok(value);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: "table",
            to: "Interface".to_string(),
            message: Some("convertion failed".to_string()),
        })
    }
}
impl UserData for Interface {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name().to_owned()));
        fields.add_field_method_get("index", |_, this| Ok(this.index().to_owned()));
        fields.add_field_method_get("description", |_, this| Ok(this.description().to_owned()));
        fields.add_field_method_get("mac", |_, this| Ok(this.mac().to_owned()));
        fields.add_field_method_get("ipv4", |_, this| Ok(this.ipv4().to_owned()));
        fields.add_field_method_get("ipv6", |_, this| Ok(this.ipv6().to_owned()));
        fields.add_field_method_get("mask", |_, this| Ok(this.mask().to_owned()));
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{}", this)));
    }
}
impl LuaSetup for Interface {
    fn setup(lua: &mlua::Lua) -> Result<()> {
        let interface_table = lua.create_table()?;
        interface_table.set(
            "by_index",
            lua.create_function(|_, index: u32| match Interface::by_index(index) {
                Ok(inf) => Ok(inf),
                Err(_) => Err(mlua::Error::BadArgument {
                    to: Some("Interface.by_index".to_string()),
                    pos: 1,
                    name: Some("index".to_string()),
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(format!(
                        "Invalid interface index: {}",
                        index
                    ))),
                }),
            })?,
        )?;
        interface_table.set(
            "by_name",
            lua.create_function(|_, name: String| match Interface::by_name(&name) {
                Ok(inf) => Ok(inf),
                Err(_) => Err(mlua::Error::BadArgument {
                    to: Some("Interface.by_name".to_string()),
                    pos: 1,
                    name: Some("name".to_string()),
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(format!(
                        "Invalid interface name: {}",
                        name
                    ))),
                }),
            })?,
        )?;
        interface_table.set("all", lua.create_function(|_, ()| Ok(Interface::all()))?)?;
        let _ = lua.globals().set("Interface", interface_table);
        Ok(())
    }
}
