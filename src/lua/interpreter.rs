use mlua::Lua;
use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{config, Editor, Helper};
use std::borrow::Cow;
use std::error::Error;

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

pub fn start_interpreter() -> Result<(), Box<dyn Error>> {
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
    let lua = super::init()?;
    rl.set_helper(Some(token_groups));
    if rl.load_history("history").is_err() {
        //println!("No History to load...");
    } else {
        //println!("Loading commands history");
    }
    let mut incomplete = false;
    let mut code = String::new();
    let mut code_lines;
    let mut prompt;
    loop {
        prompt = if incomplete {
            "\x1b[1;34m... \x1b[0m"
        } else {
            "\x1b[1;34m>>> \x1b[0m"
        };

        match rl.readline(prompt) {
            Ok(line) => {
                code.push_str(&line);
                code.push('\n');
                _ = rl.add_history_entry(line.as_str());
                match super::run(&lua, &code) {
                    Ok(_) => {
                        incomplete = false;
                        code = "".to_string();
                    }
                    Err(e) => match e {
                        mlua::Error::SyntaxError {
                            message,
                            incomplete_input,
                        } if incomplete_input => {
                            incomplete = true;
                            continue;
                        }
                        mlua::Error::SyntaxError {
                            message,
                            incomplete_input,
                        } => {
                            code_lines = code.split("\n").collect::<Vec<&str>>();
                            code_lines.pop();
                            code_lines.pop();
                            code = code_lines.join("\n");
                            code.push('\n');
                            eprintln!("{}", message.split(":").last().unwrap_or_default());
                        }
                        _ => {
                            code_lines = code.split("\n").collect::<Vec<&str>>();
                            code_lines.pop();
                            code = code_lines.join("\n");
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
    _ = rl.save_history("history");
    Ok(())
}
