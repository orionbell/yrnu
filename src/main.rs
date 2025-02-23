use clap::{
    builder, command, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command, Subcommand,
};
use mlua::Lua;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use yrnu::lua;
fn handle_cli_matches(
    lua: &Lua,
    matches: &ArgMatches,
    plugin: &(String, mlua::Table),
) -> mlua::Result<String> {
    let (name, table): &(String, mlua::Table) = plugin;
    if let Some((name, plugin_args)) = matches.subcommand() {
        let config_table = lua.create_table().unwrap_or_else(|_| {
            eprintln!("Something went wrong...");
            std::process::exit(10)
        });
        let args = table.get::<mlua::Table>("args");
        if let Ok(args) = args {
            let mut arg_name;
            let mut arg_opts;
            for arg in args.pairs::<String, mlua::Table>() {
                (arg_name, arg_opts) = arg.unwrap_or_else(|_| {
                    eprintln!("Invalid argument definition for ");
                    std::process::exit(11)
                });
                if let Some(value) = plugin_args.get_one::<String>(&arg_name) {
                    let update = arg_opts.get::<mlua::Function>("update");
                    if let Ok(update) = update {
                        _ = update
                            .call::<(mlua::Table, String)>((config_table.clone(), value.clone()));
                    } else {
                        eprintln!("Failed to find \"update\" method for {}", arg_name);
                        std::process::exit(12)
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

fn handle_lua_api(lua: &Lua, arg_options: &mlua::Table) -> mlua::Result<mlua::Function> {
    let update = arg_options.get::<mlua::Function>("update")?;
    let func = lua.create_function(move |_, (this, value): (mlua::Table, String)| {
        update.call((this, value))?;
        Ok(())
    })?;
    Ok(func)
}

fn handle_cli((arg_name, arg_options): (&String, &mlua::Table)) -> mlua::Result<Arg> {
    let mut arg = Arg::new(arg_name);
    if let Ok(required) = arg_options.get::<bool>("required") {
        arg = arg.required(required);
    }
    if let Ok(short) = arg_options.get::<String>("short") {
        if short.len() != 1 {
            return Err(todo!());
        }
        arg = arg.short(short.chars().next().unwrap());
    }
    if let Ok(long) = arg_options.get::<String>("long") {
        if long.len() == 0 {
            return Err(todo!());
        }
        arg = arg.long(long);
    }
    if let Ok(help) = arg_options.get::<String>("help") {
        arg = arg.help(help);
    }
    if let Ok(action) = arg_options.get::<String>("action") {}
    Ok(arg)
}

fn load_plugin(lua: &Lua, plugin: (&String, &mlua::Table)) -> mlua::Result<(Command, mlua::Table)> {
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
    for pair in args.pairs::<String, mlua::Table>() {
        (arg_name, arg_opts) = pair?;
        arg = handle_cli((&arg_name, &arg_opts))?;
        subcmd = subcmd.arg(arg);
        func = handle_lua_api(lua, &arg_opts)?;
        plugin_table.set(arg_name, func)?;
    }
    let mut scmd_name;
    let mut scmd_table;
    let mut scmd;
    let mut plug_table;
    if let Ok(subcommands) = subcommands {
        for pair in subcommands.pairs::<String, mlua::Table>() {
            (scmd_name, scmd_table) = pair?;
            (scmd, plug_table) = load_plugin(lua, (&scmd_name, &scmd_table))?;
            subcmd = subcmd.subcommand(scmd);
            _ = plugin_table.set(scmd_name, plug_table);
        }
    }
    Ok((subcmd, plugin_table))
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
        path.push("yrnu");
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
        print!("Loading plugin {} ...", name);
        let plugin = load_plugin(&lua_ctx, (name, table));
        if plugin.is_err() {
            println!("failed!");
            eprintln!("{:?}", plugin);
        } else {
            let (subcmd, plugin_table) = plugin.unwrap();
            let constructor = lua_ctx.create_function(move |_, ()| Ok(plugin_table.clone()));
            if let Ok(constructor) = constructor {
                _ = lua_ctx
                    .globals()
                    .set(format!("{}_config", name), constructor);
                config_args = config_args.subcommand(subcmd);
                println!("done!");
            } else {
                println!("failed!");
                eprintln!("{:?}", constructor);
            }
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
