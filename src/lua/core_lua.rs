use super::LuaSetup;
use crate::core::*;
use mlua::{MetaMethod, Result, UserData, UserDataMethods};
use std::io::{BufRead, Read};
use std::path::PathBuf;
use std::str::FromStr;

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
        kinds_table.set("public", IpKind::Public)?;
        kinds_table.set("private", IpKind::Private)?;
        kinds_table.set("loopback", IpKind::Loopback)?;
        kinds_table.set("linklocal", IpKind::Linklocal)?;
        kinds_table.set("apipa", IpKind::Apipa)?;
        kinds_table.set("uniqelocal", IpKind::Uniqelocal)?;
        kinds_table.set("uniqeglobal", IpKind::Uniqeglobal)?;
        kinds_table.set("broadcast", IpKind::Broadcast)?;
        kinds_table.set("netid", IpKind::Netid)?;
        kinds_table.set("multicast", IpKind::Multicast)?;
        kinds_table.set("unspecified", IpKind::Unspecified)?;
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
                    Ok(kind) => Ok(Some(kind)),
                    Err(_) => Ok(None),
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
                    Ok(broadcast) => Ok(Some(broadcast)),
                    Err(_) => Ok(None),
                }
            })?,
        )?;
        let _ = lua.globals().set("IpKind", kinds_table);
        Ok(())
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
        methods.add_method("get_expended", |_, this, ()| match this.get_expended() {
            Ok(addr) => Ok(Some(addr)),
            Err(_) => Ok(None),
        });
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{}", this)));
    }
}
impl LuaSetup for IpAddress {
    fn setup(lua: &mlua::Lua) -> Result<()> {
        let constructor =
            lua.create_function(
                |_, (_, address): (mlua::Value, mlua::Value)| match address {
                    mlua::Value::Table(table) => {
                        match IpAddress::new(
                            &table
                                .sequence_values::<u8>()
                                .filter(|n| n.is_ok())
                                .map(|n| n.unwrap())
                                .collect::<Vec<u8>>(),
                        ) {
                            Ok(mask) => Ok(Some(mask)),
                            Err(_) => Ok(None),
                        }
                    }
                    mlua::Value::String(address) => {
                        match IpAddress::from_str(&address.to_string_lossy()) {
                            Ok(addr) => Ok(Some(addr)),
                            Err(_) => Ok(None),
                        }
                    }
                    _ => Ok(None),
                },
            )?;
        let ipaddress_table = lua.create_table()?;
        let metatable = lua.create_table()?;
        metatable.set("__call", constructor)?;
        ipaddress_table.set_metatable(Some(metatable));
        ipaddress_table.set(
            "is_valid",
            lua.create_function(|_, address: String| Ok(IpAddress::is_valid(address.as_str())))?,
        )?;
        _ = ipaddress_table.set(
            "expend",
            lua.create_function(
                |_, address: String| match IpAddress::expend(address.as_str()) {
                    Ok(addr) => Ok(Some(addr)),
                    Err(_) => Ok(None),
                },
            )?,
        );
        _ = ipaddress_table.set(
            "shorten",
            lua.create_function(
                |_, address: String| match IpAddress::shorten(address.as_str()) {
                    Ok(addr) => Ok(Some(addr)),
                    Err(_) => Ok(None),
                },
            )?,
        );
        ipaddress_table.set(
            "eui64",
            lua.create_function(|_, address: MacAddress| Ok(IpAddress::eui64(&address)))?,
        )?;
        ipaddress_table.set(
            "from_domain",
            lua.create_function(|_, domain: String| Ok(IpAddress::from_domain(&domain)))?,
        )?;
        let _ = lua.globals().set("IpAddress", ipaddress_table);
        Ok(())
    }
}

