use std::process::Command;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    println!("Welcome to Minishell");

    loop {
        let readline = rl.readline("minishell> ");

        match readline {
            Ok(line) => {
                let mut parse = line.split_whitespace();
                Command::new(parse.next().unwrap_or("ls"))
                    .args(parse)
                    .status()
                    .expect("failed to execute");
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");

                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
