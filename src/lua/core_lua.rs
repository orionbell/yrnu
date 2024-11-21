use super::LuaSetup;
use crate::core::*;
use mlua::{FromLua, Lua, MetaMethod, Result, UserData, UserDataMethods, Value};

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
            lua.create_function(|_, address: String| {
                Ok(IpKind::get_kind(address.as_str()).unwrap())
            })?,
        )?;
        let _ = lua.globals().set("IpKind", kinds_table);
        Ok(())
    }
}


