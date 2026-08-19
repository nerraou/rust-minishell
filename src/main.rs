use std::process::Command;

pub mod lexer;
pub mod parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::lexer::tokenizer::tokenize;
use crate::parser::parser::parse_command;

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    println!("Welcome to Minishell");

    loop {
        let readline = rl.readline("minishell> ");

        match readline {
            Ok(line) => {
                let tokens = tokenize(&line).unwrap();

                println!("the tokens : {:?}", tokens);
                let parse = parse_command(tokens).unwrap();

                println!("parsed '{}'", parse.program);
                println!("parsed '{:?}'", parse.arguments);
                println!("parsed redirections'{:?}'", parse.redirections);
                // let mut path = String::from("/bin/");

                // path.push_str(&parse.program);

                // Command::new(parse.program)
                //     .args(parse.arguments)
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
