use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use clap::{command, Command, Arg};
use yrnu::lua;

fn read_script(name: String) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let args_matches = command!()
        .about("a tool for networking and cyber specialists, 
featuring Lua scripting and the yrnu library for crafting networking tools and automating tasks. 
Key features include configuring network settings, sending custom traffic, and deploying servers."
        )
        .version("0.0.1")
        .arg(
            Arg::new("script")
                .help("A lua script to execute")
        )
        .get_matches();
    if let Some(script) = args_matches.get_one::<String>("script") {
        match read_script(script.to_string()) {
            Ok(contents) => {
                //println!("Running:\n{}",contents);
                let lua_ctx = lua::init();
                if let Err(e) = lua::run(lua_ctx.unwrap(), contents) {
                    eprintln!("{}",e);
                }

            },
            Err(e) => eprintln!("{}",e),
        }
                
    }
}
