use clap::builder;
use clap::{command, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command, ValueEnum};
use git2::FetchOptions;
use mlua::Lua;
use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;
use yrnu::core::{IpAddress, MacAddress, Mask, Network};
use yrnu::lua;

/// Returns a giving string in a green color
fn success(str: &str) -> String {
    format!("\x1b[32m{str}\x1b[0m")
}

/// Returns a giving string in a red color
fn error(str: &str) -> String {
    format!("\x1b[31m{str}\x1b[0m")
}

/// Handle the cli usage of the plugins
fn handle_cli_matches(
    lua: &Lua,
    matches: &ArgMatches,
    plugin: &(String, mlua::Table),
) -> mlua::Result<String> {
    let (name, table): &(String, mlua::Table) = plugin;
    if let Some((active_scmd_name, plugin_args)) = matches.subcommand() {
        if active_scmd_name != name {
            return Ok(String::new());
        }
        let config_table = lua.create_table().unwrap_or_else(|_| {
            eprintln!("{}", error("Something went wrong..."));
            std::process::exit(10)
        });
        let args = table.get::<mlua::Table>("args");
        if let Ok(args) = args {
            let mut arg_name;
            let mut arg_opts;
            let mut arg_values;
            for arg in args.pairs::<String, mlua::Value>() {
                (arg_name, arg_values) = arg.unwrap_or_else(|_| {
                    eprintln!("{}", error("Invalid argument definition of args"));
                    std::process::exit(11)
                });
                arg_opts = match arg_values {
                    mlua::Value::Table(value) => value,
                    mlua::Value::String(value) => {
                        let table = lua.create_table()?;
                        if value.to_str().unwrap().len() == 1 {
                            table.set("short", value)?;
                        } else {
                            table.set("help", value)?;
                        }
                        table
                    }
                    _ => {
                        eprintln!("{}",
                    error(format!("Faild to load argument {}!, value should be either a table or a string", arg_name).as_str())
                );
                        continue;
                    }
                };
                let name = arg_name.clone();
                let update =
                    arg_opts
                        .get::<mlua::Function>("update")
                        .unwrap_or(lua.create_function(
                            move |_, (this, value): (mlua::Table, mlua::Value)| {
                                _ = this.set(name.to_owned(), value);
                                Ok(())
                            },
                        )?);
                match arg_opts
                    .get::<String>("arg_type")
                    .unwrap_or_default()
                    .as_str()
                {
                    "bool" | "boolish" => {
                        let value = plugin_args.get_one::<bool>(&arg_name);
                        if let Some(value) = value {
                            _ = update
                                .call::<(mlua::Table, bool)>((config_table.clone(), value.clone()))
                        }
                    }
                    "int" => {
                        let value = plugin_args.get_one::<i64>(&arg_name);
                        if let Some(value) = value {
                            _ = update.call::<(mlua::Table, mlua::Number)>((
                                config_table.clone(),
                                value.clone(),
                            ))
                        }
                    }
                    "uint" => {
                        let value = plugin_args.get_one::<u64>(&arg_name);
                        if let Some(value) = value {
                            _ = update.call::<(mlua::Table, mlua::Number)>((
                                config_table.clone(),
                                value.clone(),
                            ))
                        }
                    }
                    "real" => {
                        let value = plugin_args.get_one::<f64>(&arg_name);
                        if let Some(value) = value {
                            _ = update.call::<(mlua::Table, mlua::Number)>((
                                config_table.clone(),
                                value.clone(),
                            ))
                        }
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
                        let value = plugin_args.get_one::<String>(&arg_name);
                        if let Some(value) = value {
                            _ = update.call::<(mlua::Table, String)>((
                                config_table.clone(),
                                value.clone(),
                            ))
                        }
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

// Handle the creation of plugin argument lua API
fn handle_lua_api(
    lua: &Lua,
    arg_name: String,
    arg_options: &mlua::Table,
) -> mlua::Result<mlua::Function> {
    let update = arg_options.get::<mlua::Function>("update");
    let arg_type = arg_options.get::<String>("arg_type").unwrap_or_default();
    let func = if arg_type == "nil" {
        return Err(mlua::Error::RuntimeError(
            "Arguments with arg_type = \"nil\" have to implement update function".to_string(),
        ));
    } else {
        lua.create_function(move |_, (this, value): (mlua::Table, mlua::Value)| {
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
        })?
    };
    Ok(func)
}

// Handle the creation of plugin argument CLI
fn handle_cli((arg_name, arg_options): (&String, &mlua::Table)) -> mlua::Result<Arg> {
    let mut arg = Arg::new(arg_name);
    if let Ok(required) = arg_options.get::<bool>("required") {
        arg = arg.required(required);
    }
    if let Ok(short) = arg_options.get::<String>("short") {
        if short.len() == 0 {
            arg = arg.short(arg_name.chars().next().unwrap())
        } else {
            arg = arg.short(short.chars().next().unwrap());
        }
    } else {
        arg = arg.short(arg_name.chars().next().unwrap())
    }
    if let Ok(long) = arg_options.get::<String>("long") {
        if long.len() == 0 {
            arg = arg.long(arg_name);
        } else {
            arg = arg.long(long);
        }
    } else {
        arg = arg.long(arg_name);
    }
    if let Ok(help) = arg_options.get::<String>("help") {
        arg = arg.help(help);
    }
    if let Ok(values) = arg_options.get::<Vec<String>>("possible_values") {
        arg = arg.value_parser(values.to_owned());
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
        _ => match arg_options
            .get::<String>("arg_type")
            .unwrap_or_default()
            .as_str()
        {
            "bool" => arg.value_parser(builder::BoolValueParser::new()),
            "path" => arg.value_parser(builder::PathBufValueParser::new()),
            "int" => {
                let max = arg_options.get::<i64>("max").unwrap_or(i64::MAX);
                let min = arg_options.get::<i64>("min").unwrap_or(i64::MIN);
                arg.allow_negative_numbers(true)
                    .value_parser(value_parser!(i64).range(min..max))
            }
            "uint" => {
                let max = arg_options.get::<u64>("max").unwrap_or(u64::MAX);
                let min = arg_options.get::<u64>("min").unwrap_or(u64::MIN);
                arg.allow_negative_numbers(true)
                    .value_parser(value_parser!(u64).range(min..max))
            }
            "real" => arg
                .allow_negative_numbers(true)
                .value_parser(value_parser!(f64)),
            "boolish" => arg.value_parser(builder::BoolishValueParser::new()),
            "ip-address" => arg.value_parser(IpAddress::new),
            "network" => arg.value_parser(Network::from_str),
            "mask" => arg.value_parser(Mask::new),
            "mac-address" => arg.value_parser(MacAddress::new),
            _ => arg,
        },
    };
    Ok(arg)
}

/// Load a specific plugin
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
            mlua::Value::String(value) => {
                let table = lua.create_table()?;
                if value.to_str().unwrap().len() == 1 {
                    table.set("short", value)?;
                } else {
                    table.set("help", value)?;
                }
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
    //println!("{} load {} plugin !", success("Successfuly"), name);
    Ok((subcmd, init_func?))
}

/// Gets and installs all the plugins that been specified in the init.lua file
fn load_plugins(lua: &Lua, path: &mut PathBuf) -> mlua::Result<Vec<(String, mlua::Table)>> {
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
        let ensure_installed = init
            .get::<mlua::Table>("ensure_installed")
            .unwrap_or(lua.create_table()?);
        if !ensure_installed.is_empty() {
            let mut username;
            let mut repo;
            let mut inner_repo;
            let mut inner_username;
            let mut plugins_path = path.clone();
            plugins_path.push("plugins");
            if !plugins_path.is_dir() {
                _ = std::fs::create_dir_all(plugins_path.clone());
            }
            for pair in ensure_installed.pairs::<mlua::Value, mlua::Value>() {
                (username, repo) = pair?;
                match (username, repo) {
                    (mlua::Value::String(value), mlua::Value::Table(table)) => {
                        for link in table.pairs::<mlua::Value, String>() {
                            (_, inner_repo) = link?;
                            inner_username = value.to_str().unwrap();
                            inner_repo = format!("{inner_username}/{inner_repo}");
                            clone_plugin(&inner_repo, &mut plugins_path, false);
                        }
                    }
                    (mlua::Value::String(username), mlua::Value::String(repo)) => {
                        inner_username = username.to_str().unwrap();
                        let inner_repo = repo.to_str().unwrap();
                        clone_plugin(
                            &format!("{inner_username}/{inner_repo}"),
                            &mut plugins_path,
                            false,
                        );
                    }
                    (_, mlua::Value::String(link)) => {
                        clone_plugin(&link.to_str().unwrap(), &mut plugins_path, false);
                    }
                    _ => continue,
                }
            }
        }
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

/// Setup the plugin directory path
fn setup() -> PathBuf {
    let mut path = match std::env::var("YRNU_CONFIG_DIR") {
        Ok(path) => PathBuf::from(path),
        Err(_) => dirs::config_dir().unwrap_or_else(|| {
            eprintln!("Error: Unable to find config folder");
            std::process::exit(1);
        }),
    };
    path.push("yrnu");
    if !path.is_dir() {
        _ = std::fs::create_dir_all(path.clone());
    }
    path
}

/// Clone a remote plugin using https (todo ssh)
fn clone_plugin(link: &str, plugin_dir: &mut PathBuf, force: bool) -> String {
    let url_link = url::Url::parse(link).unwrap_or_else(|e| {
        if link.split("/").count() == 2 {
            url::Url::parse(&format!("https://github.com/{link}")).unwrap_or_else(|e| {
                eprintln!("Invalid URL: {link}\nError: {e}");
                std::process::exit(1)
            })
        } else {
            eprintln!("Invalid URL: {link}\nError: {e}");
            std::process::exit(1)
        }
    });
    let mut plugin_name = url_link
        .path_segments()
        .into_iter()
        .last()
        .unwrap()
        .map(|s| format!("{s}/"))
        .collect::<String>();
    plugin_name.pop();
    if plugin_name.ends_with(".git") {
        plugin_name = plugin_name[..plugin_name.len() - 4].to_owned();
    }
    plugin_name = plugin_name.replace("-", "_").replace("/", "-");
    plugin_dir.push(&plugin_name);
    if !plugin_dir.is_dir() {
        _ = std::fs::create_dir_all(plugin_dir.clone());
    } else if let Ok(entries) = fs::read_dir(&plugin_dir) {
        if entries.into_iter().count() > 0 && !force {
            eprintln!("The plugin directory is installed and is not empty! use \"-f\" to force");
            std::process::exit(1)
        } else if force {
            _ = std::fs::remove_dir_all(plugin_dir.clone());
            _ = std::fs::create_dir_all(plugin_dir.clone());
        }
    }
    let mut builder = git2::build::RepoBuilder::new();
    let mut fo = FetchOptions::new();
    fo.depth(1);
    builder.fetch_options(fo);
    if let Err(e) = builder.clone(url_link.as_str(), plugin_dir) {
        if e.code() != git2::ErrorCode::Exists {
            if e.class() == git2::ErrorClass::Http {
                _ = builder.clone(link, plugin_dir).unwrap_or_else(|e| {
                    eprintln!("Failed cloning plugin repo: {e}");
                    std::process::exit(1);
                })
            } else if e.class() == git2::ErrorClass::Net {
                eprintln!("Network error: {}", e.message());
                std::process::exit(1);
            } else {
                eprintln!("Failed cloning plugin repo: {e}");
                std::process::exit(1);
            }
        }
    };
    plugin_name
}

/// Clone a local directory that contains a plugin to the plugin directory
fn clone_local_plugin(src: &PathBuf, plugin_dir: &PathBuf, force: bool, mov: bool) -> String {
    let name = src.file_name().unwrap_or_else(|| {
        eprintln!("Invalid path: {}", src.to_str().unwrap_or_default());
        std::process::exit(1);
    });
    let plugin_dir = plugin_dir.join(name);
    if !plugin_dir.is_dir() {
        _ = std::fs::create_dir_all(plugin_dir.clone());
    } else if let Ok(entries) = fs::read_dir(&plugin_dir) {
        if entries.into_iter().count() > 0 && !force {
            eprintln!("The plugin directory is installed and is not empty! use \"-f\" to force");
            std::process::exit(1)
        }
    }
    if let Ok(entries) = fs::read_dir(src) {
        let mut entry;
        let mut entry_path;
        let mut entry_name;
        let mut entry_typ;
        for entry_res in entries {
            entry = entry_res.unwrap();
            entry_path = entry.path();
            entry_typ = entry.file_type().unwrap_or_else(|e| {
                eprintln!("Something went wrong.\nError: {}", e);
                std::process::exit(1);
            });
            entry_name = entry.file_name();
            if entry_typ.is_dir() {
                _ = clone_local_plugin(&entry_path, &plugin_dir, force, mov);
                _ = fs::remove_dir(&entry_path);
            } else {
                _ = fs::copy(&entry_path, plugin_dir.join(entry_name));
                _ = fs::remove_file(&entry_path);
            }
        }
        if mov {}
    }
    name.to_str()
        .unwrap_or_else(|| {
            eprintln!("Invalid path: {}", src.to_str().unwrap_or_default());
            std::process::exit(1);
        })
        .to_owned()
}

/// Reads a lua script and returns its content
fn read_script(name: String) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Prints all the plugins that currently installed on the system
fn list(path: &PathBuf, lib: bool) {
    let path = if lib {
        path.join("libs")
    } else {
        path.join("plugins")
    };
    if let Ok(entries) = fs::read_dir(path) {
        let mut entry;
        let mut entry_name;
        let mut entry_typ;
        let mut name;
        let mut ind = 0;
        for entry_res in entries {
            if ind == 0 {
                println!("{}:", if lib { "libraries" } else { "plugins" });
            }
            ind += 1;
            entry = entry_res.unwrap();
            entry_typ = entry.file_type().unwrap_or_else(|e| {
                eprintln!("Something went wrong.\nError: {}", e);
                std::process::exit(1);
            });
            entry_name = entry.file_name();
            name = entry_name.to_str().unwrap_or("");
            if entry_typ.is_dir() && entry_name != "" {
                println!("\x1b[1;34m{ind}. {name}\x1b[0m");
            }
        }
        if ind > 0 {
            println!(
                "total of {ind} {}",
                if lib { "libraries" } else { "plugins" }
            );
        }
    }
}
fn main() {
    let lua_ctx = lua::init().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1)
    });
    let mut path = setup();
    let plugins = load_plugins(&lua_ctx, &mut path).unwrap_or_else(|e| {
        eprintln!("Failed to load plugins: \n{}", e.to_string());
        vec![]
    });
    let mut config_args = Command::new("config")
        .about("configure linux/Windows machines and network devices.")
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("Config in an interactive way by asking for each argument one by one")
                .action(ArgAction::SetTrue),
        );
    let args = command!()
        .about(
            "a tool for networking and cyber specialists, 
featuring Lua scripting and the yrnu library for crafting networking tools and automating tasks. 
Key features include configuring network settings, sending custom traffic, and deploying servers.",
        )
        .version("0.0.1")
        .subcommand(
            Command::new("add")
                .about("Add yrnu plugins or library")
                .arg(
                    Arg::new("lib")
                        .help("for libaries")
                        .short('l')
                        .long("lib")
                        .action(ArgAction::SetTrue),
                ).arg(
                    Arg::new("force")
                        .help("force the installation, this will override any file in the plugin directory if exists")
                        .short('f')
                        .long("force")
                        .action(ArgAction::SetTrue),
                )
                .group(
                    ArgGroup::new("source")
                        .arg("url")
                        .arg("path")
                        .required(true),
                )
                .arg(
                    Arg::new("url")
                        .index(1)
                        .help("plugin reposiroty url")
                        .conflicts_with("path")
                        .value_name("URL"),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .conflicts_with("url")
                        .help("local plugin reposiroty path")
                        .value_name("PATH"),
                )
                .arg(
                    Arg::new("move")
                        .help("moves the folder")
                        .short('m')
                        .long("move")
                        .action(ArgAction::SetTrue)
                        .requires("path"),
                ),
        )
        .subcommand(
            Command::new("remove").about("Remove yrnu plugins or library")
                .arg(
                    Arg::new("name")
                        .help("Remove plugin")
                        .index(1)
                        .required(true)
                        .value_name("NAME"),
            )
                .arg(
                    Arg::new("yes")
                        .short('y')
                        .long("yes")
                        .help("don't ask for confirmation.")
                        .action(ArgAction::SetTrue)
            )
                .arg(
                    Arg::new("lib").help("for libaries")
                        .short('l')
                        .long("lib")
                        .help("for libraries.")
                        .action(ArgAction::SetTrue)
            ),
        )
        .subcommand(Command::new("list")
            .about("List yrnu plugins and Libraries")
            .arg(
                Arg::new("lib")
                    .help("List only libraries.")
                    .short('l')
                    .long("lib")
                    .conflicts_with("plugin")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("plugin") 
                    .help("List only plugins.")
                    .short('p')
                    .long("plugin")
                    .conflicts_with("lib")
                    .action(ArgAction::SetTrue)
            )
        )
        .subcommand(Command::new("packet").about("Send and sniff network packets."))
        .subcommand(Command::new("server").about("Spown varius types of servers."))
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
                    let output = handle_cli_matches(&lua_ctx, config, plugin).unwrap();
                    if output != "" {
                        println!("{}", output.trim());
                    }
                }
            }
            Some(("add", add_args)) => {
                let mut plugin_dir = path;
                let force = *add_args.get_one::<bool>("force").unwrap();
                if *add_args.get_one::<bool>("lib").unwrap() {
                    plugin_dir.push("libs");
                } else {
                    plugin_dir.push("plugins");
                }
                if let Some(link) = add_args.get_one::<String>("url") {
                    let plugin_name = clone_plugin(link, &mut plugin_dir, force);
                    println!("Added {plugin_name}.");
                } else {
                    let mov = add_args.get_one::<bool>("move").unwrap_or_else(|| {
                        eprintln!("{}", error("Path is invalid."));
                        std::process::exit(1)
                    });
                    let src = PathBuf::from(add_args.get_one::<String>("path").unwrap());
                    if !src.exists() {
                        eprintln!("{}", error("Path not exists."));
                        std::process::exit(1)
                    }
                    let plugin_name = clone_local_plugin(&src, &mut plugin_dir, force, *mov);
                    if *mov {
                        _ = fs::remove_dir(&src);
                    }
                    println!("Added {plugin_name}.");
                }
            }
            Some(("remove", remove_args)) => {
                let mut plugin_dir = path;
                let name = remove_args.get_one::<String>("name").unwrap();
                if name.contains("/")
                    || name.contains(".")
                    || name.contains("~")
                    || name.contains("\\")
                {
                    eprintln!("{}", error("Please provide a valid value"));
                    std::process::exit(1);
                }
                if *remove_args.get_one::<bool>("lib").unwrap() {
                    plugin_dir.push("libs");
                } else {
                    plugin_dir.push("plugins");
                }
                let mut yes = *remove_args.get_one::<bool>("yes").unwrap();
                if !yes {
                    print!("Do you really wish to remove {name} (Y/N)? ");
                    std::io::stdout().flush().unwrap();
                    let mut answer: String = String::new();
                    std::io::stdin().read_line(&mut answer).unwrap_or_else(|e| {
                        eprintln!("{}", error(&format!("An error has been accurd: {e}")));
                        std::process::exit(1);
                    });
                    yes = "yes".starts_with(&answer.trim().to_lowercase())
                }
                if !yes {
                    eprintln!("{}", error("Remove has been canceled."));
                    std::process::exit(1);
                }
                plugin_dir.push(name);
                if plugin_dir.exists() {
                    std::fs::remove_dir_all(plugin_dir).unwrap_or_else(|e| {
                        eprintln!("{}", error(&format!("Faild to remove plugin {name}\n{e}")));
                        std::process::exit(1)
                    });
                    println!("{}", success(&format!("Removed {name}.")));
                } else {
                    eprintln!(
                        "{}",
                        error(&format!("Faild to remove plugin {name}, {name} not exists"))
                    );
                    std::process::exit(1)
                }
            }
            Some(("list", list_args)) => {
                let lib = *list_args.get_one::<bool>("lib").unwrap();
                let plugin = list_args.get_one::<bool>("plugin").unwrap();
                if !lib && !plugin {
                    list(&path, false);
                    list(&path, true);
                } else {
                    list(&path, lib);
                }
            }
            _ => lua::interpreter::start_interpreter(&lua_ctx).expect("Failed to run interpreter."),
        }
    }
}
