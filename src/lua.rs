pub mod core_lua;
pub mod interpreter;
use crate::config::{self, connect, SSHAuthType};
use crate::core::*;
use crate::parser::*;
use crate::port;
use mlua::{Lua, Result, StdLib};
use quick_xml::events::{BytesDecl, BytesText, Event};
use quick_xml::writer::Writer;
use quick_xml::Reader;
use regex::Regex;
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
trait LuaSetup {
    fn setup(lua: &mlua::Lua) -> Result<()>;
}
pub fn yrnu_setup(lua: &mlua::Lua) -> Result<()> {
    let main_table = lua.create_table()?;
    main_table.set(
        "run",
        lua.create_function(
            |lua: &Lua, (host, cmds, auth, port): (String, String, mlua::Value, mlua::Value)| {
                let port_opt = match port {
                    mlua::Value::Integer(port) => {
                        if let Ok(port) = u16::try_from(port) {
                            Some(port)
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                let rt = tokio::runtime::Runtime::new().unwrap();
                let future = async {
                    match auth {
                        mlua::Value::Table(auth) => {
                            if let Ok(host) = IpAddress::from_str(&host) {
                                if let Ok(username) = auth.get("username") {
                                    if let Ok(passwd) = auth.get("password") {
                                        Ok(connect(
                                            host,
                                            port_opt,
                                            SSHAuthType::Arguments(username, passwd),
                                        )
                                        .await)
                                    } else {
                                        Err(mlua::Error::BadArgument {
                                            to: Some("connect".to_string()),
                                            pos: 1,
                                            name: Some("auth".to_string()),
                                            cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                                                "Password is missing".to_string(),
                                            )),
                                        })
                                    }
                                } else if let Ok(keys_table) = auth.get::<mlua::Table>("keys") {
                                    let passphrase =
                                        if let Ok(passphrase) = auth.get::<String>("passphrase") {
                                            Some(passphrase)
                                        } else {
                                            None
                                        };
                                    if let (Ok(user), Ok(private)) = (
                                        keys_table.get::<String>("user"),
                                        keys_table.get::<String>("private"),
                                    ) {
                                        let public = if let Ok(public) =
                                            keys_table.get::<String>("public")
                                        {
                                            Some(PathBuf::from(public))
                                        } else {
                                            None
                                        };
                                        Ok(connect(
                                            host,
                                            port_opt,
                                            SSHAuthType::KeyPair(
                                                user,
                                                public,
                                                PathBuf::from(private),
                                                passphrase,
                                            ),
                                        )
                                        .await)
                                    } else {
                                        Err(mlua::Error::BadArgument {
                                            to: Some("connect".to_string()),
                                            pos: 1,
                                            name: Some("auth".to_string()),
                                            cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                                                "Invalid Argument".to_string(),
                                            )),
                                        })
                                    }
                                } else if let Ok(agent) = auth.get("agent") {
                                    Ok(connect(host, port_opt, SSHAuthType::Agent(agent)).await)
                                } else {
                                    Err(mlua::Error::BadArgument {
                                        to: Some("connect".to_string()),
                                        pos: 0,
                                        name: Some("auth".to_string()),
                                        cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                                            "Invalid Argument".to_string(),
                                        )),
                                    })
                                }
                            } else {
                                Err(mlua::Error::BadArgument {
                                    to: Some("connect".to_string()),
                                    pos: 0,
                                    name: Some("host".to_string()),
                                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                                        "Invalid Argument".to_string(),
                                    )),
                                })
                            }
                        }
                        mlua::Nil => {
                            if let Ok(host) = IpAddress::from_str(&host) {
                                Ok(connect(host, port_opt, SSHAuthType::UserInput).await)
                            } else {
                                Err(mlua::Error::BadArgument {
                                    to: Some("connect".to_string()),
                                    pos: 0,
                                    name: Some("host".to_string()),
                                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                                        "Invalid Argument".to_string(),
                                    )),
                                })
                            }
                        }
                        _ => Err(mlua::Error::BadArgument {
                            to: Some("connect".to_string()),
                            pos: 1,
                            name: Some("auth".to_string()),
                            cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                                "Invalid Argument".to_string(),
                            )),
                        }),
                    }
                };
                let sess = rt.block_on(future);
                match sess {
                    Ok(sess) => match sess {
                        Ok(sess) => {
                            let output_tables = lua.create_table()?;
                            let tostring = lua.create_function(|_: &Lua, this: mlua::Table| {
                                this.get::<mlua::Value>("output")
                            })?;
                            let mut output_table;
                            let mut metatable;
                            for cmd in cmds.lines() {
                                output_table = lua.create_table()?;
                                match config::run(&sess, cmd.to_owned()) {
                                    Ok((stdout, stderr, status)) => {
                                        if status == 0 {
                                            metatable = lua.create_table()?;
                                            metatable.set("__tostring", tostring.clone())?;
                                            output_table.set_metatable(Some(metatable));
                                            output_table.set("success", true)?;
                                            output_table.set("status_code", status)?;
                                            output_table.set("output", stdout.clone())?;
                                            output_table.set("stderr", stderr)?;
                                            output_tables.push(output_table)?;
                                        } else {
                                            metatable = lua.create_table()?;
                                            metatable.set("__tostring", tostring.clone())?;
                                            output_table.set_metatable(Some(metatable));
                                            output_table.set("success", false)?;
                                            output_table.set("status_code", status)?;
                                            output_table.set("output", stderr.clone())?;
                                            output_table.set("stdout", stdout)?;
                                            output_tables.push(output_table)?;
                                        }
                                    }
                                    Err(e) => {
                                        metatable = lua.create_table()?;
                                        metatable.set("__tostring", tostring.clone())?;
                                        output_table.set_metatable(Some(metatable));
                                        output_table.set("success", false)?;
                                        output_table.set("output", e.message())?;
                                        output_tables.push(output_table)?;
                                    }
                                }
                            }
                            if output_tables.raw_len() == 1 {
                                Ok(mlua::Value::Table(output_tables.get(1)?))
                            } else {
                                Ok(mlua::Value::Table(output_tables))
                            }
                        }
                        Err(e) => {
                            eprintln!("{e}");
                            Ok(mlua::Nil)
                        }
                    },
                    Err(e) => {
                        eprintln!("{e}");
                        Ok(mlua::Nil)
                    }
                }
            },
        )?,
    )?;
    let exec_func = lua.create_function(|lua: &Lua, (cmd, options): (String, mlua::Value)| {
        let mut stdin = None;
        let mut shell = if cfg!(target_os = "windows") {
            "cmd".to_string()
        } else {
            "sh".to_string()
        };
        if let mlua::Value::Table(table) = options {
            if let Ok(stdin_str) = table.get::<String>("stdin") {
                stdin = Some(stdin_str);
            }
            shell = table
                .get::<String>("shell")
                .unwrap_or(if cfg!(target_os = "windows") {
                    "cmd".to_string()
                } else {
                    "sh".to_string()
                });
            if let Ok(stdin_pipe) = table.get::<String>("stdin") {
                stdin = Some(stdin_pipe);
            }
        }
        let exec_flag = match shell.as_str() {
            "sh" | "bash" | "zsh" | "fish" | "powershell" => "-c",
            "cmd" => "/C",
            _ => {
                if cfg!(target_os = "windows") {
                    shell = "cmd".to_string();
                    "/C"
                } else {
                    shell = "sh".to_string();
                    "-c"
                }
            }
        };
        if stdin.is_some() {
            let stdin = stdin.unwrap();
            let proc_handler = std::process::Command::new(&shell)
                .arg(exec_flag)
                .arg(&cmd)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn();
            if proc_handler.is_err() {
                if let Some(e) = proc_handler.err() {
                    return Err(mlua::Error::RuntimeError(format!(
                        "Failed to run: {cmd}\nError: {e}"
                    )));
                } else {
                    return Err(mlua::Error::RuntimeError(
                        "Failed to run: {cmd}\n".to_string(),
                    ));
                }
            } else {
                let mut proc_handler = proc_handler.unwrap();
                if let Some(mut proc_stdin) = proc_handler.stdin.take() {
                    _ = proc_stdin.write_all(stdin.as_bytes());
                }
                match proc_handler.wait() {
                    Ok(status) => {
                        let mut stdout = vec![];
                        let mut stderr = vec![];
                        if let Some(mut stdout_handler) = proc_handler.stdout {
                            _ = stdout_handler.read_to_end(&mut stdout);
                        }
                        if let Some(mut stderr_handler) = proc_handler.stderr {
                            _ = stderr_handler.read_to_end(&mut stderr);
                        }
                        let output_table = lua.create_table()?;
                        let metatable = lua.create_table()?;
                        let tostring = lua.create_function(|_: &Lua, this: mlua::Table| {
                            if let (Ok(success), Ok(stdout), Ok(stderr)) = (
                                this.get::<bool>("success"),
                                this.get::<Vec<u8>>("stdout"),
                                this.get::<Vec<u8>>("stderr"),
                            ) {
                                if success {
                                    Ok(String::from_utf8(stdout)
                                        .unwrap_or_default()
                                        .trim()
                                        .to_string())
                                } else {
                                    Ok(String::from_utf8(stderr)
                                        .unwrap_or_default()
                                        .trim()
                                        .to_string())
                                }
                            } else {
                                Err(mlua::Error::BadArgument {
                                    to: Some("output".to_string()),
                                    pos: 0,
                                    name: Some("self".to_string()),
                                    cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                                        "Invalid output table".to_string(),
                                    )),
                                })
                            }
                        })?;
                        _ = metatable.set("__tostring", tostring.clone());
                        output_table.set_metatable(Some(metatable));
                        output_table.set("status_code", status.code())?;
                        output_table.set("success", status.success())?;
                        output_table.set("stdout", stdout)?;
                        output_table.set("stderr", stderr)?;
                        output_table.set("output", tostring)?;
                        Ok(output_table)
                    }
                    Err(e) => Err(mlua::Error::runtime(e.to_string())),
                }
            }
        } else {
            let output = std::process::Command::new(&shell)
                .arg(exec_flag)
                .arg(&cmd)
                .output();
            if output.is_err() {
                if let Some(e) = output.err() {
                    return Err(mlua::Error::RuntimeError(format!(
                        "Failed to run: {cmd}\nError: {e}"
                    )));
                } else {
                    return Err(mlua::Error::RuntimeError(
                        "Failed to run: {cmd}".to_string(),
                    ));
                }
            } else {
                let output = output.unwrap();
                let output_table = lua.create_table()?;
                let metatable = lua.create_table()?;
                let tostring = lua.create_function(|_: &Lua, this: mlua::Table| {
                    if let (Ok(success), Ok(stdout), Ok(stderr)) = (
                        this.get::<bool>("success"),
                        this.get::<Vec<u8>>("stdout"),
                        this.get::<Vec<u8>>("stderr"),
                    ) {
                        if success {
                            Ok(String::from_utf8(stdout)
                                .unwrap_or_default()
                                .trim()
                                .to_string())
                        } else {
                            Ok(String::from_utf8(stderr)
                                .unwrap_or_default()
                                .trim()
                                .to_string())
                        }
                    } else {
                        Err(mlua::Error::BadArgument {
                            to: Some("output".to_string()),
                            pos: 0,
                            name: Some("self".to_string()),
                            cause: std::sync::Arc::new(mlua::Error::RuntimeError(
                                "Invalid output table".to_string(),
                            )),
                        })
                    }
                })?;
                _ = metatable.set("__tostring", tostring.clone());
                output_table.set_metatable(Some(metatable));
                output_table.set("status_code", output.status.code())?;
                output_table.set("success", output.status.success())?;
                output_table.set("stdout", output.stdout)?;
                output_table.set("stderr", output.stderr)?;
                output_table.set("output", tostring)?;
                Ok(output_table)
            }
        }
    })?;
    main_table.set(
        "exec",
        lua.create_function(move |lua: &Lua, (cmds, options): (String, mlua::Value)| {
            let output_tables = lua.create_table()?;
            for cmd in cmds.lines() {
                match exec_func.call::<mlua::Table>((cmd, &options)) {
                    Ok(output) => output_tables.push(output)?,
                    Err(e) => {
                        eprintln!("{e}");
                        continue;
                    }
                }
            }
            if output_tables.raw_len() == 1 {
                Ok(output_tables.get(1)?)
            } else {
                Ok(output_tables)
            }
        })?,
    )?;
    main_table.set(
        "regex_cmp",
        lua.create_function(|_, (regex, str): (String, String)| {
            let reg = Regex::new(&regex);
            if reg.is_err() {
                return Ok(mlua::Nil);
            }
            Ok(mlua::Value::Boolean(reg.unwrap().is_match(&str)))
        })?,
    )?;
    main_table.set(
        "serialize",
        lua.create_function(
            |lua, (value, fmt, options): (mlua::Value, String, Option<mlua::Table>)| {
                let mut spaces = 4;
                let mut pretty = true;
                let mut depth = 100;
                if let Some(options) = &options {
                    spaces = options.get::<u8>("spaces").unwrap_or(4);
                    pretty = options.get::<bool>("pretty").unwrap_or(true);
                    depth = options.get::<u8>("depth").unwrap_or(100);
                }
                match fmt.as_str() {
                    "json" => Ok(mlua::Value::String(lua.create_string(if pretty {
                        to_json(value, depth).pretty(spaces as u16)
                    } else {
                        to_json(value, depth).dump()
                    })?)),
                    "yaml" => {
                        let mut yaml_str = String::new();
                        let mut emitter = yaml_rust2::emitter::YamlEmitter::new(&mut yaml_str);
                        emitter.dump(&to_yaml(value, depth)).unwrap();
                        Ok(mlua::Value::String(lua.create_string(yaml_str)?))
                    }
                    "toml" => Ok(mlua::Value::String(lua.create_string(
                        match if pretty {
                            toml::to_string_pretty(&to_toml(value, depth))
                        } else {
                            toml::to_string(&to_toml(value, depth))
                        } {
                            Ok(toml_str) => toml_str,
                            Err(e) => {
                                eprintln!("Failed to serialize the giving table.\nError: {e}");
                                return Ok(mlua::Value::Nil);
                            }
                        },
                    )?)),
                    "csv" => {
                        if let mlua::Value::Table(table) = value {
                            let csv_str = to_csv(
                                table,
                                if let Some(opts) = &options {
                                    opts.get::<Option<mlua::Table>>("headers").unwrap_or(None)
                                } else {
                                    None
                                },
                            );
                            if csv_str == "" {
                                Ok(mlua::Value::Nil)
                            } else {
                                Ok(mlua::Value::String(lua.create_string(csv_str.trim())?))
                            }
                        } else {
                            Ok(mlua::Value::Nil)
                        }
                    }
                    "xml" => {
                        if let mlua::Value::Table(table) = value {
                            let mut document = vec![];
                            if table.sequence_values::<mlua::Value>().count()
                                == table.pairs::<String, mlua::Value>().count()
                            {
                                for tag in table.sequence_values::<mlua::Value>() {
                                    if let Ok(mlua::Value::Table(table)) = tag {
                                        if let Ok(num) = table.get::<f64>("version") {
                                            let encoding = table.get::<String>("encoding").unwrap_or_default();
                                            let standalone = table.get::<String>("standalone").unwrap_or_default();
                                            document.push(Event::Decl(
                                                BytesDecl::new(
                                                    &num.to_string(),
                                                    if encoding.is_empty() {
                                                        None
                                                    } else {
                                                        Some(&encoding)
                                                    },
                                                    if standalone.is_empty() {
                                                        None
                                                    } else {
                                                        Some(&standalone)
                                                    },
                                                )
                                                .into_owned(),
                                            ));
                                            if let Ok(doctype) = table.get::<String>("doctype") {
                                                document.push(Event::DocType(BytesText::new(&doctype).into_owned()));
                                            }
                                        } else if let Ok(doctype) = table.get::<String>("doctype") {
                                            document.push(Event::DocType(BytesText::new(&doctype).into_owned()));
                                        } else {
                                            to_xml(table, &mut document, 0, depth, pretty, spaces);
                                        }
                                    } else {
                                        continue;
                                    }
                                }
                            } else {
                                to_xml(table, &mut document, 0, depth, pretty, spaces);
                            }
                            let mut writer = if pretty {
                                Writer::new_with_indent(
                                    std::io::Cursor::new(vec![]),
                                    b' ',
                                    spaces as usize,
                                )
                            } else {
                                Writer::new(std::io::Cursor::new(vec![]))
                            };
                            for event in document {
                                if let Event::Text(_) = event {
                                    if let Err(e) = writer.write_indent() {
                                        eprintln!("Failed to convert to XML.\nError: {e}");
                                        return Ok(mlua::Value::Nil);
                                    }
                                }
                                if let Err(e) = writer.write_event(event) {
                                    eprintln!("Failed to convert to XML.\nError: {e}");
                                    return Ok(mlua::Value::Nil);
                                }
                            }
                            if let Ok(str) = std::str::from_utf8(&writer.into_inner().into_inner())
                            {
                                return Ok(mlua::Value::String(lua.create_string(str)?));
                            }
                        }
                        Ok(mlua::Value::Nil)
                    }
                    _ => {
                        eprintln!("Invalid format type.\nCurrently supported formats are json/toml/yaml/xml/csv");
                        Ok(mlua::Value::Nil)
                    },
                }
            },
        )?,
    )?;
    main_table.set(
        "deserialize",
        lua.create_function(|lua, (value, fmt): (String, String)| match fmt.as_str() {
            "json" => match json::parse(&value) {
                Ok(json_val) => Ok(from_json(lua, &json_val)),
                Err(e) => {
                    eprintln!("Failed to parse JSON string!: {e}");
                    Ok(mlua::Value::Nil)
                }
            },
            "yaml" => match yaml_rust2::YamlLoader::load_from_str(&value) {
                Ok(yaml_val) => Ok(from_yaml(lua, &yaml_rust2::yaml::Yaml::Array(yaml_val))),
                Err(e) => {
                    eprintln!("Failed to parse YAML string!: {e}");
                    Ok(mlua::Value::Nil)
                }
            },
            "toml" => match value.trim().parse::<toml::Table>() {
                Ok(toml_val) => Ok(from_toml(lua, &toml::Value::Table(toml_val))),
                Err(e) => {
                    eprintln!("Failed to parse TOML string!: {e}");
                    Ok(mlua::Value::Nil)
                }
            },
            "xml" => {
                let mut reader = Reader::from_str(value.trim());
                let tags = lua.create_table()?;
                reader.config_mut().trim_text(true);
                let mut events = vec![];
                loop {
                    match reader.read_event() {
                        Ok(Event::Eof) => break,
                        Ok(e) => events.push(e),
                        Err(e) => {
                            eprintln!("Failed to parse XML.\nError: {e}");
                            return Ok(mlua::Value::Nil);
                        }
                    }
                }
                let mut index = 0;
                while index < events.len() {
                    if let Event::Decl(decl) = &events[index] {
                        if let Ok(version) = decl.version() {
                            let decl_table = lua.create_table()?;
                            if let Ok(version) = String::from_utf8(version.as_ref().to_vec()) {
                                decl_table.set("version", version)?;
                            }
                            if let Some(Ok(encoding)) = decl.encoding() {
                                if let Ok(encoding) = String::from_utf8(encoding.as_ref().to_vec())
                                {
                                    decl_table.set("encoding", encoding)?;
                                }
                            }
                            if let Some(Ok(standalone)) = decl.standalone() {
                                if let Ok(standalone) =
                                    String::from_utf8(standalone.as_ref().to_vec())
                                {
                                    decl_table.set("standalone", standalone)?;
                                }
                            }
                            tags.push(decl_table)?;
                            index += 1;
                        }
                    } else if let Event::DocType(doctype) = &events[index] {
                        let doctype_table = lua.create_table()?;
                        let doctype = String::from_utf8(doctype.to_vec());
                        if let Err(e) = doctype {
                            eprintln!("Faild to parse XML.\nError: {e}");
                            return Ok(mlua::Value::Nil);
                        }
                        doctype_table.set("doctype", doctype.unwrap())?;
                        tags.push(doctype_table)?;
                        index += 1;
                    } else if let Some((table, ind)) = from_xml(
                        lua,
                        &events,
                        index,
                        if let Event::Empty(_) = events[0] {
                            true
                        } else {
                            false
                        },
                    ) {
                        index = ind + 1;
                        tags.push(mlua::Value::Table(table))?;
                    } else {
                        return Ok(mlua::Value::Nil);
                    }
                }
                Ok(mlua::Value::Table(tags))
            }
            "csv" => Ok(if let Some(table) = from_csv(lua, value) {
                mlua::Value::Table(table)
            } else {
                mlua::Value::Nil
            }),
            _ => Ok(mlua::Value::Nil),
        })?,
    )?;
    lua.globals().set("yrnu", main_table)?;
    Ok(())
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
    _ = yrnu_setup(&lua);
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
