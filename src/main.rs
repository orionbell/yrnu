use clap::{
    builder, command, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command, Subcommand,
};
use mlua::Lua;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use yrnu::core::{Interface, IpAddress, IpKind, IpVersion, MacAddress, Mask, Network};
use yrnu::lua;

fn success(str: &str) -> String {
    format!("\x1b[32m{str}\x1b[0m")
}
fn error(str: &str) -> String {
    format!("\x1b[31m{str}\x1b[0m")
}
fn handle_cli_matches(
    lua: &Lua,
    matches: &ArgMatches,
    plugin: &(String, mlua::Table),
) -> mlua::Result<String> {
    let (name, table): &(String, mlua::Table) = plugin;
    if let Some((name, plugin_args)) = matches.subcommand() {
        let config_table = lua.create_table().unwrap_or_else(|_| {
            eprintln!("{}", error("Something went wrong..."));
            std::process::exit(10)
        });
        let args = table.get::<mlua::Table>("args");
        if let Ok(args) = args {
            let mut arg_name;
            let mut arg_opts;
            for arg in args.pairs::<String, mlua::Table>() {
                (arg_name, arg_opts) = arg.unwrap_or_else(|_| {
                    eprintln!("{}", error("Invalid argument definition of args"));
                    std::process::exit(11)
                });
                let name = arg_name.clone();
                let update =
                    arg_opts
                        .get::<mlua::Function>("update")
                        .unwrap_or(lua.create_function(
                            move |_, (this, value): (mlua::Table, bool)| {
                                _ = this.set(name.to_owned(), value);
                                Ok(())
                            },
                        )?);
                match arg_opts
                    .get::<String>("arg_type")
                    .unwrap_or_default()
                    .as_str()
                {
                    "bool" => {
                        let value = plugin_args.get_one::<bool>(&arg_name).unwrap();
                        _ = update
                            .call::<(mlua::Table, bool)>((config_table.clone(), value.clone()))
                    }
                    "number" => {
                        let value = plugin_args.get_one::<u8>(&arg_name).unwrap();
                        _ = update.call::<(mlua::Table, mlua::Number)>((
                            config_table.clone(),
                            value.clone(),
                        ))
                    }
                    "table" => {
                        let value = plugin_args
                            .get_many::<String>(&arg_name)
                            .unwrap_or_default()
                            .map(|v| v.as_str())
                            .collect::<Vec<_>>();
                        _ = update.call::<(mlua::Table, mlua::Table)>((
                            config_table.clone(),
                            value.clone(),
                        ))
                    }
                    "nil" => _ = update.call::<mlua::Table>(config_table.clone()),
                    _ => {
                        let value = plugin_args.get_one::<String>(&arg_name).unwrap();
                        _ = update
                            .call::<(mlua::Table, String)>((config_table.clone(), value.clone()))
                    }
                }
            }
        }
        let config_func = table.get::<mlua::Function>("config");
        let mut config_str = if let Ok(config) = config_func {
            config.call::<String>(config_table)?
        } else {
            "".to_string()
        };
        let subcmds = table.get::<mlua::Table>("subcommands");
        if let Ok(subcmds) = subcmds {
            let mut scmd_name;
            let mut scmd_opts;
            let mut subconfig;
            for subcmd in subcmds.pairs::<String, mlua::Table>() {
                (scmd_name, scmd_opts) = subcmd?;
                subconfig = handle_cli_matches(lua, plugin_args, &(scmd_name, scmd_opts));
                if let Ok(subconfig) = subconfig {
                    config_str = format!("{config_str}{}", subconfig);
                }
            }
        }
        return Ok(config_str);
    }
    Ok("".to_string())
}
fn handle_lua_api(
    lua: &Lua,
    arg_name: String,
    arg_options: &mlua::Table,
) -> mlua::Result<mlua::Function> {
    let update = arg_options.get::<mlua::Function>("update");
    let arg_type = arg_options.get::<String>("arg_type");
    let func = match arg_type.unwrap_or_default().as_str() {
        "IpVersion" => lua.create_function(move |_, (this, value): (mlua::Table, IpVersion)| {
            if let Ok(update) = &update {
                match update.call::<()>((this, value)) {
                    Err(e) => {
                        println!("{e}");
                    }
                    _ => {}
                }
            } else {
                this.set(arg_name.to_owned(), value)?;
            }
            Ok(())
        })?,
        "IpKind" => lua.create_function(move |_, (this, value): (mlua::Table, IpKind)| {
            if let Ok(update) = &update {
                update.call((this, value))?;
            } else {
                this.set(arg_name.to_owned(), value)?;
            }
            Ok(())
        })?,
        "IpAddress" => lua.create_function(move |_, (this, value): (mlua::Table, IpAddress)| {
            if let Ok(update) = &update {
                update.call((this, value))?;
            } else {
                this.set(arg_name.to_owned(), value)?;
            }
            Ok(())
        })?,
        "Mask" => lua.create_function(move |_, (this, value): (mlua::Table, Mask)| {
            if let Ok(update) = &update {
                update.call((this, value))?;
            } else {
                this.set(arg_name.to_owned(), value)?;
            }
            Ok(())
        })?,
        "Network" => lua.create_function(move |_, (this, value): (mlua::Table, Network)| {
            if let Ok(update) = &update {
                update.call((this, value))?;
            } else {
                this.set(arg_name.to_owned(), value)?;
            }
            Ok(())
        })?,
        "Interface" => lua.create_function(move |_, (this, value): (mlua::Table, Interface)| {
            if let Ok(update) = &update {
                update.call((this, value))?;
            } else {
                this.set(arg_name.to_owned(), value)?;
            }
            Ok(())
        })?,
        "MacAddress" => {
            lua.create_function(move |_, (this, value): (mlua::Table, MacAddress)| {
                if let Ok(update) = &update {
                    update.call((this, value))?;
                } else {
                    this.set(arg_name.to_owned(), value)?;
                }
                Ok(())
            })?
        }
        "bool" => lua.create_function(move |_, (this, value): (mlua::Table, bool)| {
            if let Ok(update) = &update {
                update.call((this, value))?;
            } else {
                this.set(arg_name.to_owned(), value)?;
            }
            Ok(())
        })?,
        "number" => lua.create_function(move |_, (this, value): (mlua::Table, mlua::Number)| {
            if let Ok(update) = &update {
                update.call((this, value))?;
            } else {
                this.set(arg_name.to_owned(), value)?;
            }
            Ok(())
        })?,
        "table" => lua.create_function(move |_, (this, value): (mlua::Table, mlua::Table)| {
            if let Ok(update) = &update {
                update.call((this, value))?;
            } else {
                this.set(arg_name.to_owned(), value)?;
            }
            Ok(())
        })?,
        "nil" => lua.create_function(move |_, this: mlua::Table| {
            if let Ok(update) = &update {
                update.call(this)?;
            } else {
                this.set(arg_name.to_owned(), true)?;
            }
            Ok(())
        })?,
        "string" => lua.create_function(move |_, (this, value): (mlua::Table, String)| {
            if let Ok(update) = &update {
                update.call((this, value))?;
            } else {
                this.set(arg_name.to_owned(), value)?;
            }
            Ok(())
        })?,
        _ => lua.create_function(move |_, (this, value): (mlua::Table, mlua::Value)| {
            if let Ok(update) = &update {
                match update.call::<()>((this, value)) {
                    Err(e) => {
                        println!("{e}");
                    }
                    _ => {}
                }
            } else {
                this.set(arg_name.to_owned(), value)?;
            }
            Ok(())
        })?,
    };
    Ok(func)
}
fn handle_cli((arg_name, arg_options): (&String, &mlua::Table)) -> mlua::Result<Arg> {
    let mut arg = Arg::new(arg_name);
    if let Ok(required) = arg_options.get::<bool>("required") {
        arg = arg.required(required);
    }
    if let Ok(short) = arg_options.get::<String>("short") {
        if short.len() == 0 {
            return Err(todo!());
        }
        arg = arg.short(short.chars().next().unwrap());
    } else {
        arg = arg.short(arg_name.chars().next().unwrap())
    }
    if let Ok(long) = arg_options.get::<String>("long") {
        if long.len() == 0 {
            return Err(todo!());
        }
        arg = arg.long(long);
    } else {
        arg = arg.long(arg_name);
    }
    if let Ok(help) = arg_options.get::<String>("help") {
        arg = arg.help(help);
    }
    arg = match arg_options
        .get::<String>("action")
        .unwrap_or_default()
        .as_str()
    {
        "store-true" => arg.action(ArgAction::SetTrue),
        "store-false" => arg.action(ArgAction::SetFalse),
        "store-count" => arg.action(ArgAction::Count),
        "store-table" => arg.action(ArgAction::Append),
        _ => arg,
    };
    Ok(arg)
}
fn load_plugin(
    lua: &Lua,
    plugin: (&String, &mlua::Table),
) -> mlua::Result<(Command, mlua::Function)> {
    let (name, table) = plugin;
    let start_config = table.get("preconfig").unwrap_or("".to_string());
    let end_config = table.get("postconfig").unwrap_or("".to_string());
    let mut subcmd = Command::new(name);
    let args = table.get::<mlua::Table>("args")?;
    let subcommands = table.get::<mlua::Table>("subcommands");
    if let Ok(about) = table.get::<String>("about") {
        subcmd = subcmd.about(about);
    }
    let plugin_table = lua.create_table()?;
    let config_func = table.get::<mlua::Function>("config")?;
    let config_func = lua.create_function(move |_, this: mlua::Table| {
        let mut conf = start_config.clone();
        conf = conf + &config_func.call::<String>(this)?;
        conf = conf + &end_config;
        Ok(conf)
    })?;
    plugin_table.set("config", config_func)?;
    let mut arg;
    let mut func;
    let mut arg_name;
    let mut arg_opts;
    let mut arg_values;
    for pair in args.pairs::<String, mlua::Value>() {
        (arg_name, arg_values) = pair?;
        arg_opts = match arg_values {
            mlua::Value::Table(value) => value,
            mlua::Value::Nil => {
                let table = lua.create_table()?;
                table.set("help", "")?;
                table
            }
            mlua::Value::String(value) => {
                let table = lua.create_table()?;
                table.set("help", value)?;
                table
            }
            _ => {
                eprintln!("{}",
                    error(format!("Faild to load argument {}!, value should be either a table, string or nil", arg_name).as_str())
                );
                continue;
            }
        };
        arg = handle_cli((&arg_name, &arg_opts))?;
        subcmd = subcmd.arg(arg);
        func = handle_lua_api(lua, arg_name.clone(), &arg_opts)?;
        plugin_table.set(format!("set_{}", arg_name), func)?;
    }
    let mut scmd_name;
    let mut scmd_table;
    if let Ok(subcommands) = subcommands {
        for pair in subcommands.pairs::<String, mlua::Table>() {
            (scmd_name, scmd_table) = pair?;
            let name_clone = scmd_name.clone();
            if let Ok((scmd, constructor)) = load_plugin(lua, (&scmd_name, &scmd_table)) {
                let construct = lua.create_function(
                    move |_, (this, param): (mlua::Table, Option<mlua::Table>)| {
                        this.set(
                            name_clone.to_owned(),
                            constructor.call::<Option<mlua::Table>>(param)?,
                        )?;
                        Ok(())
                    },
                )?;
                plugin_table.set(format!("add_{}", scmd_name), construct)?;
                subcmd = subcmd.subcommand(scmd);
            } else {
                eprintln!(
                    "{}",
                    error(format!("\x1b[31mFaild to load {} plugin\x1b[0m", scmd_name).as_str())
                );
            }
        }
    }
    let init_func = if let Ok(init) = table.get::<mlua::Function>("init") {
        lua.create_function(move |_, param: Option<mlua::Table>| {
            init.call::<Option<mlua::Table>>((plugin_table.clone(), param))
        })
    } else {
        lua.create_function(move |_, ()| Ok(plugin_table.clone()))
    };
    lua.globals().set(name.to_owned(), init_func.to_owned()?)?;
    println!("{} load {} plugin !", success("Successfuly"), name);
    Ok((subcmd, init_func?))
}
fn load_plugins(lua: &Lua) -> mlua::Result<Vec<(String, mlua::Table)>> {
    let path = dirs::config_dir().unwrap().join("yrnu");
    let file = path.join("init.lua");
    if file.is_file() {
        lua.load(format!(
            "package.path = package.path .. \";{}/?.lua\"",
            path.display()
        ))
        .exec()?;
        let init = lua
            .load(std::fs::read_to_string(file)?)
            .eval::<mlua::Table>()?;
        let plugins = init
            .get::<mlua::Table>("plugins")
            .unwrap_or(lua.create_table()?);
        if plugins.is_empty() {
            println!("No plugins to load");
            Ok(vec![])
        } else {
            let mut plugin_list = vec![];
            for plugin in plugins.pairs::<String, mlua::Table>() {
                let (name, table) = plugin?;
                plugin_list.push((name, table));
            }
            Ok(plugin_list)
        }
    } else {
        println!("No plugins to load");
        Ok(vec![])
    }
}

