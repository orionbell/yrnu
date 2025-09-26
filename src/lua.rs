pub mod core_lua;
pub mod interpreter;
use crate::core::*;
use crate::port;
use mlua::{Lua, Result, StdLib};
trait LuaSetup {
    fn setup(lua: &mlua::Lua) -> Result<()>;
}
pub fn ports_setup(lua: &mlua::Lua) -> Result<()> {
    let ports_table = lua.create_table()?;
    ports_table.set("ftp", port::FTP)?;
    ports_table.set("ftp_data", port::FTP_DATA)?;
    ports_table.set("ssh", port::SSH)?;
    ports_table.set("telnet", port::TELNET)?;
    ports_table.set("smtp", port::SMTP)?;
    ports_table.set("whois", port::WHOIS)?;
    ports_table.set("tacacs", port::TACACS)?;
    ports_table.set("dns", port::DNS)?;
    ports_table.set("tftp", port::TFTP)?;
    ports_table.set("http", port::HTTP)?;
    ports_table.set("pop3", port::POP3)?;
    ports_table.set("ntp", port::NTP)?;
    ports_table.set("imap", port::IMAP)?;
    ports_table.set("bgp", port::BGP)?;
    ports_table.set("https", port::HTTPS)?;
    ports_table.set("isakmp", port::ISAKMP)?;
    ports_table.set("syslog", port::SYSLOG)?;
    ports_table.set("rip", port::RIP)?;
    ports_table.set("ftps", port::FTPS)?;
    ports_table.set("ftps_data", port::FTPS_DATA)?;
    lua.globals().set("port", ports_table)?;
    Ok(())
}
pub fn init() -> Result<Lua> {
    let lua = Lua::new();
    lua.load_std_libs(StdLib::ALL_SAFE)?;
    _ = ports_setup(&lua);
    _ = IpVersion::setup(&lua);
    _ = IpKind::setup(&lua);
    _ = IpAddress::setup(&lua);
    _ = Mask::setup(&lua);
    _ = Network::setup(&lua);
    _ = MacAddress::setup(&lua);
    _ = Interface::setup(&lua);
    _ = Path::setup(&lua);
    _ = Url::setup(&lua);
    Ok(lua)
}
pub fn run(lua: &Lua, code: &str) -> Result<mlua::Value> {
    lua.load(code).eval::<mlua::Value>()
}
