pub mod core_lua;

use mlua::{FromLua, IntoLua, IntoLuaMulti, Lua, MetaMethod, Result, StdLib, UserData, UserDataMethods, Value};
use crate::core::{self, *};

trait LuaSetup {
    fn setup(lua: &mlua::Lua) -> Result<()>;
}

pub fn init() -> Result<Lua> {
    let lua = Lua::new();
    lua.load_std_libs(StdLib::ALL_SAFE)?;
    let _ = IpVersion::setup(&lua);
    let _ = IpKind::setup(&lua);
    Ok(lua)
}

pub fn run(lua: Lua, code: String) -> Result<()> {
    lua.load(code).exec()?;
    Ok(())
}