fn setup() {
    if let Some(mut path) = dirs::config_dir() {
        path.push("Yrnu");
        if !path.is_dir() {
            _ = std::fs::create_dir_all(path);
        }
    } else {
        eprintln!("Error: Unable to find config folder");
        std::process::exit(1);
    }
}

fn read_script(name: String) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let lua_ctx = lua::init().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1)
    });
    setup();
    let plugins = load_plugins(&lua_ctx).unwrap_or_else(|e| {
        eprintln!("Failed to load plugins: \n{}", e.to_string());
        vec![]
    });
    let mut config_args =
        Command::new("config").about("configure linux/Windows machines and network devices.");
    let args = command!()
        .about(
            "a tool for networking and cyber specialists, 
featuring Lua scripting and the yrnu library for crafting networking tools and automating tasks. 
Key features include configuring network settings, sending custom traffic, and deploying servers.",
        )
        .version("0.0.1")
        .subcommand(Command::new("packet").about("send and sniff network packets."))
        .subcommand(Command::new("server").about("spown varius types of servers."))
        .arg(
            Arg::new("script")
                .help("A lua script to execute")
                .exclusive(true)
                .value_name("SCRIPT"),
        );
    for (name, table) in &plugins {
        let plugin = load_plugin(&lua_ctx, (name, table));
        if plugin.is_err() {
            println!("failed!");
            eprintln!("{:?}", plugin);
        } else {
            config_args = config_args.subcommand(plugin.unwrap().0);
        }
    }
    let args_matches = args.subcommand(config_args).get_matches();
    if let Some(script) = args_matches.get_one::<String>("script") {
        match read_script(script.to_string()) {
            Ok(contents) => {
                if let Err(e) = lua::run(&lua_ctx, &contents) {
                    eprintln!("{}", e);
                }
            }
            Err(e) => eprintln!("{}", e),
        }
    } else {
        match args_matches.subcommand() {
            Some(("config", config)) => {
                for plugin in &plugins {
                    println!("{}", handle_cli_matches(&lua_ctx, config, plugin).unwrap());
                }
            }
            _ => lua::interpreter::start_interpreter(&lua_ctx).expect("Failed to run interpreter."),
        }
    }
}