impl UserData for Mask {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("mask", |_, this| Ok(this.mask().to_owned()));
        fields.add_field_method_get("prefix", |_, this| Ok(this.prefix().to_owned()));
        fields.add_field_method_get("num_of_hosts", |_, this| Ok(this.num_of_hosts().to_owned()));
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("wildcard", |_, this, ()| Ok(this.wildcard()));
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{}", this)));
    }
}
impl LuaSetup for Mask {
    fn setup(lua: &mlua::Lua) -> Result<()> {
        let constructor =
            lua.create_function(|_, (_, mask): (mlua::Value, mlua::Value)| match mask {
                mlua::Value::Table(table) => {
                    match Mask::new(
                        &table
                            .sequence_values::<u8>()
                            .filter(|n| n.is_ok())
                            .map(|n| n.unwrap())
                            .collect::<Vec<u8>>(),
                    ) {
                        Ok(mask) => Ok(Some(mask)),
                        Err(_) => Ok(None),
                    }
                }
                mlua::Value::String(mask) => match Mask::from_str(&mask.to_string_lossy()) {
                    Ok(mask) => Ok(Some(mask)),
                    Err(_) => Ok(None),
                },
                _ => Ok(None),
            })?;
        let mask_table = lua.create_table()?;
        let metatable = lua.create_table()?;
        metatable.set("__call", constructor)?;
        mask_table.set_metatable(Some(metatable));
        mask_table.set(
            "is_valid",
            lua.create_function(|_, mask: String| Ok(Mask::is_valid(mask.as_str())))?,
        )?;
        mask_table.set(
            "from_prefix",
            lua.create_function(|_, prefix: u8| match Mask::from_prefix(prefix) {
                Ok(mask) => Ok(Some(mask)),
                Err(_) => Ok(None),
            })?,
        )?;
        mask_table.set(
            "get_prefix",
            lua.create_function(|_, mask: mlua::Value| match mask {
                mlua::Value::Table(table) => Ok(Some(Mask::get_prefix(
                    &table
                        .sequence_values::<u8>()
                        .filter(|n| n.is_ok())
                        .map(|n| n.unwrap())
                        .collect::<Vec<u8>>(),
                ))),
                mlua::Value::String(mask) => {
                    let octets = IpAddress::octets_from_str(&mask.to_string_lossy());
                    if let Err(_) = octets {
                        return Ok(None);
                    }
                    Ok(Some(Mask::get_prefix(&octets.unwrap())))
                }
                _ => Ok(None),
            })?,
        )?;
        let _ = lua.globals().set("Mask", mask_table);
        Ok(())
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
            |_, this, address: String| match IpAddress::from_str(&address) {
                Ok(address) => Ok(this.contains(&address)),
                Err(_) => Ok(false),
            },
        );
    }
}
impl LuaSetup for Network {
    fn setup(lua: &mlua::Lua) -> Result<()> {
        let constructor =
            lua.create_function(|_, (_, netid, mask): (mlua::Value, IpAddress, Mask)| {
                match Network::new(netid.clone(), mask.clone()) {
                    Ok(net) => Ok(Some(net)),
                    Err(_) => Ok(None),
                }
            })?;
        let network_table = lua.create_table()?;
        let metatable = lua.create_table()?;
        metatable.set("__call", constructor)?;
        network_table.set_metatable(Some(metatable));
        network_table.set(
            "from",
            lua.create_function(|_, net: String| match Network::from_str(net.as_str()) {
                Ok(net) => Ok(Some(net)),
                Err(_) => Ok(None),
            })?,
        )?;
        let _ = lua.globals().set("Network", network_table);
        Ok(())
    }
}

impl UserData for MacAddress {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("address", |_, this| Ok(this.address()));
        fields.add_field_method_get("vendor", |_, this| Ok(this.vendor().clone()));
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{}", this)));
        methods.add_method("as_bytes", |_, this, ()| Ok(this.as_bytes().to_owned()));
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
            lua.create_function(
                |_, (_, address): (mlua::Value, String)| match MacAddress::from_str(&address) {
                    Ok(address) => Ok(Some(address)),
                    Err(_) => Ok(None),
                },
            )?;
        let macaddress_table = lua.create_table()?;
        let metatable_table = lua.create_table()?;
        metatable_table.set("__call", constructor)?;
        macaddress_table.set_metatable(Some(metatable_table));
        macaddress_table.set(
            "is_valid",
            lua.create_function(|_, address: String| Ok(MacAddress::is_valid(&address)))?,
        )?;
        let _ = lua.globals().set("MacAddress", macaddress_table);
        Ok(())
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
                Ok(inf) => Ok(Some(inf)),
                Err(_) => Ok(None),
            })?,
        )?;
        interface_table.set(
            "by_name",
            lua.create_function(|_, name: String| match Interface::by_name(&name) {
                Ok(inf) => Ok(Some(inf)),
                Err(_) => Ok(None),
            })?,
        )?;
        interface_table.set("all", lua.create_function(|_, ()| Ok(Interface::all()))?)?;
        let _ = lua.globals().set("Interface", interface_table);
        Ok(())
    }
}

