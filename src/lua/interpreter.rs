#[allow(unused_assignments)]
use log::error;
use mlua::Lua;
use rustyline::Helper;
use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use std::borrow::Cow;
use std::error::Error;
use std::path::PathBuf;

struct TokenGroups {
    keywords: Vec<String>,
    operators: Vec<String>,
}

impl Hinter for TokenGroups {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        let last = line.split(" ").last().unwrap_or_default();
        if last == "" {
            return None;
        }
        for member in &self.keywords {
            if member.starts_with(last) {
                return Some(member[last.len()..].to_owned());
            }
        }
        for member in &self.operators {
            if member.starts_with(last) {
                return Some(member[last.len()..].to_owned());
            }
        }
        HistoryHinter::new().hint(line, pos, ctx)
    }
}
impl Validator for TokenGroups {}
impl Highlighter for TokenGroups {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::from(format!("\x1b[1;90m{}\x1b[0m", hint))
    }
}
impl Completer for TokenGroups {
    type Candidate = String;
}
impl Helper for TokenGroups {}

pub fn print_table(table: &mlua::Table, depth: u8, max_depth: u8) {
    if depth == max_depth {
        println!("...");
        return;
    }
    if table.sequence_values::<mlua::Value>().count() != 0
        && table.sequence_values::<mlua::Value>().count()
            == table.pairs::<mlua::Value, mlua::Value>().count()
    {
        println!("{}{{", "  ".repeat(depth as usize));
        for entry in table.pairs::<String, mlua::Value>() {
            if let Ok((key, val)) = entry {
                if val.is_table() {
                    print_table(val.as_table().unwrap(), depth - 1, max_depth);
                } else {
                    println!(
                        "{}{} = {},",
                        "  ".repeat((depth + 1) as usize),
                        key,
                        val.to_string().unwrap_or_default()
                    );
                }
            }
        }
        println!("{}}}", "  ".repeat(depth as usize));
    }
}
pub fn start_interpreter(lua: &Lua, root: &PathBuf) -> Result<(), Box<dyn Error>> {
    let token_groups = TokenGroups {
        keywords: vec![
            "local".into(),
            "function".into(),
            "if".into(),
            "end".into(),
            "for".into(),
            "while".into(),
            " and ".into(),
            " or ".into(),
            "break".into(),
            "continue".into(),
            "else".into(),
            "elseif".into(),
            "do".into(),
            " in ".into(),
            " not ".into(),
            "return".into(),
            "until".into(),
            " then".into(),
            "repeat".into(),
        ],
        operators: vec![
            "=".into(),
            "==".into(),
            "<=".into(),
            ">=".into(),
            "<".into(),
            ">".into(),
            "+".into(),
            "-".into(),
            "/".into(),
            "*".into(),
            "%".into(),
            ")".into(),
            "(".into(),
            "^".into(),
            "~=".into(),
            "{".into(),
            "}".into(),
            "[".into(),
            "]".into(),
            ";".into(),
            ":".into(),
            "..".into(),
            "...".into(),
            "#".into(),
        ],
    };
    let mut rl = rustyline::Editor::<TokenGroups, DefaultHistory>::new().unwrap();
    rl.set_helper(Some(token_groups));
    if let Err(_e) = rl.load_history(&root.join("history")) {
        println!("No History file to load, creating history...");
        if let Err(e) = std::fs::File::create(root.join("history")) {
            eprintln!("Failed to create history file.\nError: {e}");
            error!("Failed to create history file.\nError: {e}");
        } else {
            if let Err(e) = rl.load_history(&root.join("history")) {
                eprintln!("Failed to load history file.\nError: {e}");
                error!("Failed to load history file.\nError: {e}");
            }
        }
    }
    let mut incomplete = false;
    let mut code = String::new();
    let mut code_lines;
    let mut prompt;
    println!(
        "\x1b[1;34m\n\n\n\t██╗   ██╗██████╗ ███╗   ██╗██╗   ██╗
\t╚██╗ ██╔╝██╔══██╗████╗  ██║██║   ██║
\t ╚████╔╝ ██████╔╝██╔██╗ ██║██║   ██║
\t  ╚██╔╝  ██╔══██╗██║╚██╗██║██║   ██║
\t   ██║   ██║  ██║██║ ╚████║╚██████╔╝
\t   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝ 
\x1b[38;5;220m                     \n
\t\tGithub: orionbell
\n \x1b[0m"
    );
    loop {
        prompt = if incomplete {
            "\x1b[1;34m...\x1b[0m"
        } else {
            "\x1b[1;34m>>> \x1b[0m"
        };

        match rl.readline(prompt) {
            Ok(line) => {
                let mut is_val = false;
                code.push_str(&line);
                code.push('\n');
                _ = rl.add_history_entry(line.as_str());
                match super::run(&lua, &code) {
                    Ok(value) => {
                        if !value.is_nil() {
                            is_val = true;
                            match value {
                                mlua::Value::Table(table) => {
                                    if let Ok(_) = table.get::<mlua::Function>("config") {
                                        let value = mlua::Value::Table(table);
                                        println!("{}", value.to_string().unwrap_or_default())
                                    } else if let Some(metatable) = table.metatable() {
                                        if let Ok(_) = metatable.get::<mlua::Function>("__tostring")
                                        {
                                            println!(
                                                "{}",
                                                mlua::Value::Table(table)
                                                    .to_string()
                                                    .unwrap_or_default()
                                            )
                                        } else {
                                            print_table(&table, 0, 3);
                                        }
                                    } else {
                                        print_table(&table, 0, 3);
                                    }
                                }
                                _ => println!("{}", value.to_string().unwrap_or_default()),
                            }
                        }
                        incomplete = false;
                        code.clear();
                    }
                    Err(e) => match e {
                        mlua::Error::SyntaxError {
                            message,
                            incomplete_input,
                        } if incomplete_input && !is_val => {
                            if message.contains("'=' expected near '<eof>'") {
                                code_lines = code.split("\n").collect::<Vec<&str>>();
                                code_lines.pop();
                                code_lines.pop();
                                code = code_lines.join("\n");
                                code.push('\n');
                                eprintln!("{}", message.split(":").last().unwrap_or_default());
                            } else {
                                incomplete = true;
                                continue;
                            }
                        }
                        mlua::Error::SyntaxError { message, .. }
                            if is_val
                                && (message.contains("unexpected symbol")
                                    || message.contains("'=' expected near '<eof>'")) =>
                        {
                            code.clear()
                        }
                        mlua::Error::SyntaxError { message, .. } => {
                            code_lines = code.split("\n").collect::<Vec<&str>>();
                            code_lines.pop();
                            code_lines.pop();
                            code = code_lines.join("\n");
                            code.push('\n');
                            eprintln!("{}", message.split(":").last().unwrap_or_default());
                        }
                        _ => {
                            code.clear();
                            eprintln!("{e}");
                        }
                    },
                }
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(_) => continue,
        }
    }
    _ = rl.save_history(&root.join("history"));
    Ok(())
}
