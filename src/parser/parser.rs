use crate::{
    lexer::token::{Token, WordState},
    parser::{ast::Command, error::ParseError},
};

pub fn parse_command(tokens: Vec<Token>) -> Result<Command, ParseError> {
    let mut command = Command {
        program: String::new(),
        arguments: Vec::new(),
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
            Token::Pipe => todo!(),
            Token::RedirectIn => todo!(),
            Token::RedirectOut => todo!(),
            Token::RedirectAppend => todo!(),
            Token::Heredoc => todo!(),
        }
    }

    return Ok(command);
}
