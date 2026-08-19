use std::iter::Peekable;
use std::vec::IntoIter;

use crate::{
    lexer::token::{Token, WordState},
    parser::{
        ast::{Command, Redirection, RedirectionType},
        error::ParseError,
    },
};

pub fn parse_redirection(
    iter: &mut Peekable<IntoIter<Token>>,
    kind: RedirectionType,
) -> Result<Redirection, ParseError> {
    iter.next();

    if let Some(Token::Word(WordState { text: target, .. })) = iter.peek() {
        let redirection = Redirection {
            kind,
            target: target.to_string(),
        };

        iter.next();

        Ok(redirection)
    } else {
        Err(ParseError::ExpectedRedirectionTarget)
    }
}

pub fn parse_command(tokens: Vec<Token>) -> Result<Command, ParseError> {
    let mut command = Command {
        program: String::new(),
        arguments: Vec::new(),
        redirections: Vec::new(),
    };

    let mut iter = tokens.into_iter().peekable();

    while let Some(value) = iter.peek() {
        match value {
            Token::Word(WordState { text: value, .. }) => {
                if command.program.is_empty() {
                    if value.is_empty() {
                        return Err(ParseError::UnexpectedProgram(value.to_string()));
                    }
                    command.program = value.to_string()
                } else {
                    command.arguments.push(value.to_string());
                }
                iter.next();
            }
            Token::RedirectIn => {
                let redirection = parse_redirection(&mut iter, RedirectionType::Input)?;
                command.redirections.push(redirection);
            }

            Token::RedirectOut => {
                let redirection = parse_redirection(&mut iter, RedirectionType::Output)?;
                command.redirections.push(redirection);
            }

            Token::RedirectAppend => {
                let redirection = parse_redirection(&mut iter, RedirectionType::Append)?;
                command.redirections.push(redirection);
            }

            Token::Heredoc => {
                let redirection = parse_redirection(&mut iter, RedirectionType::Heredoc)?;
                command.redirections.push(redirection);
            }
            Token::Pipe => todo!(),
        }
    }

    return Ok(command);
}
