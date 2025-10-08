use clap::builder;
use clap::{command, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use git2::FetchOptions;
use log::{error, info, warn, LevelFilter};
use mlua::Lua;
use quick_xml::events::{BytesDecl, BytesText, Event};
use quick_xml::writer::Writer;
use quick_xml::Reader;
use regex::Regex;
use std::convert::TryFrom;
use std::default::Default;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use which::which;
use yrnu::config::{self, connect, SSHAuthType};
use yrnu::core::{Interface, IpAddress, MacAddress, Mask, Network, Path, Url};
use yrnu::lua;
use yrnu::lua::interpreter;
use yrnu::parser::*;

/// The global yrnu
#[derive(Debug, Clone, Default)]
struct Yrnu {
    lua: Lua,      // main Lua context
    root: PathBuf, // Config directory Path
    // Yrnu plugins
    plugins: Vec<(
        String,                     // Plugin name
        Vec<(String, mlua::Table)>, // Plugin globals
        bool,                       // is_cli (would to include CLI bindings or not)
        bool,                       // is_api (would to include Lua bindings or not)
        String,                     // description
    )>,
    args: Command, // Yrnu clap definition
    install_cmd: String,
}

impl Yrnu {
    // Handle the creation of global argument Lua function
    fn get_arg_lua_function(
        &self,
        arg_name: String,
        arg_options: &mlua::Table,
    ) -> mlua::Result<mlua::Function> {
        let update = arg_options.get::<mlua::Function>("update");
        let arg_type = arg_options.get::<String>("arg_type").unwrap_or_default();
        let func =
            if arg_type == "nil" {
                return Err(mlua::Error::RuntimeError(
                    "Arguments with arg_type = \"nil\" have to implement update function"
                        .to_string(),
                ));
            } else {
                let action = arg_options.get::<String>("action").unwrap_or_default();
                if action == "store-table" {
                    self.lua.create_function(
                        move |_, (this, value): (mlua::Table, mlua::Table)| {
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
                        },
                    )?
                } else if action == "store-true" || action == "store-false" {
                    self.lua
                        .create_function(move |_, (this, value): (mlua::Table, bool)| {
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
                } else {
                    match arg_options // will be defined if action is not equals to the previous
                        // values or doesn't defined (because *)
                        .get::<String>("arg_type")
                        .unwrap_or_default()
                        .as_str()
                    {
                        "bool" | "boolish" => self.lua.create_function(
                            move |_, (this, value): (mlua::Table, bool)| {
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
                            },
                        )?,
                        "int" => self.lua.create_function(
                            move |_, (this, value): (mlua::Table, i64)| {
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
                            },
                        )?,
                        "uint" => self.lua.create_function(
                            move |_, (this, value): (mlua::Table, u64)| {
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
                            },
                        )?,
                        "real" => self.lua.create_function(
                            move |_, (this, value): (mlua::Table, f64)| {
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
                            },
                        )?,
                        "ip-address" => self.lua.create_function(
                            move |_, (this, value): (mlua::Table, IpAddress)| {
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
                            },
                        )?,
                        "network" => self.lua.create_function(
                            move |_, (this, value): (mlua::Table, Network)| {
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
                            },
                        )?,
                        "mask" => self.lua.create_function(
                            move |_, (this, value): (mlua::Table, Mask)| {
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
                            },
                        )?,
                        "mac-address" => self.lua.create_function(
                            move |_, (this, value): (mlua::Table, MacAddress)| {
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
                            },
                        )?,
                        "interface" => self.lua.create_function(
                            move |_, (this, value): (mlua::Table, Interface)| {
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
                            },
                        )?,
                        "path" => self.lua.create_function(
                            move |_, (this, value): (mlua::Table, Path)| {
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
                            },
                        )?,
                        "url" => self.lua.create_function(
                            move |_, (this, value): (mlua::Table, Url)| {
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
                            },
                        )?,
                        _ => self.lua.create_function(
                            move |_, (this, value): (mlua::Table, mlua::Value)| {
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
                            },
                        )?,
                    }
                }
            };
        Ok(func)
    }
    // Handle the plugin argument CLI creation
    fn get_arg_clap_cmd(
        &self,
        (arg_name, arg_options): (&String, &mlua::Table),
    ) -> mlua::Result<Arg> {
        // Creating Clap argument
        let mut arg = Arg::new(arg_name);
        // Going over each defined argument property and define it in clap
        if let Ok(required) = arg_options.get::<bool>("required") {
            if required {
                arg = arg.required_unless_present("wizard");
            }
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
                arg = arg.long(arg_name.replace("_", "-"));
            } else {
                arg = arg.long(long);
            }
        } else {
            arg = arg.long(arg_name.replace("_", "-"));
        }
        if let Ok(help) = arg_options.get::<String>("help") {
            arg = arg.help(help);
        }
        if let Ok(values) = arg_options.get::<Vec<String>>("possible_values") {
            arg = arg.value_parser(values.to_owned());
        }
        arg = match arg_options
            .get::<String>("action")
            .unwrap_or_default() // *
            .as_str()
        {
            "store-true" => arg.action(ArgAction::SetTrue),
            "store-false" => arg.action(ArgAction::SetFalse),
            "store-count" => arg.action(ArgAction::Count),
            action => {
                let arg = match action {
                    "store-table" => {
                        if let Ok(delim) = arg_options.get::<String>("delimiter") {
                            if delim.len() == 1 && delim != " " {
                                arg.value_delimiter(delim.chars().next())
                            } else {
                                arg.action(ArgAction::Append)
                            }
                        } else {
                            arg.action(ArgAction::Append)
                        }
                    }
                    "store-count" => arg.action(ArgAction::Count),
                    _ => arg,
                };
                match arg_options // will be defined if action is not equals to the previous
                    // values or doesn't defined (because *)
                    .get::<String>("arg_type")
                    .unwrap_or_default()
                    .as_str()
                {
                    "bool" => arg.value_parser(builder::BoolValueParser::new()),
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
                    "ip-address" => arg.value_parser(IpAddress::from_str),
                    "network" => arg.value_parser(Network::from_str),
                    "mask" => arg.value_parser(Mask::from_str),
                    "mac-address" => arg.value_parser(MacAddress::from_str),
                    "interface" => arg.value_parser(Interface::from_str),
                    "path" => arg.value_parser(Path::from_str),
                    "url" => arg.value_parser(Url::from_str),
                    _ => arg,
                }
            }
        };
        Ok(arg)
    }
    // Loads a specific global
    fn load_global(
        &self,
        name: &String,       // global name
        table: &mlua::Table, // global table
        plugin_name: &String,
        is_cli: bool, // Whether to define Clap argument for this global
        is_api: bool, // Whether to define a Lua function for this global
    ) -> mlua::Result<(Option<Command>, Option<mlua::Function>)> {
        // Returns the defined Clap
        // Argument and Lua function if defined
        let start_config = table.get("preconfig").unwrap_or("".to_string()); // Text to append before
                                                                             // the configuration string
        let end_config = table.get("postconfig").unwrap_or("".to_string()); // Text to append after
                                                                            // the configuration string
                                                                            // The global Clap definition
        let mut subcmd = Command::new(name).arg(
            Arg::new("wizard")
                .short('w')
                .long("wizard")
                .exclusive(true)
                .help("Config using a wizard by asking for each argument one by one")
                .action(ArgAction::SetTrue),
        );
        // Reading the global table
        let args = table.get::<mlua::Table>("args");
        let subcommands = table.get::<mlua::Table>("subcommands");
        // If about is define add it as the command about
        if let Ok(about) = table.get::<String>("about") {
            subcmd = subcmd.about(about);
        }
        // Creating the global table
        let global_table = self.lua.create_table()?;
        // Getting the config method
        let config_func = table.get::<mlua::Function>("config")?;
        // Creating a wrapper for the config method that adds the pre and post config
        let config_func = self.lua.create_function(move |_, this: mlua::Table| {
            let mut conf = format!("{start_config}");
            conf = conf + &config_func.call::<String>(this)?;
            conf = conf + &end_config;
            Ok(conf.trim().to_owned())
        })?;
        // Adds the config wrapper to the global table as "config" and as the "__tostring" methods
        global_table.set("config", config_func.clone())?;
        let metatable = self.lua.create_table()?;
        let tostring = self.lua.create_function(move |_, this: mlua::Table| {
            match config_func.call::<String>(this) {
                Ok(config_str) => Ok(config_str),
                Err(config_err) => Ok(format!("Error: {config_err}")),
            }
        })?;
        metatable.set("__tostring", tostring)?;
        global_table.set_metatable(Some(metatable));
        // Argument handling
        let mut arg;
        let mut func;
        let mut arg_name;
        let mut arg_opts;
        let mut arg_values;
        // Iterating over the global arguments table
        if let Ok(args) = args {
            for pair in args.pairs::<String, mlua::Value>() {
                (arg_name, arg_values) = pair?;
                arg_opts = match arg_values {
                    mlua::Value::Table(value) => value, // If the argument value is a table, return it.
                    mlua::Value::String(value) => {
                        let table = self.lua.create_table()?; // Create an argument table
                                                              // If the argument value is a string check
                        if value.to_str().unwrap().len() == 1 {
                            // If the string length is 1 use
                            // it as a short Clap argument
                            table.set("short", value)?;
                        } else {
                            // Else use it as a help message
                            table.set("help", value)?;
                        }
                        table // Return the newly created argument table
                    }
                    _ => {
                        // For other types just skip this argument and log an error
                        error!(
                        "Faild to load argument {}!, value should be either a table, string or nil",
                        arg_name
                    );
                        continue;
                    }
                };
                // For each argument get the clap argument and Lua function and added them to the
                // command or table
                arg = self.get_arg_clap_cmd((&arg_name, &arg_opts))?;
                subcmd = subcmd.arg(arg);
                func = self.get_arg_lua_function(arg_name.clone(), &arg_opts)?;
                global_table.set(format!("set_{}", arg_name), func)?;
            }
        } else {
            info!("Didn't found argument table. Skiping argument setup...")
        }
        // Subcommand handling
        let mut scmd_name;
        let mut scmd_table;
        if let Ok(subcommands) = subcommands {
            // Iterating over the global subcommand table
            for pair in subcommands.pairs::<String, mlua::Table>() {
                (scmd_name, scmd_table) = pair?;
                let name_clone = scmd_name.clone(); // Used for the subcommand constructor wrapper (needs an owned type)
                if let Ok((scmd, constructor)) =
                    // Recursion
                    self.load_global(&scmd_name, &scmd_table, &plugin_name, is_cli, is_api)
                {
                    if let Some(constructor) = constructor {
                        // Creating a wrapper function for the subcommand constructor
                        let construct = self.lua.create_function(
                            move |_, (this, param): (mlua::Table, Option<mlua::Table>)| {
                                this.set(
                                    name_clone.to_owned(),
                                    constructor.call::<Option<mlua::Table>>(param)?, // calling the
                                                                                     // constructor
                                )?;
                                Ok(())
                            },
                        )?;
                        // Adds the subcommand constructor to the global table
                        global_table.set(format!("add_{}", scmd_name), construct)?;
                    }
                    if let Some(scmd) = scmd {
                        // Adds the subcommand Clap command to the global command
                        subcmd = subcmd.subcommand(scmd);
                    }
                } else {
                    error!(
                        "Faild to load {} subcommand of {} plugin",
                        scmd_name, plugin_name
                    );
                }
            }
        } else {
            info!("Didn't found subcommands table. Skiping subcommand setup...")
        }
        // Check if init function is defined in the global config
        let init_func = if let Ok(init) = table.get::<mlua::Function>("init") {
            // Creating an init function wrapper for the global table
            self.lua
                .create_function(move |_, param: Option<mlua::Table>| {
                    init.call::<Option<mlua::Table>>((global_table.clone(), param))
                })
        } else {
            // Creating an init function that returns a new global table
            self.lua
                .create_function(move |_, ()| Ok(global_table.clone()))
        };
        Ok((
            if is_cli { Some(subcmd) } else { None },
            if is_cli { Some(init_func?) } else { None },
        ))
    }
    // Reads a single plugin and returns its list of globals and a __tostring function
    fn read_plugin(
        &self,
        plugin: &String, // The plugin name
    ) -> mlua::Result<(Vec<(String, mlua::Table)>, Option<String>)> {
        let plugin_path = self.root.join(format!("plugins/{plugin}"));
        //let plugin_name = plugin.split("-").last().unwrap().to_string();
        // Check if plugin directory exists
        if plugin_path.is_dir() {
            // Creating a default __tostring function
            let mut to_string = String::new();
            let init_file = plugin_path.join("init.lua");
            // Check if the plugin contains an init.lua file
            if init_file.is_file() {
                // Reading the init.lua file
                let init = self
                    .lua
                    .load(std::fs::read_to_string(init_file)?)
                    .eval::<mlua::Table>();
                if let Ok(init_table) = init {
                    // Reading the plugin directory
                    let entries = plugin_path.read_dir();
                    // Checking if __tostring text is defined
                    if let Ok(tostring) = init_table.get::<String>("description") {
                        to_string = tostring;
                    };
                    if let Ok(dependencies) = init_table.get::<mlua::Table>("dependencies") {
                        if let (Ok(false), Ok(false)) = (
                            dependencies.contains_key("programs"),
                            dependencies.contains_key("plugins"),
                        ) {
                            if let Ok(programs) = init_table.get::<Vec<String>>("dependencies") {
                                let (shell, exec_flag) = if std::env::consts::OS == "windows" {
                                    ("cmd", "/C")
                                } else {
                                    ("sh", "-c")
                                };
                                info!("Installing dependencies...");
                                for program in programs {
                                    if let Ok(_) = which::which(&program) {
                                        info!("[*] {program} is already install. Skipping...");
                                        continue;
                                    } else {
                                        println!("[*] Installing {}...", program);
                                        let status = std::process::Command::new(&shell)
                                            .arg(exec_flag)
                                            .arg(format!("{} {}", self.install_cmd, program))
                                            .status();
                                        if let Ok(status) = status {
                                            if status.success() {
                                                println!("{program} is successfully installed");
                                                info!("{program} is successfully installed");
                                            } else {
                                                eprintln!("Faild installing {program}");
                                                error!("Faild installing {program}");
                                                return Err(mlua::Error::external(
                                                    "Faild installing dependencies",
                                                ));
                                            }
                                        } else {
                                            let err = status.err().unwrap();
                                            eprintln!("Faild installing {program}.\nError: {err}");
                                            error!("Faild installing {program}.\nError: {err}");
                                            return Err(mlua::Error::external(err));
                                        }
                                    }
                                }
                            }
                        } else {
                            if let Ok(programs) = dependencies.get::<Vec<String>>("programs") {
                                let (shell, exec_flag) = if std::env::consts::OS == "windows" {
                                    ("cmd", "/C")
                                } else {
                                    ("sh", "-c")
                                };
                                info!("Installing dependencies...");
                                for program in programs {
                                    if let Ok(_) = which::which(&program) {
                                        info!("[*] {program} is already install. Skipping...");
                                        continue;
                                    } else {
                                        println!("[*] Installing {}...", program);
                                        let status = std::process::Command::new(&shell)
                                            .arg(exec_flag)
                                            .arg(format!("{} {}", self.install_cmd, program))
                                            .status();
                                        if let Ok(status) = status {
                                            if status.success() {
                                                println!("{program} is successfully installed");
                                                info!("{program} is successfully installed");
                                            } else {
                                                eprintln!("Faild installing {program}");
                                                error!("Faild installing {program}");
                                                return Err(mlua::Error::external(
                                                    "Faild installing dependencies",
                                                ));
                                            }
                                        } else {
                                            let err = status.err().unwrap();
                                            eprintln!("Faild installing {program}.\nError: {err}");
                                            error!("Faild installing {program}.\nError: {err}");
                                            return Err(mlua::Error::external(err));
                                        }
                                    }
                                }
                            }
                            if let Ok(plugins) = dependencies.get::<mlua::Table>("plugins") {
                                let mut username;
                                let mut repo;
                                let mut inner_username;
                                let mut inner_repo;
                                let mut plugins_path = self.root.clone();
                                plugins_path.push("plugins");
                                for pair in plugins.pairs::<mlua::Value, mlua::Value>() {
                                    (username, repo) = pair?;
                                    match (username, repo) {
                                        (mlua::Value::String(value), mlua::Value::Table(table)) => {
                                            for link in table.pairs::<mlua::Value, String>() {
                                                (_, inner_repo) = link?;
                                                inner_username = value.to_str().unwrap();
                                                if self
                                                    .root
                                                    .join(format!(
                                                        "plugins/{inner_username}-{inner_repo}"
                                                    ))
                                                    .exists()
                                                {
                                                    continue;
                                                }
                                                inner_repo =
                                                    format!("{inner_username}/{inner_repo}");
                                                Yrnu::clone_plugin(
                                                    &inner_repo,
                                                    &mut plugins_path,
                                                    false,
                                                    true,
                                                );
                                            }
                                        }
                                        (
                                            mlua::Value::String(username),
                                            mlua::Value::String(repo),
                                        ) => {
                                            inner_username = username.to_str().unwrap();
                                            let inner_repo = repo.to_str().unwrap();
                                            if self
                                                .root
                                                .join(format!(
                                                    "plugins/{inner_username}-{inner_repo}"
                                                ))
                                                .exists()
                                            {
                                                continue;
                                            }
                                            Yrnu::clone_plugin(
                                                &format!("{inner_username}/{inner_repo}"),
                                                &mut plugins_path,
                                                false,
                                                true,
                                            );
                                        }
                                        (_, mlua::Value::String(link)) => {
                                            if self
                                                .root
                                                .join(format!(
                                                    "plugins/{}",
                                                    link.to_string_lossy().replace("/", "-")
                                                ))
                                                .exists()
                                            {
                                                continue;
                                            }
                                            Yrnu::clone_plugin(
                                                &link.to_str().unwrap(),
                                                &mut plugins_path,
                                                false,
                                                true,
                                            );
                                        }
                                        _ => continue,
                                    }
                                }
                            }
                        }
                    }
                    // Check if public or private globals list is defined
                    if let Ok(public) = init_table.get::<mlua::Table>("public") {
                        let mut globals = vec![];
                        let plugin_path = self.root.join(format!("plugins/{plugin}"));
                        let mut global_name;
                        // Iterating over the plugins and including only those that been
                        // declared as public
                        for global in public.sequence_values::<String>() {
                            global_name = global?;
                            // Checking if this global file exists
                            if plugin_path.join(format!("{global_name}.lua")).exists() {
                                // If successfully evaluated as Lua table add this global to the
                                // list
                                match self
                                    .lua
                                    .load(std::fs::read_to_string(
                                        plugin_path.join(format!("{global_name}.lua")),
                                    )?)
                                    .eval::<mlua::Table>()
                                {
                                    Ok(global_table) => {
                                            globals.push((global_name, global_table))},
                                    Err(e) => eprintln!("Failed to evaluate {global_name}.lua as a vaild lua table.\nError: {e}"),
                                }
                            } else {
                                eprintln!("{global_name}.lua do not exists.\nSkipping...");
                            }
                        }
                        Ok((globals, Some(to_string)))
                    } else if let Ok(private) = init_table.get::<mlua::Table>("private") {
                        if let Err(e) = entries {
                            eprintln!("Failed to read {plugin} directory.\nError: {e}.");
                            Ok((vec![], None))
                        } else {
                            let mut globals = vec![];
                            let mut path;
                            // Iterating over the entries in the plugin directory
                            for entry in entries.unwrap() {
                                if let Ok(entry) = entry {
                                    if entry.path().is_file() {
                                        // Checking if the current file is in not in the private
                                        // list
                                        if !private.sequence_values::<String>().any(|v| {
                                            v.is_ok()
                                                && entry.file_name().into_string().is_ok()
                                                && format!("{}.lua", v.unwrap())
                                                    == entry.file_name().into_string().unwrap()
                                        }) {
                                            path = entry.path();
                                            // Checking that the file is a Lua file and not the init file
                                            if path.extension().is_some()
                                                && path.extension().unwrap() == "lua"
                                                && path.file_name().is_some()
                                                && path.file_name().unwrap() != "init.lua"
                                            {
                                                // Adding the global to the globals list if the global file can be evaluated as
                                                // Lua table
                                                match self
                                                    .lua
                                                    .load(std::fs::read_to_string(entry.path())?)
                                                    .eval::<mlua::Table>()
                                                {
                                                    Ok(global_table) => {
                                                            let name = path.file_stem().unwrap_or_default().display().to_string();
                                                            globals.push((name, global_table))},
                                                    Err(e) => eprintln!("Failed to evaluate {}.lua as a vaild lua table.\nError: {e}", entry.file_name().display()),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Ok((globals, Some(to_string)))
                        }
                    } else {
                        let mut globals = vec![];
                        let mut path;
                        // Iterating over the plugin directory entries
                        for entry in entries.unwrap() {
                            if let Ok(entry) = entry {
                                path = entry.path();
                                // If the current entry is a Lua file and not the init file
                                if path.extension().is_some()
                                    && path.extension().unwrap() == "lua"
                                    && path.file_name().is_some()
                                    && path.file_name().unwrap() != "init.lua"
                                {
                                    // Adding the global to the globals list if the global file can be evaluated as
                                    // Lua table
                                    match self
                                    .lua
                                    .load(std::fs::read_to_string(entry.path())?)
                                    .eval::<mlua::Table>()
                                    {
                                        Ok(global_table) => {
                                                let name = path.file_stem().unwrap_or_default().display().to_string();
                                                globals.push((name, global_table))},
                                        Err(e) => eprintln!("Failed to evaluate {}.lua as a vaild lua table.\nError: {e}", entry.file_name().display()),
                                    }
                                }
                            }
                        }
                        Ok((globals, Some(to_string)))
                    }
                } else {
                    eprintln!("Found init.lua file in {plugin} plugin but couldn't evaluate it as a table\nSkiping...");
                    Ok((vec![], None))
                }
            } else {
                let entries = plugin_path.read_dir();
                if let Err(e) = entries {
                    eprintln!("Failed to read {plugin} directory.\nError: {e}.");
                    Ok((vec![], None))
                } else {
                    let mut globals = vec![];
                    let mut path;
                    // Iterating over the plugin directory entries
                    for entry in entries.unwrap() {
                        if let Ok(entry) = entry {
                            path = entry.path();
                            // Checking that the file is a Lua file and not the init file
                            if path.extension().is_some()
                                && path.extension().unwrap() == "lua"
                                && path.file_name().is_some()
                                && path.file_name().unwrap() != "init.lua"
                            {
                                // Adding the global to the globals list if the global file can be evaluated as
                                // Lua table
                                match self
                                    .lua
                                    .load(std::fs::read_to_string(entry.path())?)
                                    .eval::<mlua::Table>()
                                {
                                    Ok(global_table) => {
                                        let name = path
                                            .file_stem()
                                            .unwrap_or_default()
                                            .display()
                                            .to_string();
                                        globals.push((name, global_table))
                                    }
                                    Err(e) => eprintln!(
                                        "Failed to evaluate {} as a vaild lua table.\nError: {e}",
                                        entry.file_name().display()
                                    ),
                                }
                            }
                        }
                    }
                    Ok((globals, Some(to_string)))
                }
            }
        } else {
            eprintln!(
                "Failed to load plugin {plugin}.Error: {} not exists.",
                plugin_path.display()
            );
            Ok((vec![], None))
        }
    }
    // Read plugins from the disk
    fn read_plugins(&mut self) -> mlua::Result<()> {
        let file = self.root.join("init.lua");
        if file.is_file() {
            // Adding the plugin directory to the Lua path
            self.lua
                .load(format!(
                    "package.path = package.path .. \";{}/?.lua\"",
                    self.root.display()
                ))
                .exec()?;
            // Reading the init file as a table
            let init = self
                .lua
                .load(std::fs::read_to_string(file)?)
                .eval::<mlua::Table>()?;
            // Handling the init file options
            let ensure_installed = init
                .get::<mlua::Table>("ensure_installed")
                .unwrap_or(self.lua.create_table()?);
            let install_cmd = init.get::<Option<String>>("install_cmd").unwrap_or(None);
            let install_cmd = if install_cmd.is_none() {
                match std::env::consts::OS {
                    "linux" => {
                        if which("apt").is_ok() {
                            Some(String::from("apt install"))
                        } else if which("apt-get").is_ok() {
                            Some(String::from("apt-get install"))
                        } else if which("dnf").is_ok() {
                            Some(String::from("dnf install"))
                        } else if which("pacman").is_ok() {
                            Some(String::from("pacman -Sy"))
                        } else {
                            None
                        }
                    }
                    "freebsd" | "openbsd" | "netbsd" => {
                        if which("pkg").is_ok() {
                            Some(String::from("pkg install"))
                        } else {
                            None
                        }
                    }
                    "macos" => {
                        if which("brew").is_ok() {
                            Some(String::from("brew install"))
                        } else {
                            None
                        }
                    }
                    "windows" => {
                        if which("winget").is_ok() {
                            Some(String::from("winget install"))
                        } else if which("choco").is_ok() {
                            Some(String::from("choco install"))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                install_cmd
            };
            if install_cmd.is_none() {
                eprintln!("Error: Couldn't determined your install command (e.g pacman -S in Arch based distros), please specify it manualy in your init.lua file.\n***install_cmd = <INSTALL CMD>***");
                return Err(mlua::Error::external(
                    "Required argument \"install_cmd\" is missing.",
                ));
            } else {
                self.install_cmd = install_cmd.unwrap();
            }
            if !ensure_installed.is_empty() {
                let mut username;
                let mut repo;
                let mut inner_repo;
                let mut inner_username;
                let mut plugins_path = self.root.clone();
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
                                if self
                                    .root
                                    .join(format!("plugins/{inner_username}-{inner_repo}"))
                                    .exists()
                                {
                                    continue;
                                }
                                inner_repo = format!("{inner_username}/{inner_repo}");
                                Yrnu::clone_plugin(&inner_repo, &mut plugins_path, false, true);
                            }
                        }
                        (mlua::Value::String(username), mlua::Value::String(repo)) => {
                            inner_username = username.to_str().unwrap();
                            let inner_repo = repo.to_str().unwrap();
                            if self
                                .root
                                .join(format!("plugins/{inner_username}-{inner_repo}"))
                                .exists()
                            {
                                continue;
                            }
                            Yrnu::clone_plugin(
                                &format!("{inner_username}/{inner_repo}"),
                                &mut plugins_path,
                                false,
                                true,
                            );
                        }
                        (_, mlua::Value::String(link)) => {
                            if self
                                .root
                                .join(format!(
                                    "plugins/{}",
                                    link.to_string_lossy().replace("/", "-")
                                ))
                                .exists()
                            {
                                continue;
                            }
                            Yrnu::clone_plugin(
                                &link.to_str().unwrap(),
                                &mut plugins_path,
                                false,
                                true,
                            );
                        }
                        _ => continue,
                    }
                }
            }
            let plugins = init
                .get::<mlua::Table>("plugins")
                .unwrap_or(self.lua.create_table()?);
            if plugins.is_empty() {
                Ok(())
            } else {
                let cli_table = plugins
                    .get::<mlua::Table>("cli")
                    .unwrap_or(self.lua.create_table()?);
                let api_table = plugins
                    .get::<mlua::Table>("api")
                    .unwrap_or(self.lua.create_table()?);
                let mut is_cli = true;
                let mut is_api = true;
                let mut plugin_table;
                let mut plugin_to_string;
                for plugin in plugins.sequence_values::<String>() {
                    if let Ok(plugin) = plugin {
                        if cli_table
                            .sequence_values::<String>()
                            .any(|v| v.unwrap_or_default() == plugin)
                            && !api_table
                                .sequence_values::<String>()
                                .any(|v| v.unwrap_or_default() == plugin)
                        {
                            is_api = false;
                        }
                        if !cli_table
                            .sequence_values::<String>()
                            .any(|v| v.unwrap_or_default() == plugin)
                            && api_table
                                .sequence_values::<String>()
                                .any(|v| v.unwrap_or_default() == plugin)
                        {
                            is_cli = false;
                        }
                        let plugin_name = plugin.clone();
                        (plugin_table, plugin_to_string) = match self.read_plugin(&plugin)? {
                            (table, Some(plugin_to_string)) => (table, plugin_to_string),
                            (table, _) => (table, plugin_name),
                        };
                        self.plugins
                            .push((plugin, plugin_table, is_cli, is_api, plugin_to_string));
                    }
                }
                Ok(())
            }
        } else {
            // Creating the file with default configuration
            if let Err(e) = fs::write(
                &file,
                "return {
    ensure_installed = {},
    plugins = {},
    globals = {},
}",
            ) {
                eprintln!("Failed to create {}: {e}", file.display());
                std::process::exit(1);
            }
            Ok(())
        }
    }
    // Define the cli usage of the program
    fn define_cli_usage() -> Command {
        command!()
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
        )
    }
    // Loads the user plugins (uses load_plugin under the hood)
    pub fn load_plugins(mut self) -> mlua::Result<Self> {
        match self.read_plugins() {
            Err(e) => {
                error!("Faild to read plugins: {}", e);
                return Err(e);
            }
            _ => {
                info!("Successfuly read plugins")
            }
        }
        let mut is_err = false;
        let mut config_args = Command::new("config")
            .about("configure linux/Windows machines and network devices or any other thing that comes to mind.");
        let mut plugin_scmd;
        let mut plugin_name;
        let mut plugin_metatable;
        let mut plugin_desc;
        // Iterating over the plugin vector and initializing the plugins
        for plugin in self.plugins.clone() {
            // Removing the plugin origin
            plugin_name = plugin.0.split("-").last().unwrap().to_owned();
            let plugin_table = self.lua.create_table()?;
            plugin_metatable = self.lua.create_table()?;
            plugin_desc = plugin.4.clone();
            plugin_metatable.set(
                "__tostring",
                self.lua
                    .create_function(move |_, _this: mlua::Table| Ok(plugin_desc.clone()))?,
            )?;
            plugin_table.set_metatable(Some(plugin_metatable));
            plugin_scmd = Command::new(&plugin_name)
                .arg_required_else_help(true)
                .about(plugin.4);
            for global in &plugin.1 {
                info!("reading {}", global.0);
                match Self::load_global(&self, &global.0, &global.1, &plugin.0, plugin.2, plugin.3)
                {
                    Ok((cmd, func)) => {
                        if let Some(cmd) = cmd {
                            plugin_scmd = plugin_scmd.subcommand(cmd);
                        }
                        if let Some(func) = func {
                            plugin_table.set(global.0.to_owned(), func)?;
                        }
                    }
                    Err(e) => {
                        error!("Faild to load {}: {e}", plugin.0);
                        is_err = true
                    }
                }
            }
            if plugin.2 {
                config_args = config_args.subcommand(plugin_scmd);
            }
            if plugin.3 {
                self.lua
                    .globals()
                    .set(plugin_name.to_owned(), plugin_table)?;
            }
        }
        if is_err {
            warn!("There is plugins that failed to load")
        }
        self.args = self.args.subcommand(config_args);
        Ok(self)
    }
    // Initiate a new Yrnu instance
    pub fn new(level: Option<log::LevelFilter>) -> Result<Self, Box<dyn std::error::Error>> {
        let debug = std::env::var("YRNU_DEBUG") == Ok(String::from("1"));
        let mut path = match std::env::var("YRNU_CONFIG_DIR") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                if let Some(path) = dirs::config_dir() {
                    path
                } else {
                    if let Some(path) = dirs::home_dir() {
                        path
                    } else {
                        PathBuf::from("/")
                    }
                }
            }
        };
        path.push("yrnu");
        if !path.is_dir() {
            _ = std::fs::create_dir_all(path.clone());
        }
        let lua = lua::init();
        if lua.is_err() {
            eprintln!("Faild to initiate Lua context.");
            std::process::exit(-1);
        }
        if let Some(level) = level {
            if debug {
                simple_logging::log_to_stderr(level);
            } else {
                simple_logging::log_to_file(&path.join("yrnu.log"), level)
                    .unwrap_or_else(|e| error!("{e}"));
            }
        } else {
            if debug {
                simple_logging::log_to_stderr(LevelFilter::Info);
            } else {
                simple_logging::log_to_file(&path.join("yrnu.log"), LevelFilter::Info)
                    .unwrap_or_else(|e| error!("{e}"));
            }
        };
        let args = Self::define_cli_usage();
        let mut yrnu = Self {
            lua: lua.unwrap(),
            root: path,
            args: args.clone(),
            ..Default::default()
        };
        yrnu = yrnu.load_plugins()?;
        let yrnu = yrnu.lua_setup()?;
        Ok(yrnu)
    }
    /// Handle the cli usage of the plugins
    pub fn handle_cli_matches(
        &self,
        arg_matches: &ArgMatches,
        plugin: (String, &mlua::Table),
        wizard: bool,
    ) -> mlua::Result<String> {
        let (_, table) = plugin;
        let config_table = self.lua.create_table().unwrap_or_else(|_| {
            error!("Something went wrong...");
            std::process::exit(10)
        });
        let mut input = String::new();
        let _yes = String::from("yes");
        let _true = String::from("true");
        let _on = String::from("on");
        let _no = String::from("no");
        let _false = String::from("false");
        let _off = String::from("off");
        let args_table = table.get::<mlua::Table>("args");
        if let Ok(args_table) = args_table {
            let mut prompt;
            let mut required;
            let mut args = vec![];
            let mut args_sorted = vec![];
            let mut arg_name;
            let mut arg_opts;
            let mut arg_values;
            // Sorting the args by the index
            for pair in args_table.pairs::<String, mlua::Value>() {
                (arg_name, arg_values) = pair.unwrap_or_else(|_| {
                    error!("Invalid argument definition of args");
                    std::process::exit(11)
                });
                if let mlua::Value::Table(arg_values) = arg_values {
                    if let Ok(index) = arg_values.get::<u32>("index") {
                        args_sorted.push((arg_name, mlua::Value::Table(arg_values), index))
                    } else {
                        args.push((arg_name, mlua::Value::Table(arg_values)))
                    }
                } else {
                    args.push((arg_name, arg_values))
                }
            }
            args_sorted.sort_by(|a, b| a.2.cmp(&b.2));
            let mut args_sorted = args_sorted
                .into_iter()
                .map(|v| (v.0, v.1))
                .collect::<Vec<(String, mlua::Value)>>();
            args_sorted.append(&mut args);
            for (arg_name, arg_values) in args_sorted {
                arg_opts = match arg_values {
                    mlua::Value::Table(value) => value,
                    mlua::Value::String(value) => {
                        let table = self.lua.create_table()?;
                        if value.to_str().unwrap().len() == 1 {
                            table.set("short", value)?;
                        } else {
                            table.set("help", value)?;
                        }
                        table
                    }
                    _ => {
                        error!("Faild to load argument {}!, value should be either a table or a string", arg_name);
                        continue;
                    }
                };
                required = arg_opts.get::<bool>("required").unwrap_or(false);
                prompt = arg_opts.get::<String>("wizard").unwrap_or(arg_name.clone());
                let name = arg_name.clone();
                let update =
                    arg_opts
                        .get::<mlua::Function>("update")
                        .unwrap_or(self.lua.create_function(
                            move |_, (this, value): (mlua::Table, mlua::Value)| {
                                _ = this.set(name.to_owned(), value);
                                Ok(())
                            },
                        )?);
                if wizard {
                    let mut is_err = false;
                    let mut correct_type;
                    loop {
                        if is_err {
                            println!("{arg_name} is required.");
                        }
                        input.clear();
                        print!("{prompt}: ");
                        std::io::stdout().flush().unwrap_or_else(|e| {
                            eprintln!("Something went bad!\nError: {e}");
                        });
                        std::io::stdin().read_line(&mut input).unwrap_or_else(|e| {
                            eprintln!("Something went bad!\nError: {e}");
                            1
                        });
                        input = input.trim().to_owned();
                        correct_type = match arg_opts
                            .get::<String>("arg_type")
                            .unwrap_or_default()
                            .as_str()
                        {
                            "bool" => _true.contains(&input) || _false.contains(&input),
                            "boolish" => {
                                _true.contains(&input)
                                    || _false.contains(&input)
                                    || _on.contains(&input)
                                    || _off.contains(&input)
                                    || _yes.contains(&input)
                                    || _no.contains(&input)
                            }
                            "int" => input.parse::<i64>().is_ok(),
                            "uint" => input.parse::<u64>().is_ok(),
                            "real" => input.parse::<f64>().is_ok(),
                            "ip-address" => IpAddress::is_valid(&input),
                            "network" => Network::from_str(&input).is_ok(),
                            "mask" => IpAddress::from_str(&input).is_ok(),
                            "mac-address" => MacAddress::is_valid(&input),
                            "interface" => Interface::from_str(&input).is_ok(),
                            "path" => Path::from_str(&input).is_ok(),
                            "url" => Url::from_str(&input).is_ok(),
                            _ => true,
                        };
                        if !required || (!input.trim().is_empty() && correct_type) {
                            break;
                        } else {
                            is_err = true;
                        }
                    }
                    if input.is_empty() {
                        continue;
                    }
                }
                let arg_action = arg_opts.get::<String>("action").unwrap_or_default();
                let delimeter = arg_opts
                    .get::<String>("delimeter")
                    .unwrap_or(String::from(" "));
                match arg_opts
                    .get::<String>("arg_type")
                    .unwrap_or_default()
                    .as_str()
                {
                    "bool" | "boolish" => {
                        if arg_action == "store-table" && wizard {
                            let values = input
                                .split(&delimeter)
                                .filter_map(|b| {
                                    if arg_opts.get::<String>("arg_type").unwrap_or_default()
                                        == "bool"
                                    {
                                        Some(_true.contains(b))
                                    } else {
                                        if String::from("true").contains(&input)
                                            || String::from("yes").contains(&input)
                                            || String::from("on").contains(&input)
                                        {
                                            Some(true)
                                        } else if String::from("false").contains(&input)
                                            || String::from("no").contains(&input)
                                            || String::from("off").contains(&input)
                                        {
                                            Some(false)
                                        } else {
                                            None
                                        }
                                    }
                                })
                                .collect::<Vec<bool>>();
                            _ = update
                                .call::<(mlua::Table, mlua::Table)>((config_table.clone(), values))
                        } else {
                            if arg_action == "store-table" {
                                if let Some(vals) = arg_matches.get_many::<bool>(&arg_name) {
                                    _ = update.call::<(mlua::Table, mlua::Table)>((
                                        config_table.clone(),
                                        vals.map(|v| v.to_owned()).collect::<Vec<bool>>(),
                                    ))
                                }
                            } else {
                                let value = if wizard {
                                    if arg_opts.get::<String>("arg_type").unwrap_or_default()
                                        == "bool"
                                    {
                                        Some(_true.contains(&input))
                                    } else {
                                        if _true.contains(&input)
                                            || _yes.contains(&input)
                                            || _on.contains(&input)
                                        {
                                            Some(true)
                                        } else if _false.contains(&input)
                                            || _no.contains(&input)
                                            || _off.contains(&input)
                                        {
                                            Some(false)
                                        } else {
                                            None
                                        }
                                    }
                                } else {
                                    if let Some(val) = arg_matches.get_one::<bool>(&arg_name) {
                                        Some(val.to_owned())
                                    } else {
                                        None
                                    }
                                };
                                if let Some(value) = value {
                                    _ = update.call::<(mlua::Table, bool)>((
                                        config_table.clone(),
                                        value.clone(),
                                    ))
                                }
                            }
                        }
                    }
                    "int" => {
                        if arg_action == "store-table" {
                            if let Some(vals) = arg_matches.get_many::<i64>(&arg_name) {
                                _ = update.call::<(mlua::Table, mlua::Table)>((
                                    config_table.clone(),
                                    vals.map(|v| v.to_owned()).collect::<Vec<i64>>(),
                                ))
                            }
                        } else {
                            let value = if wizard {
                                let mut num = input.trim().parse::<i64>();
                                if required {
                                    while num.is_err() {
                                        print!("\x1B[1A");
                                        print!("\x1B[2K");
                                        input.clear();
                                        print!("{prompt}: ");
                                        std::io::stdout().flush().unwrap_or_else(|e| {
                                            error!("Something went bad!\nError: {e}");
                                        });
                                        std::io::stdin().read_line(&mut input).unwrap_or_else(
                                            |e| {
                                                error!("Something went bad!\nError: {e}");
                                                1
                                            },
                                        );
                                        num = input.trim().parse::<i64>();
                                    }
                                }
                                if num.is_ok() {
                                    Some(num.unwrap())
                                } else {
                                    None
                                }
                            } else {
                                if let Some(val) = arg_matches.get_one::<i64>(&arg_name) {
                                    Some(val.to_owned())
                                } else {
                                    None
                                }
                            };
                            if let Some(value) = value {
                                _ = update.call::<(mlua::Table, mlua::Number)>((
                                    config_table.clone(),
                                    value.clone(),
                                ))
                            }
                        }
                    }
                    "uint" => {
                        if arg_action == "store-table" {
                            if let Some(vals) = arg_matches.get_many::<u64>(&arg_name) {
                                _ = update.call::<(mlua::Table, mlua::Table)>((
                                    config_table.clone(),
                                    vals.map(|v| v.to_owned()).collect::<Vec<u64>>(),
                                ))
                            }
                        } else {
                            let value = if wizard {
                                let mut num = input.trim().parse::<u64>();
                                if required {
                                    while num.is_err() {
                                        input.clear();
                                        print!("{prompt}: ");
                                        std::io::stdout().flush().unwrap_or_else(|e| {
                                            eprintln!("Something went bad!\nError: {e}");
                                        });
                                        std::io::stdin().read_line(&mut input).unwrap_or_else(
                                            |e| {
                                                eprintln!("Something went bad!\nError: {e}");
                                                1
                                            },
                                        );
                                        num = input.trim().parse::<u64>();
                                    }
                                }
                                if num.is_ok() {
                                    Some(num.unwrap())
                                } else {
                                    None
                                }
                            } else {
                                if let Some(val) = arg_matches.get_one::<u64>(&arg_name) {
                                    Some(val.to_owned())
                                } else {
                                    None
                                }
                            };
                            if let Some(value) = value {
                                _ = update.call::<(mlua::Table, mlua::Number)>((
                                    config_table.clone(),
                                    value.clone(),
                                ))
                            }
                        }
                    }
                    "real" => {
                        if arg_action == "store-table" {
                            if let Some(vals) = arg_matches.get_many::<f64>(&arg_name) {
                                _ = update.call::<(mlua::Table, mlua::Table)>((
                                    config_table.clone(),
                                    vals.map(|v| v.to_owned()).collect::<Vec<f64>>(),
                                ))
                            }
                        } else {
                            let value = if wizard {
                                let mut num = input.trim().parse::<f64>();
                                if required {
                                    while num.is_err() {
                                        input.clear();
                                        print!("{prompt}: ");
                                        std::io::stdout().flush().unwrap_or_else(|e| {
                                            eprintln!("Something went bad!\nError: {e}");
                                        });
                                        std::io::stdin().read_line(&mut input).unwrap_or_else(
                                            |e| {
                                                eprintln!("Something went bad!\nError: {e}");
                                                1
                                            },
                                        );
                                        num = input.trim().parse::<f64>();
                                    }
                                }
                                if num.is_ok() {
                                    Some(num.unwrap())
                                } else {
                                    None
                                }
                            } else {
                                if let Some(val) = arg_matches.get_one::<f64>(&arg_name) {
                                    Some(val.to_owned())
                                } else {
                                    None
                                }
                            };
                            if let Some(value) = value {
                                _ = update.call::<(mlua::Table, mlua::Number)>((
                                    config_table.clone(),
                                    value.clone(),
                                ))
                            }
                        }
                    }
                    "table" => {
                        let values = if wizard {
                            input.split(",").collect()
                        } else {
                            arg_matches
                                .get_many::<String>(&arg_name)
                                .unwrap_or_default()
                                .map(|v| v.as_str())
                                .collect::<Vec<_>>()
                        };
                        let table = self.lua.create_table()?;
                        let mut split;
                        for (i, val) in values.iter().enumerate() {
                            if val.split("=").count() == 1 {
                                table.set(i + 1, val.trim())?;
                            } else if val.split("=").count() == 2 {
                                split = val.split("=");
                                table.set(
                                    split.next().unwrap().trim(),
                                    split.next().unwrap().trim(),
                                )?;
                            }
                        }
                        _ = update.call::<(mlua::Table, mlua::Table)>((config_table.clone(), table))
                    }
                    "nil" => {
                        let include = if wizard {
                            print!("Include {arg_name} (Y/N): ");
                            std::io::stdout().flush().unwrap_or_else(|e| {
                                eprintln!("Something went bad!\nError: {e}");
                            });
                            std::io::stdin().read_line(&mut input).unwrap_or_else(|e| {
                                eprintln!("Something went bad!\nError: {e}");
                                1
                            });
                            &String::from("yes").contains(&input.to_lowercase())
                        } else {
                            arg_matches.get_one::<bool>(&arg_name).unwrap_or(&false)
                        };
                        if *include {
                            _ = update.call::<mlua::Table>(config_table.clone())
                        }
                    }
                    "ip-address" => {
                        if arg_action == "store-table" {
                            if let Some(vals) = arg_matches.get_many::<IpAddress>(&arg_name) {
                                _ = update.call::<(mlua::Table, mlua::Table)>((
                                    config_table.clone(),
                                    vals.map(|v| v.to_owned()).collect::<Vec<IpAddress>>(),
                                ))
                            }
                        } else {
                            let value = if wizard {
                                let mut num = input.trim().parse::<IpAddress>();
                                if required {
                                    while num.is_err() {
                                        input.clear();
                                        print!("{prompt}: ");
                                        std::io::stdout().flush().unwrap_or_else(|e| {
                                            error!("Something went bad!\nError: {e}");
                                        });
                                        std::io::stdin().read_line(&mut input).unwrap_or_else(
                                            |e| {
                                                error!("Something went bad!\nError: {e}");
                                                1
                                            },
                                        );
                                        num = input.trim().parse::<IpAddress>();
                                    }
                                }
                                if num.is_ok() {
                                    Some(num.unwrap())
                                } else {
                                    None
                                }
                            } else {
                                if let Some(val) = arg_matches.get_one::<IpAddress>(&arg_name) {
                                    Some(val.to_owned())
                                } else {
                                    None
                                }
                            };
                            if let Some(value) = value {
                                _ = update.call::<(mlua::Table, mlua::Number)>((
                                    config_table.clone(),
                                    value.clone(),
                                ))
                            }
                        }
                    }
                    "network" => {
                        if arg_action == "store-table" {
                            if let Some(vals) = arg_matches.get_many::<Network>(&arg_name) {
                                _ = update.call::<(mlua::Table, mlua::Table)>((
                                    config_table.clone(),
                                    vals.map(|v| v.to_owned()).collect::<Vec<Network>>(),
                                ))
                            }
                        } else {
                            let value = if wizard {
                                let mut num = input.trim().parse::<Network>();
                                if required {
                                    while num.is_err() {
                                        input.clear();
                                        print!("{prompt}: ");
                                        std::io::stdout().flush().unwrap_or_else(|e| {
                                            error!("Something went bad!\nError: {e}");
                                        });
                                        std::io::stdin().read_line(&mut input).unwrap_or_else(
                                            |e| {
                                                error!("Something went bad!\nError: {e}");
                                                1
                                            },
                                        );
                                        num = input.trim().parse::<Network>();
                                    }
                                }
                                if num.is_ok() {
                                    Some(num.unwrap())
                                } else {
                                    None
                                }
                            } else {
                                if let Some(val) = arg_matches.get_one::<Network>(&arg_name) {
                                    Some(val.to_owned())
                                } else {
                                    None
                                }
                            };
                            if let Some(value) = value {
                                _ = update.call::<(mlua::Table, mlua::Number)>((
                                    config_table.clone(),
                                    value.clone(),
                                ))
                            }
                        }
                    }
                    "mask" => {
                        if arg_action == "store-table" {
                            if let Some(vals) = arg_matches.get_many::<Mask>(&arg_name) {
                                _ = update.call::<(mlua::Table, mlua::Table)>((
                                    config_table.clone(),
                                    vals.map(|v| v.to_owned()).collect::<Vec<Mask>>(),
                                ))
                            }
                        } else {
                            let value = if wizard {
                                let mut num = input.trim().parse::<Mask>();
                                if required {
                                    while num.is_err() {
                                        input.clear();
                                        print!("{prompt}: ");
                                        std::io::stdout().flush().unwrap_or_else(|e| {
                                            error!("Something went bad!\nError: {e}");
                                        });
                                        std::io::stdin().read_line(&mut input).unwrap_or_else(
                                            |e| {
                                                error!("Something went bad!\nError: {e}");
                                                1
                                            },
                                        );
                                        num = input.trim().parse::<Mask>();
                                    }
                                }
                                if num.is_ok() {
                                    Some(num.unwrap())
                                } else {
                                    None
                                }
                            } else {
                                if let Some(val) = arg_matches.get_one::<Mask>(&arg_name) {
                                    Some(val.to_owned())
                                } else {
                                    None
                                }
                            };
                            if let Some(value) = value {
                                _ = update.call::<(mlua::Table, mlua::Number)>((
                                    config_table.clone(),
                                    value.clone(),
                                ))
                            }
                        }
                    }
                    "mac-address" => {
                        if arg_action == "store-table" {
                            if let Some(vals) = arg_matches.get_many::<MacAddress>(&arg_name) {
                                _ = update.call::<(mlua::Table, mlua::Table)>((
                                    config_table.clone(),
                                    vals.map(|v| v.to_owned()).collect::<Vec<MacAddress>>(),
                                ))
                            }
                        } else {
                            let value = if wizard {
                                let mut num = input.trim().parse::<MacAddress>();
                                if required {
                                    while num.is_err() {
                                        input.clear();
                                        print!("{prompt}: ");
                                        std::io::stdout().flush().unwrap_or_else(|e| {
                                            error!("Something went bad!\nError: {e}");
                                        });
                                        std::io::stdin().read_line(&mut input).unwrap_or_else(
                                            |e| {
                                                error!("Something went bad!\nError: {e}");
                                                1
                                            },
                                        );
                                        num = input.trim().parse::<MacAddress>();
                                    }
                                }
                                if num.is_ok() {
                                    Some(num.unwrap())
                                } else {
                                    None
                                }
                            } else {
                                if let Some(val) = arg_matches.get_one::<MacAddress>(&arg_name) {
                                    Some(val.to_owned())
                                } else {
                                    None
                                }
                            };
                            if let Some(value) = value {
                                _ = update.call::<(mlua::Table, mlua::Number)>((
                                    config_table.clone(),
                                    value.clone(),
                                ))
                            }
                        }
                    }
                    "interface" => {
                        if arg_action == "store-table" {
                            if let Some(vals) = arg_matches.get_many::<Interface>(&arg_name) {
                                _ = update.call::<(mlua::Table, mlua::Table)>((
                                    config_table.clone(),
                                    vals.map(|v| v.to_owned()).collect::<Vec<Interface>>(),
                                ))
                            }
                        } else {
                            let value = if wizard {
                                let mut num = input.trim().parse::<Interface>();
                                if required {
                                    while num.is_err() {
                                        input.clear();
                                        print!("{prompt}: ");
                                        std::io::stdout().flush().unwrap_or_else(|e| {
                                            error!("Something went bad!\nError: {e}");
                                        });
                                        std::io::stdin().read_line(&mut input).unwrap_or_else(
                                            |e| {
                                                error!("Something went bad!\nError: {e}");
                                                1
                                            },
                                        );
                                        num = input.trim().parse::<Interface>();
                                    }
                                }
                                if num.is_ok() {
                                    Some(num.unwrap())
                                } else {
                                    None
                                }
                            } else {
                                if let Some(val) = arg_matches.get_one::<Interface>(&arg_name) {
                                    Some(val.to_owned())
                                } else {
                                    None
                                }
                            };
                            if let Some(value) = value {
                                _ = update.call::<(mlua::Table, mlua::Number)>((
                                    config_table.clone(),
                                    value.clone(),
                                ))
                            }
                        }
                    }
                    "path" => {
                        if arg_action == "store-table" {
                            if let Some(vals) = arg_matches.get_many::<Path>(&arg_name) {
                                _ = update.call::<(mlua::Table, mlua::Table)>((
                                    config_table.clone(),
                                    vals.map(|v| v.to_owned()).collect::<Vec<Path>>(),
                                ))
                            }
                        } else {
                            let value = if wizard {
                                let mut num = input.trim().parse::<Path>();
                                if required {
                                    while num.is_err() {
                                        input.clear();
                                        print!("{prompt}: ");
                                        std::io::stdout().flush().unwrap_or_else(|e| {
                                            error!("Something went bad!\nError: {e}");
                                        });
                                        std::io::stdin().read_line(&mut input).unwrap_or_else(
                                            |e| {
                                                error!("Something went bad!\nError: {e}");
                                                1
                                            },
                                        );
                                        num = input.trim().parse::<Path>();
                                    }
                                }
                                if num.is_ok() {
                                    Some(num.unwrap())
                                } else {
                                    None
                                }
                            } else {
                                if let Some(val) = arg_matches.get_one::<Path>(&arg_name) {
                                    Some(val.to_owned())
                                } else {
                                    None
                                }
                            };
                            if let Some(value) = value {
                                _ = update.call::<(mlua::Table, mlua::Number)>((
                                    config_table.clone(),
                                    value.clone(),
                                ))
                            }
                        }
                    }
                    "url" => {
                        if arg_action == "store-table" {
                            if let Some(vals) = arg_matches.get_many::<Url>(&arg_name) {
                                _ = update.call::<(mlua::Table, mlua::Table)>((
                                    config_table.clone(),
                                    vals.map(|v| v.to_owned()).collect::<Vec<Url>>(),
                                ))
                            }
                        } else {
                            let value = if wizard {
                                let mut num = input.trim().parse::<Url>();
                                if required {
                                    while num.is_err() {
                                        input.clear();
                                        print!("{prompt}: ");
                                        std::io::stdout().flush().unwrap_or_else(|e| {
                                            error!("Something went bad!\nError: {e}");
                                        });
                                        std::io::stdin().read_line(&mut input).unwrap_or_else(
                                            |e| {
                                                error!("Something went bad!\nError: {e}");
                                                1
                                            },
                                        );
                                        num = input.trim().parse::<Url>();
                                    }
                                }
                                if num.is_ok() {
                                    Some(num.unwrap())
                                } else {
                                    None
                                }
                            } else {
                                if let Some(val) = arg_matches.get_one::<Url>(&arg_name) {
                                    Some(val.to_owned())
                                } else {
                                    None
                                }
                            };
                            if let Some(value) = value {
                                _ = update.call::<(mlua::Table, mlua::Number)>((
                                    config_table.clone(),
                                    value.clone(),
                                ))
                            }
                        }
                    }
                    _ => {
                        if arg_action == "store-table" && wizard {
                            let values = input
                                .trim()
                                .split(&delimeter)
                                .map(str::trim)
                                .collect::<Vec<&str>>();
                            _ = update
                                .call::<(mlua::Table, mlua::Table)>((config_table.clone(), values))
                        } else {
                            if arg_action == "store-table" {
                                if let Some(vals) = arg_matches.get_many::<String>(&arg_name) {
                                    _ = update.call::<(mlua::Table, mlua::Table)>((
                                        config_table.clone(),
                                        vals.map(|v| v.to_owned()).collect::<Vec<String>>(),
                                    ))
                                }
                            } else {
                                let value = if wizard {
                                    if input.trim() != "" {
                                        Some(&input)
                                    } else {
                                        None
                                    }
                                } else {
                                    arg_matches.get_one::<String>(&arg_name)
                                };
                                if let Some(value) = value {
                                    _ = update.call::<(mlua::Table, String)>((
                                        config_table.clone(),
                                        value.trim(),
                                    ))
                                }
                            }
                        }
                    }
                }
            }
        }
        let config_func = table.get::<mlua::Function>("config");
        let mut config_str = if let Ok(config) = config_func {
            config.call::<String>(config_table)?
        } else {
            config_func.err().unwrap().to_string()
        };
        let subcmds = table.get::<mlua::Table>("subcommands");
        if let Ok(subcmds) = subcmds {
            let mut scmd_name;
            let mut scmd_opts;
            let mut subconfig;
            let mut scmd = None;
            for subcmd in subcmds.pairs::<String, mlua::Table>() {
                (scmd_name, scmd_opts) = subcmd?;
                let include = if wizard {
                    input.clear();
                    print!("Include {scmd_name} (Y/N): ");
                    std::io::stdout().flush().unwrap_or_else(|e| {
                        eprintln!("Something went bad!\nError: {e}");
                    });
                    std::io::stdin().read_line(&mut input).unwrap_or_else(|e| {
                        eprintln!("Something went bad!\nError: {e}");
                        1
                    });
                    print!("\x1B[1A");
                    print!("\x1B[2K");
                    _yes.contains(&input.trim().to_lowercase())
                } else {
                    if let Some((clap_scmd_name, clap_scmd)) = arg_matches.subcommand() {
                        scmd = Some(clap_scmd);
                        clap_scmd_name == scmd_name
                    } else {
                        false
                    }
                };
                if include {
                    subconfig = self.handle_cli_matches(
                        scmd.unwrap_or(arg_matches),
                        (scmd_name, &scmd_opts),
                        *scmd
                            .unwrap_or(arg_matches)
                            .get_one::<bool>("wizard")
                            .unwrap(),
                    )?;
                    config_str = format!("{config_str}{}", subconfig);
                }
            }
        }
        Ok(config_str)
    }
    /// Clone a remote plugin using https (to-do ssh)
    pub fn clone_plugin(link: &str, plugin_dir: &mut PathBuf, force: bool, skip: bool) -> String {
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
        if plugin_name.ends_with(".yrnu") {
            plugin_name = plugin_name[..plugin_name.len() - 5].to_owned();
        }
        plugin_name = plugin_name.replace("-", "_").replace("/", "-");
        plugin_dir.push(&plugin_name);
        if !plugin_dir.is_dir() {
            _ = std::fs::create_dir_all(plugin_dir.clone());
        } else if let Ok(entries) = fs::read_dir(&plugin_dir) {
            if !skip {
                if entries.into_iter().count() > 0 && !force {
                    eprintln!(
                        "The plugin directory is installed and is not empty! use \"-f\" to force"
                    );
                    std::process::exit(1)
                }
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
    pub fn clone_local_plugin(
        src: &PathBuf,
        plugin_dir: &PathBuf,
        force: bool,
        mov: bool,
    ) -> String {
        let name = src.file_name().unwrap_or_else(|| {
            eprintln!("Invalid path: {}", src.to_str().unwrap_or_default());
            std::process::exit(1);
        });
        let plugin_dir = plugin_dir.join(format!("local-{}", name.display()));
        if !plugin_dir.is_dir() {
            _ = std::fs::create_dir_all(plugin_dir.clone());
        } else if let Ok(entries) = fs::read_dir(&plugin_dir) {
            if entries.into_iter().count() > 0 && !force {
                eprintln!(
                    "The plugin directory is installed and is not empty! use \"-f\" to force"
                );
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
                    _ = Self::clone_local_plugin(&entry_path, &plugin_dir, force, mov);
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
    /// Runs a Lua script
    pub fn run_script(&self, name: &String) -> mlua::Result<()> {
        let mut file = File::open(name)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        lua::run(&self.lua, &content)?;
        Ok(())
    }
    /// Prints all the plugins that currently installed on the system
    fn list(&self, lib: bool) {
        let path = if lib {
            self.root.join("libs")
        } else {
            self.root.join("plugins")
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
    /// Getter for the ArgMatches
    pub fn get_matches(&self) -> ArgMatches {
        self.args.clone().get_matches()
    }

    pub fn lua_setup(self) -> mlua::Result<Self> {
        let main_table = self.lua.create_table()?;
        main_table.set(
    "run",
    self.lua.create_function(
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
        let exec_func =
            self.lua
                .create_function(|lua: &Lua, (cmd, options): (String, mlua::Value)| {
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
                        shell = table.get::<String>("shell").unwrap_or(
                            if cfg!(target_os = "windows") {
                                "cmd".to_string()
                            } else {
                                "sh".to_string()
                            },
                        );
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
                                    let tostring =
                                        lua.create_function(|_: &Lua, this: mlua::Table| {
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
                                                    cause: std::sync::Arc::new(
                                                        mlua::Error::RuntimeError(
                                                            "Invalid output table".to_string(),
                                                        ),
                                                    ),
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
            self.lua.create_function(
                move |lua: &Lua, (cmds, options): (String, mlua::Value)| {
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
                },
            )?,
        )?;
        main_table.set(
            "match",
            self.lua
                .create_function(|_, (regex, str): (String, String)| {
                    let reg = Regex::new(&regex);
                    if reg.is_err() {
                        return Ok(mlua::Nil);
                    }
                    Ok(mlua::Value::Boolean(reg.unwrap().is_match(&str)))
                })?,
        )?;
        main_table.set(
    "serialize",
    self.lua.create_function(
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
            self.lua
                .create_function(|lua, (value, fmt): (String, String)| match fmt.as_str() {
                    "json" => match json::parse(&value) {
                        Ok(json_val) => Ok(from_json(lua, &json_val)),
                        Err(e) => {
                            eprintln!("Failed to parse JSON string!: {e}");
                            Ok(mlua::Value::Nil)
                        }
                    },
                    "yaml" => match yaml_rust2::YamlLoader::load_from_str(&value) {
                        Ok(yaml_val) => {
                            Ok(from_yaml(lua, &yaml_rust2::yaml::Yaml::Array(yaml_val)))
                        }
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
                                    if let Ok(version) =
                                        String::from_utf8(version.as_ref().to_vec())
                                    {
                                        decl_table.set("version", version)?;
                                    }
                                    if let Some(Ok(encoding)) = decl.encoding() {
                                        if let Ok(encoding) =
                                            String::from_utf8(encoding.as_ref().to_vec())
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
                                if let Event::Empty(_) = &events[index] {
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
        self.lua.globals().set("yrnu", main_table)?;
        Ok(self)
    }
}

fn main() {
    // Creating the main yrnu instance
    let yrnu = Yrnu::new(None).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1)
    });
    let arg_matches = yrnu.get_matches();
    if let Some(script) = arg_matches.get_one::<String>("script") {
        // Run a Lua file as a Yrnu script
        if let Err(e) = yrnu.run_script(script) {
            println!("{e}");
            std::process::exit(-1);
        };
    } else {
        let mut wizard; // Whether to run as a wizard mode
        match arg_matches.subcommand() {
            Some(("config", config)) => {
                if let Some((_, scmd)) = config.subcommand() {
                    // Figuring out which plugin to run
                    for plugin in &yrnu.plugins {
                        if let Some((scmd_name, scmd)) = scmd.subcommand() {
                            // Figuring out which global to run
                            for global in &plugin.1 {
                                if scmd_name == global.0 {
                                    // Whether to run as a wizard mode
                                    wizard = *scmd.get_one::<bool>("wizard").unwrap()
                                        && scmd.subcommand().is_none();
                                    // Executing the command
                                    let output = yrnu.handle_cli_matches(
                                        scmd,                              // The Clap command matches
                                        (scmd_name.to_owned(), &global.1), // The global name and
                                        // table
                                        wizard,
                                    );
                                    // Printing output
                                    match output {
                                        Ok(output) => println!("\n\n{}", output.trim()),
                                        Err(e) => eprintln!("Error: {e}"),
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Some(("add", add_args)) => {
                let mut plugin_dir = yrnu.root;
                let force = *add_args.get_one::<bool>("force").unwrap();
                if *add_args.get_one::<bool>("lib").unwrap() {
                    plugin_dir.push("libs");
                } else {
                    plugin_dir.push("plugins");
                }
                if let Some(link) = add_args.get_one::<String>("url") {
                    let plugin_name = Yrnu::clone_plugin(link, &mut plugin_dir, force, false);
                    println!("Added {plugin_name}.");
                } else {
                    let mov = add_args.get_one::<bool>("move").unwrap_or_else(|| {
                        error!("Path is invalid.");
                        std::process::exit(1)
                    });
                    let src = PathBuf::from(add_args.get_one::<String>("path").unwrap());
                    if !src.exists() {
                        error!("Path not exists.");
                        std::process::exit(1)
                    }
                    let plugin_name = Yrnu::clone_local_plugin(&src, &mut plugin_dir, force, *mov);
                    if *mov {
                        _ = fs::remove_dir(&src);
                    }
                    println!("Added {plugin_name}.");
                }
            }
            Some(("remove", remove_args)) => {
                let mut plugin_dir = yrnu.root;
                let name = remove_args.get_one::<String>("name").unwrap();
                if name.contains("/")
                    || name.contains(".")
                    || name.contains("~")
                    || name.contains("\\")
                {
                    error!("Please provide a valid value");
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
                        error!("An error has been accurd: {e}");
                        std::process::exit(1);
                    });
                    yes = "yes".starts_with(&answer.trim().to_lowercase())
                }
                if !yes {
                    error!("Remove has been canceled.");
                    std::process::exit(1);
                }
                plugin_dir.push(name);
                if plugin_dir.exists() {
                    std::fs::remove_dir_all(plugin_dir).unwrap_or_else(|e| {
                        error!("Faild to remove plugin {name}\n{e}");
                        std::process::exit(1)
                    });
                    info!("Removed {name}.");
                } else {
                    error!("Faild to remove plugin {name}, {name} not exists");
                    std::process::exit(1)
                }
            }
            Some(("list", list_args)) => {
                let lib = *list_args.get_one::<bool>("lib").unwrap();
                let plugin = list_args.get_one::<bool>("plugin").unwrap();
                if !lib && !plugin {
                    yrnu.list(false);
                    yrnu.list(true);
                } else {
                    yrnu.list(lib);
                }
            }
            _ => interpreter::start_interpreter(&yrnu.lua, &yrnu.root)
                .expect("Failed to run interpreter."),
        }
    }
}
