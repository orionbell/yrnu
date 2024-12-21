pub mod core_lua;
pub mod interpreter;
use crate::core::*;
use mlua::{Lua, Result, StdLib};

trait LuaSetup {
    fn setup(lua: &mlua::Lua) -> Result<()>;
}

pub fn init() -> Result<Lua> {
    let lua = Lua::new();
    lua.load_std_libs(StdLib::ALL_SAFE)?;
    let _ = IpVersion::setup(&lua);
    let _ = IpKind::setup(&lua);
    let _ = IpAddress::setup(&lua);
    let _ = Mask::setup(&lua);
    let _ = Network::setup(&lua);
    let _ = MacAddress::setup(&lua);
    let _ = Interface::setup(&lua);
    Ok(lua)
}

pub fn run(lua: &Lua, code: &str) -> Result<()> {
    lua.load(code).exec()?;
    Ok(())
}