impl UserData for Path {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("exists", |_, this| Ok(this.0.exists()));
        fields.add_field_method_get("is_dir", |_, this| Ok(this.0.is_dir()));
        fields.add_field_method_get("is_file", |_, this| Ok(this.0.is_file()));
        fields.add_field_method_get("is_relative", |_, this| Ok(this.0.is_relative()));
        fields.add_field_method_get("name", |_, this| {
            if let Some(name) = this.0.file_name() {
                Ok(Some(name.to_owned()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_set("name", |_, this, name: String| {
            this.0.set_file_name(name);
            Ok(())
        });
        fields.add_field_method_get("extension", |_, this| {
            if let Some(name) = this.0.extension() {
                Ok(Some(name.to_owned()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_set("extension", |_, this, ext: String| {
            this.0.set_extension(ext);
            Ok(())
        });
        fields.add_field_method_get("is_symlink", |_, this| Ok(this.0.is_symlink()));
        fields.add_field_method_get("parent", |_, this| {
            if let Some(parent) = this.0.parent() {
                Ok(Some(Path(parent.to_path_buf())))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_get("children", |_, this| {
            let mut paths: Vec<Path> = vec![];
            if let Ok(entries) = this.0.read_dir() {
                for entry in entries {
                    if let Ok(entry) = entry {
                        paths.push(Path(entry.path()))
                    }
                }
            }
            Ok(paths)
        });
        fields.add_field_method_get("content_lines", |_, this| {
            match std::fs::File::open(&this.0) {
                Ok(file) => {
                    let mut lines = vec![];
                    for line in std::io::BufReader::new(file).lines().filter_map(|l| l.ok()) {
                        lines.push(line);
                    }
                    Ok(lines)
                }
                Err(e) => Err(mlua::Error::external(e)),
            }
        });
        fields.add_field_method_get("content", |_, this| match std::fs::File::open(&this.0) {
            Ok(mut file) => {
                let mut content = String::new();
                match file.read_to_string(&mut content) {
                    Ok(_) => Ok(content),
                    Err(e) => Err(mlua::Error::external(e)),
                }
            }
            Err(e) => Err(mlua::Error::external(e)),
        });
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("push", |_, this, entry: String| Ok(this.0.push(entry)));
        methods.add_method("join", |_, this, entry: String| Ok(this.0.join(entry)));
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.to_string()));
    }
}
impl LuaSetup for Path {
    fn setup(lua: &mlua::Lua) -> Result<()> {
        let path_table = lua.create_table()?;
        let metatable_table = lua.create_table()?;
        let constructor = lua
            .create_function(|_, (_, path): (mlua::Value, String)| Ok(Path(PathBuf::from(path))))?;
        metatable_table.set("__call", constructor)?;
        path_table.set_metatable(Some(metatable_table));
        let _ = lua.globals().set("Path", path_table);
        Ok(())
    }
}

impl UserData for Url {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("scheme", |_, this| Ok(this.0.scheme().to_owned()));
        fields.add_field_method_set("scheme", |_, this, scheme: String| {
            if this.0.set_scheme(&scheme).is_err() {
                if this.0.cannot_be_a_base() {
                    eprintln!("Error: Invalid scheme value - {scheme} for this type of URL, doing nothing...");
                } else if scheme == "file" {
                    eprintln!("Error: Can't set this type of URL to a file URL, doing nothing...");
                } else {
                    eprintln!("Error: Invalid scheme value - {scheme}, doing nothing...");
                }
            }
            Ok(())
        });
        fields.add_field_method_get("host", |_, this| {
            if let Some(host) = this.0.host() {
                match host {
                    url::Host::Domain(domain) => Ok(Some(domain.to_string())),
                    url::Host::Ipv4(addr) => Ok(Some(addr.to_string())),
                    url::Host::Ipv6(addr) => Ok(Some(addr.to_string())),
                }
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_set("host", |_, this, host: Option<String>| {
            if let Some(host) = host {
                if this.0.set_host(Some(host.as_str())).is_err() {
                    if this.0.cannot_be_a_base() {
                        eprintln!("Error: Can't add host to this type of URL , doing nothing...");
                    } else {
                        eprintln!("Error: Invalid host value - {host}, doing nothing...");
                    }
                }
            } else {
                _ = this.0.set_host(None);
            }
            Ok(())
        });
        fields.add_field_method_get("username", |_, this| {
            let username = this.0.username();
            if username != "" {
                Ok(Some(username.to_string()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_set("username", |_, this, username: String| {
            if this.0.set_username(&username).is_err() {
                if this.0.cannot_be_a_base() {
                    eprintln!("Error: Can't add username to this type of URL, doing nothing...");
                } else {
                    eprintln!("Error: Can't add username to URL without host, doing nothing...");
                }
            }
            Ok(())
        });
        fields.add_field_method_get("password", |_, this| {
            if let Some(password) = this.0.password() {
                Ok(Some(password.to_string()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_set("password", |_, this, password: Option<String>| {
            if let Some(password) = password {
                if this.0.set_password(Some(password.as_str())).is_err() {
                    if this.0.cannot_be_a_base() {
                        eprintln!(
                            "Error: Can't add password to this type of URL, doing nothing..."
                        );
                    } else {
                        eprintln!(
                            "Error: Can't add password to URL without host, doing nothing..."
                        );
                    }
                }
            } else {
                _ = this.0.set_password(None);
            }
            Ok(())
        });
        fields.add_field_method_get("port", |_, this| Ok(this.0.port_or_known_default()));
        fields.add_field_method_set("port", |_, this, port: Option<u16>| {
            if this.0.set_port(port).is_err() {
                if this.0.cannot_be_a_base() {
                    eprintln!("Error: Can't set port to this type of URL, doing nothing...");
                } else if this.0.scheme() == "file" {
                    eprintln!("Error: Can't set port to file URL, doing nothing...");
                } else {
                    eprintln!("Error: Can't set port to URL without host, doing nothing...");
                }
            }
            Ok(())
        });
        fields.add_field_method_get("path", |_, this| Ok(this.0.path().to_owned()));
        fields.add_field_method_set("path", |_, this, path: String| {
            this.0.set_path(&path);
            Ok(())
        });
        fields.add_field_method_get("params", |_, this| {
            if let Some(params) = this.0.query() {
                Ok(Some(params.to_string()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_set("params", |_, this, params: Option<String>| {
            if let Some(params) = params {
                this.0.set_query(Some(&params));
            } else {
                this.0.set_query(None);
            }
            Ok(())
        });
        fields.add_field_method_get("fragment", |_, this| {
            if let Some(fragment) = this.0.fragment() {
                Ok(Some(fragment.to_string()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_set("fragment", |_, this, fragment: Option<String>| {
            if let Some(fragment) = fragment {
                this.0.set_fragment(Some(&fragment));
            } else {
                this.0.set_fragment(None);
            }
            Ok(())
        });
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.0.to_string()));
        methods.add_method_mut("join", |_, this, url: String| {
            if let Ok(joined_url) = this.0.join(&url) {
                Ok(Some(Url(joined_url)))
            } else {
                eprintln!("Error: Can't join {} to {}", url, this.0);
                Ok(None)
            }
        });
        methods.add_method_mut("get_relative", |_, this, url: Url| {
            if let Some(rel_url) = this.0.make_relative(&url.0) {
                Ok(Some(rel_url))
            } else {
                eprintln!("Error: Can't create relative from {} to {}", this.0, url);
                Ok(None)
            }
        });
        methods.add_method("segments", |_, this, ()| {
            if let Some(segments) = this.0.path_segments() {
                Ok(Some(
                    segments
                        .map(|s| s.to_owned())
                        .filter(|s| s != "")
                        .collect::<Vec<String>>(),
                ))
            } else {
                Ok(None)
            }
        });
        methods.add_method("get_params", |lua, this, ()| {
            let params_table = lua.create_table()?;
            for param in this.0.query_pairs() {
                params_table.set(param.0, param.1)?;
            }
            Ok(params_table)
        });
    }
}
impl LuaSetup for Url {
    fn setup(lua: &mlua::Lua) -> Result<()> {
        let url_table = lua.create_table()?;
        let metatable_table = lua.create_table()?;
        let constructor = lua.create_function(|_, (_, url_str): (mlua::Value, String)| {
            match url::Url::parse(&url_str) {
                Ok(url) => Ok(Some(Url(url))),
                Err(_) => Ok(None),
            }
        })?;
        metatable_table.set("__call", constructor)?;
        url_table.set_metatable(Some(metatable_table));
        let _ = lua.globals().set("Url", url_table);
        Ok(())
    }
}
