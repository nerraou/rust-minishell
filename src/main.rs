use std::process::Command;

pub mod lexer;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::lexer::tokenizer::tokenize;

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    println!("Welcome to Minishell");

    loop {
        let readline = rl.readline("minishell> ");

        match readline {
            Ok(line) => {
                let tokens = tokenize(&line);

                println!("the tokens : {:?}", tokens);
                let mut parse = line.split_whitespace();

                let Some(program) = parse.next() else {
                    continue;
                };

                // Command::new(program)
                //     .args(parse)
                //     .status()
                //     .expect("failed to execute");
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
