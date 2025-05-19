use clap::builder;
use clap::{command, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use git2::FetchOptions;
use log::{error, info, warn, LevelFilter};
use mlua::Lua;
use std::default::Default;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;
use yrnu::core::{Interface, IpAddress, MacAddress, Mask, Network, Path, Url};
use yrnu::lua;

/// The global yrnu
#[derive(Debug, Clone, Default)]
struct Yrnu {
    lua: Lua,
    root: PathBuf,
    plugins: Vec<(String, mlua::Table)>,
    args: Command,
}

impl Yrnu {
    // Handle the creation of plugin argument lua API
    fn get_arg_lua_function(
        &self,
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
            self.lua
                .create_function(move |_, (this, value): (mlua::Table, mlua::Value)| {
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
    fn get_arg_clap_cmd(
        &self,
        (arg_name, arg_options): (&String, &mlua::Table),
    ) -> mlua::Result<Arg> {
        let mut arg = Arg::new(arg_name);
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
            },
        };
        Ok(arg)
    }
    // Loads a specific plugin
    fn load_plugin(
        &self,
        name: &String,
        table: &mlua::Table,
        plugin_name: &String,
    ) -> mlua::Result<(Command, mlua::Function)> {
        let start_config = table.get("preconfig").unwrap_or("".to_string());
        let end_config = table.get("postconfig").unwrap_or("".to_string());
        let mut subcmd = Command::new(name).arg(
            Arg::new("wizard")
                .short('w')
                .long("wizard")
                .exclusive(true)
                .help("Config using a wizard by asking for each argument one by one")
                .action(ArgAction::SetTrue),
        );
        let args = table.get::<mlua::Table>("args")?;
        let subcommands = table.get::<mlua::Table>("subcommands");
        if let Ok(about) = table.get::<String>("about") {
            subcmd = subcmd.about(about);
        }
        let plugin_table = self.lua.create_table()?;
        let config_func = table.get::<mlua::Function>("config")?;
        let config_func = self.lua.create_function(move |_, this: mlua::Table| {
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
                    let table = self.lua.create_table()?;
                    if value.to_str().unwrap().len() == 1 {
                        table.set("short", value)?;
                    } else {
                        table.set("help", value)?;
                    }
                    table
                }
                _ => {
                    error!(
                        "Faild to load argument {}!, value should be either a table, string or nil",
                        arg_name
                    );
                    continue;
                }
            };
            arg = self.get_arg_clap_cmd((&arg_name, &arg_opts))?;
            subcmd = subcmd.arg(arg);
            func = self.get_arg_lua_function(arg_name.clone(), &arg_opts)?;
            plugin_table.set(format!("set_{}", arg_name), func)?;
        }
        let mut scmd_name;
        let mut scmd_table;
        if let Ok(subcommands) = subcommands {
            for pair in subcommands.pairs::<String, mlua::Table>() {
                (scmd_name, scmd_table) = pair?;
                let name_clone = scmd_name.clone();
                if let Ok((scmd, constructor)) =
                    self.load_plugin(&scmd_name, &scmd_table, &plugin_name)
                {
                    let construct = self.lua.create_function(
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
                    error!(
                        "Faild to load {} subcommand of {} plugin",
                        scmd_name, plugin_name
                    );
                }
            }
        }
        let init_func = if let Ok(init) = table.get::<mlua::Function>("init") {
            self.lua
                .create_function(move |_, param: Option<mlua::Table>| {
                    init.call::<Option<mlua::Table>>((plugin_table.clone(), param))
                })
        } else {
            self.lua
                .create_function(move |_, ()| Ok(plugin_table.clone()))
        };
        self.lua
            .globals()
            .set(name.to_owned(), init_func.to_owned()?)?;
        Ok((subcmd, init_func?))
    }
    // Read the plugins from the disk
    fn read_plugins(&mut self) -> mlua::Result<()> {
        let file = self.root.join("init.lua");
        if file.is_file() {
            self.lua
                .load(format!(
                    "package.path = package.path .. \";{}/?.lua\"",
                    self.root.display()
                ))
                .exec()?;
            let init = self
                .lua
                .load(std::fs::read_to_string(file)?)
                .eval::<mlua::Table>()?;
            let ensure_installed = init
                .get::<mlua::Table>("ensure_installed")
                .unwrap_or(self.lua.create_table()?);
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
                                inner_repo = format!("{inner_username}/{inner_repo}");
                                Yrnu::clone_plugin(&inner_repo, &mut plugins_path, false);
                            }
                        }
                        (mlua::Value::String(username), mlua::Value::String(repo)) => {
                            inner_username = username.to_str().unwrap();
                            let inner_repo = repo.to_str().unwrap();
                            Yrnu::clone_plugin(
                                &format!("{inner_username}/{inner_repo}"),
                                &mut plugins_path,
                                false,
                            );
                        }
                        (_, mlua::Value::String(link)) => {
                            Yrnu::clone_plugin(&link.to_str().unwrap(), &mut plugins_path, false);
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
                for plugin in plugins.pairs::<String, mlua::Table>() {
                    let (name, table) = plugin?;
                    self.plugins.push((name, table));
                }
                Ok(())
            }
        } else {
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
    pub fn load_plugins(mut self) -> Self {
        match self.read_plugins() {
            Err(e) => {
                error!("Faild to read plugins: {}", e);
            }
            _ => {
                info!("Successfuly read plugins")
            }
        }
        let mut is_err = false;
        let mut config_args = Command::new("config")
            .about("configure linux/Windows machines and network devices or any other thing that comes to mind.");
        for plugin in &self.plugins {
            match Self::load_plugin(&self, &plugin.0, &plugin.1, &plugin.0) {
                Ok((cmd, func)) => {
                    config_args = config_args.subcommand(cmd);
                    self.lua
                        .globals()
                        .set(plugin.0.to_owned(), func)
                        .unwrap_or_else(|e| {
                            error!("Faild to create subcommand for {}. Error: {e}", plugin.0)
                        });
                }
                Err(e) => {
                    error!("Faild to load {}: {e}", plugin.0);
                    is_err = true
                }
            }
        }
        if is_err {
            warn!("There is plugins that failed to load")
        }
        self.args = self.args.subcommand(config_args);
        self
    }
    // Initiate a new Yrnu instance
    pub fn new(level: Option<log::LevelFilter>) -> Result<Self, Box<dyn std::error::Error>> {
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
            simple_logging::log_to_file(&path.join("yrnu.log"), level)
                .unwrap_or_else(|e| error!("{e}"));
        } else {
            simple_logging::log_to_file(&path.join("yrnu.log"), LevelFilter::Info)
                .unwrap_or_else(|e| error!("{e}"));
        };
        let args = Self::define_cli_usage();
        let mut yrnu = Self {
            lua: lua.unwrap(),
            root: path,
            args: args.clone(),
            ..Default::default()
        };
        yrnu = yrnu.load_plugins();
        Ok(yrnu)
    }
    /// Handle the cli usage of the plugins
    pub fn handle_cli_matches(
        &self,
        arg_matches: &ArgMatches,
        plugin: &(String, mlua::Table),
        wizard: bool,
    ) -> mlua::Result<String> {
        let (_, table): &(String, mlua::Table) = plugin;
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
        let args = table.get::<mlua::Table>("args");
        if let Ok(args) = args {
            let mut arg_name;
            let mut arg_opts;
            let mut arg_values;
            let mut prompt;
            let mut required;
            for arg in args.pairs::<String, mlua::Value>() {
                (arg_name, arg_values) = arg.unwrap_or_else(|_| {
                    error!("Invalid argument definition of args");
                    std::process::exit(11)
                });
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
                    print!("\x1B[1A");
                    print!("\x1B[2K");
                    input.clear();
                    print!("{prompt}: ");
                    std::io::stdout().flush().unwrap_or_else(|e| {
                        eprintln!("Something went bad!\nError: {e}");
                    });
                    std::io::stdin().read_line(&mut input).unwrap_or_else(|e| {
                        eprintln!("Something went bad!\nError: {e}");
                        1
                    });
                }
                match arg_opts
                    .get::<String>("arg_type")
                    .unwrap_or_default()
                    .as_str()
                {
                    "bool" | "boolish" => {
                        let value = if wizard {
                            if arg_opts.get::<String>("arg_type").unwrap_or_default() == "bool" {
                                Some(_true.contains(&input))
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
                        } else {
                            if let Some(val) = arg_matches.get_one::<bool>(&arg_name) {
                                Some(val.to_owned())
                            } else {
                                None
                            }
                        };
                        if let Some(value) = value {
                            _ = update
                                .call::<(mlua::Table, bool)>((config_table.clone(), value.clone()))
                        }
                    }
                    "int" => {
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
                                    std::io::stdin().read_line(&mut input).unwrap_or_else(|e| {
                                        error!("Something went bad!\nError: {e}");
                                        1
                                    });
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
                    "uint" => {
                        let value = if wizard {
                            let mut num = input.trim().parse::<u64>();
                            if required {
                                while num.is_err() {
                                    input.clear();
                                    print!("{prompt}: ");
                                    std::io::stdout().flush().unwrap_or_else(|e| {
                                        eprintln!("Something went bad!\nError: {e}");
                                    });
                                    std::io::stdin().read_line(&mut input).unwrap_or_else(|e| {
                                        eprintln!("Something went bad!\nError: {e}");
                                        1
                                    });
                                    print!("\x1B[1A");
                                    print!("\x1B[2K");
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
                    "real" => {
                        let value = if wizard {
                            let mut num = input.trim().parse::<f64>();
                            if required {
                                while num.is_err() {
                                    input.clear();
                                    print!("{prompt}: ");
                                    std::io::stdout().flush().unwrap_or_else(|e| {
                                        eprintln!("Something went bad!\nError: {e}");
                                    });
                                    std::io::stdin().read_line(&mut input).unwrap_or_else(|e| {
                                        eprintln!("Something went bad!\nError: {e}");
                                        1
                                    });
                                    print!("\x1B[1A");
                                    print!("\x1B[2K");
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
                    "network" => {
                        let value = if wizard {
                            let mut num = input.trim().parse::<Network>();
                            if required {
                                while num.is_err() {
                                    print!("\x1B[1A");
                                    print!("\x1B[2K");
                                    input.clear();
                                    print!("{prompt}: ");
                                    std::io::stdout().flush().unwrap_or_else(|e| {
                                        error!("Something went bad!\nError: {e}");
                                    });
                                    std::io::stdin().read_line(&mut input).unwrap_or_else(|e| {
                                        error!("Something went bad!\nError: {e}");
                                        1
                                    });
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
                    _ => {
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
                            _ = update
                                .call::<(mlua::Table, String)>((config_table.clone(), value.trim()))
                        }
                    }
                }
            }
        }
        print!("\x1B[1A");
        print!("\x1B[2K");
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
                        &(scmd_name, scmd_opts),
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
    pub fn clone_plugin(link: &str, plugin_dir: &mut PathBuf, force: bool) -> String {
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
                eprintln!(
                    "The plugin directory is installed and is not empty! use \"-f\" to force"
                );
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
        let plugin_dir = plugin_dir.join(name);
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
    /// Runs a lua script
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
}

fn main() {
    let yrnu = Yrnu::new(None).unwrap();
    let arg_matches = yrnu.get_matches();
    if let Some(script) = arg_matches.get_one::<String>("script") {
        if let Err(e) = yrnu.run_script(script) {
            println!("{e}");
            std::process::exit(-1);
        };
    } else {
        let mut wizard;
        match arg_matches.subcommand() {
            Some(("config", config)) => {
                for plugin in &yrnu.plugins {
                    if let Some((scmd_name, scmd)) = config.subcommand() {
                        wizard =
                            *scmd.get_one::<bool>("wizard").unwrap() && scmd.subcommand().is_none();
                        if scmd_name == plugin.0 {
                            let output = yrnu.handle_cli_matches(scmd, plugin, wizard);
                            match output {
                                Ok(output) => println!("{}", output.trim()),
                                Err(e) => eprintln!("Error: {e}"),
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
                    let plugin_name = Yrnu::clone_plugin(link, &mut plugin_dir, force);
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
            _ => {
                lua::interpreter::start_interpreter(&yrnu.lua).expect("Failed to run interpreter.")
            }
        }
    }
}
