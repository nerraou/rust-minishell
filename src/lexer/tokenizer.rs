use std::{iter::Peekable, str::Chars};

use crate::lexer::{
    error::LexerError,
    token::{QuoteState, Token, WordState},
};

pub fn skip_whitespace<'a>(iter: &mut Peekable<Chars<'a>>) {
    iter.next();
}

pub fn read_quoted(
    iter: &mut Peekable<Chars>,
    tokens: &mut Vec<Token>,
    quote: char,
) -> Result<(), LexerError> {
    iter.next();
    let mut word = String::new();
    while let Some(c) = iter.next() {
        if c == quote {
            let quote_type = if quote == '\'' {
                QuoteState::Single
            } else {
                QuoteState::Double
            };
            tokens.push(Token::Word(WordState {
                text: word,
                quote: quote_type,
            }));
            return Ok(());
        }

        word.push(c);
    }
    return Err(LexerError::UnclosedQuote(quote));
}

pub fn is_token(value: char) -> bool {
    if value == '|' || value == '<' || value == '>' {
        return true;
    }

    false
}

pub fn read_word(iter: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    let mut word = String::new();
    while let Some(&c) = iter.peek() {
        if c.is_whitespace() || is_token(c) {
            break;
        }
        iter.next();
        word.push(c);
    }
    tokens.push(Token::Word(WordState {
        text: word,
        quote: QuoteState::None,
    }));
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut iter = input.chars().peekable();
    while let Some(&value) = iter.peek() {
        match value {
            c if c.is_whitespace() => skip_whitespace(&mut iter),
            '\'' | '\"' => read_quoted(&mut iter, &mut tokens, value)?,
            '|' => {
                tokens.push(Token::Pipe);
                iter.next();
            }

            '>' => {
                iter.next();
                if let Some(&next) = iter.peek() {
                    if next == '>' {
                        tokens.push(Token::RedirectAppend);
                        iter.next();
                    } else {
                        tokens.push(Token::RedirectOut);
                    }
                } else {
                    tokens.push(Token::RedirectOut);
                }
            }
            '<' => {
                iter.next();
                if let Some(&next) = iter.peek() {
                    if next == '<' {
                        tokens.push(Token::Heredoc);
                        iter.next();
                    } else {
                        tokens.push(Token::RedirectIn);
                    }
                } else {
                    tokens.push(Token::RedirectIn);
                }
            }
            _ => read_word(&mut iter, &mut tokens),
        }
    }
    Ok(tokens)
}
