pub mod core_lua;
pub mod interpreter;
use crate::config::{self, connect, SSHAuthType};
use crate::core::*;
use mlua::{IntoLua, Lua, Result, StdLib};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::path::PathBuf;

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
                            if let Ok(host) = IpAddress::new(&host) {
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
                            if let Ok(host) = IpAddress::new(&host) {
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
                                        output_table.set("success", true)?;
                                        if status == 0 {
                                            metatable = lua.create_table()?;
                                            metatable.set("__tostring", tostring.clone())?;
                                            output_table.set_metatable(Some(metatable));
                                            output_table.set("success", true)?;
                                            output_table.set("status_code", status)?;
                                            output_table.set("output", stdout.clone())?;
                                            output_table.set("stdout", stdout)?;
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
                                            output_table.set("stderr", stderr)?;
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
                    return Err(mlua::Error::RuntimeError(
                        "Failed to run: {cmd}\nError: {e}".to_string(),
                    ));
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
    lua.globals().set("yrnu", main_table)?;
    Ok(())
}

pub fn init() -> Result<Lua> {
    let lua = Lua::new();
    lua.load_std_libs(StdLib::ALL_SAFE)?;
    _ = yrnu_setup(&lua);
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

pub fn run(lua: &Lua, code: &str) -> Result<()> {
    lua.load(code).exec()?;
    Ok(())
}
