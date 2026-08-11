use std::{iter::Peekable, str::Chars};

use crate::lexer::{
    error::LexerError,
    token::{
        QuoteState,
        Token::{self, Word},
        WordState,
    },
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
            tokens.push(Word(WordState {
                text: word,
                quote: quote_type,
            }));
            return Ok(());
        }

        word.push(c);
    }
    return Err(LexerError::UnclosedQuote(quote));
}

pub fn read_word(iter: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    let mut word = String::new();
    while let Some(c) = iter.next() {
        if c.is_whitespace() {
            break;
        }
        word.push(c);
    }
    tokens.push(Word(WordState {
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
            c if c.is_alphabetic() => read_word(&mut iter, &mut tokens),
            _ => return Err(LexerError::UnexpectedCharacter(value)),
        }
    }
    Ok(tokens)
}
