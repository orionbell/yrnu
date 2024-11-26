use super::LuaSetup;
use crate::core::*;
use mlua::{FromLua, Lua, LuaNativeFn, MetaMethod, Result, UserData, UserDataMethods, Value};

impl FromLua for IpVersion {
    fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
        if let Some(value) = value.as_userdata() {
            let value = value.borrow::<IpVersion>()?;
            return Ok(value.clone());
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
            let value = value.borrow::<IpAddress>()?;
            return Ok(value.clone());
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
        fields.add_field_method_get("address", |_, this| Ok(this.address()));
        fields.add_field_method_get("version", |_, this| Ok(this.version()));
        fields.add_field_method_get("kind", |_, this| Ok(this.kind()));
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_octats", |_, this, ()| match this.get_octats() {
            Ok(kind) => Ok(kind),
            Err(_) => Err(mlua::Error::BadArgument {
                to: None,
                pos: 1,
                name: None,
                cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                    "Invalid Argument".to_string(),
                )),
            }),
        });
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
                        to: None,
                        pos: 1,
                        name: None,
                        cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                            "Invalid Argument".to_string(),
                        )),
                    }),
                },
            )?;
        let ipaddress_table = lua.create_table()?;
        ipaddress_table.set("new", constructor)?;
        ipaddress_table.set(
            "is_valid",
            lua.create_function(|_, address: String| Ok(IpAddress::is_valid(address.as_str())))?,
        )?;
        ipaddress_table.set(
            "expend",
            lua.create_function(
                |_, address: String| match IpAddress::expend(address.as_str()) {
                    Ok(addr) => Ok(addr),
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
        );
        ipaddress_table.set(
            "shorten",
            lua.create_function(
                |_, address: String| match IpAddress::shorten(address.as_str()) {
                    Ok(addr) => Ok(addr),
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
            let value = value.borrow::<Mask>()?;
            return Ok(value.clone());
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
        fields.add_field_method_get("mask", |_, this| Ok(this.mask()));
        fields.add_field_method_get("prefix", |_, this| Ok(this.prefix()));
        fields.add_field_method_get("num_of_hosts", |_, this| Ok(this.num_of_hosts()));
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
                    to: None,
                    pos: 1,
                    name: None,
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
                    to: None,
                    pos: 1,
                    name: None,
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                        "Invalid Argument".to_string(),
                    )),
                }),
            })?,
        )?;
        mask_table.set(
            "get_prefix",
            lua.create_function(|_, mask: String| match Mask::get_prefix(mask.as_str()) {
                Ok(prefix) => Ok(prefix),
                Err(_) => Err(mlua::Error::BadArgument {
                    to: None,
                    pos: 1,
                    name: None,
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                        "Invalid Argument".to_string(),
                    )),
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
            let value = value.borrow::<Network>()?;
            return Ok(value.clone());
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
        fields.add_field_method_get("broadcast", |_, this| Ok(this.broadcast()));
        fields.add_field_method_get("id", |_, this| Ok(this.netid()));
        fields.add_field_method_get("mask", |_, this| Ok(this.mask().clone()));
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{}", this)));
        methods.add_method("containes", |_, this, addr: IpAddress| {
            Ok(this.containes(&addr))
        });
        methods.add_method(
            "containes_str",
            |_, this, address: String| match IpAddress::new(&address) {
                Ok(address) => Ok(this.containes(&address)),
                Err(_) => Err(mlua::Error::BadArgument {
                    to: None,
                    pos: 1,
                    name: None,
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                        "Invalid Argument".to_string(),
                    )),
                }),
            },
        );
    }
}
impl LuaSetup for Network {
    fn setup(lua: &mlua::Lua) -> Result<()> {
        let constructor = lua.create_function(|_, (netid, mask): (IpAddress, Mask)| {
            match Network::new(netid, mask) {
                Ok(net) => Ok(net),
                Err(_) => Err(mlua::Error::BadArgument {
                    to: None,
                    pos: 1,
                    name: None,
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                        "Invalid Argument".to_string(),
                    )),
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
                    to: None,
                    pos: 1,
                    name: None,
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                        "Invalid Argument".to_string(),
                    )),
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
        methods.add_method("as_array", |_, this, ()| Ok(this.as_vector()));
        methods.add_method("as_bytes", |_, this, ()| Ok(this.as_bytes()));
        methods.add_meta_method(MetaMethod::Eq, |_, this, other: MacAddress| {
            Ok(this == &other)
        });
        methods.add_meta_method(MetaMethod::Lt, |_, this, other: MacAddress| {
            Ok(this < &other)
        });
        methods.add_meta_method(MetaMethod::Le, |_, this, other: MacAddress| {
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
                    to: None,
                    pos: 1,
                    name: None,
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                        "Invalid Argument".to_string(),
                    )),
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
            let value = value.borrow::<Interface>()?;
            return Ok(value.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: "table",
            to: "MacAddress".to_string(),
            message: Some("convertion failed".to_string()),
        })
    }
}
impl UserData for Interface {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name()));
        fields.add_field_method_get("index", |_, this| Ok(this.index()));
        fields.add_field_method_get("description", |_, this| Ok(this.description()));
        fields.add_field_method_get("mac", |_, this| Ok(this.mac()));
        fields.add_field_method_get("ipv4", |_, this| Ok(this.ipv4()));
        fields.add_field_method_get("ipv6", |_, this| Ok(this.ipv6()));
        fields.add_field_method_get("mask", |_, this| Ok(this.mask()));
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
                    to: None,
                    pos: 1,
                    name: None,
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                        "Invalid Argument".to_string(),
                    )),
                }),
            })?,
        )?;
        interface_table.set(
            "by_name",
            lua.create_function(|_, name: String| match Interface::by_name(&name) {
                Ok(inf) => Ok(inf),
                Err(_) => Err(mlua::Error::BadArgument {
                    to: None,
                    pos: 1,
                    name: None,
                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                        "Invalid Argument".to_string(),
                    )),
                }),
            })?,
        )?;
        interface_table.set("all", lua.create_function(|_, ()| Ok(Interface::all()))?)?;
        let _ = lua.globals().set("Interface", interface_table);
        Ok(())
    }
}
