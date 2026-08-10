use crate::shell::parser::{
    QuoteState::{Double, None, Single},
    Token::Word,
};

#[derive(Debug)]
pub enum ParseError {
    UnterminatedSingleQuote,
    UnterminatedDoubleQuote,
}
#[derive(Debug)]
pub enum QuoteState {
    None,
    Single,
    Double,
}
#[derive(Debug)]
pub struct WordState {
    text: String,
    quote: QuoteState,
}
#[derive(Debug)]
pub enum Token {
    Word(WordState),
    // Pipe,
    // RedirectIn,
    // RedirectOut,
    // RedirectAppend,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut iter = input.chars().peekable();
    while let Some(&value) = iter.peek() {
        if value.is_whitespace() {
            iter.next();
        } else if value == '\'' {
            iter.next();
            let mut word = String::new();
            while let Some(c) = iter.next() {
                if c == '\'' {
                    tokens.push(Word(WordState {
                        text: word,
                        quote: Single,
                    }));
                    break;
                }
                word.push(c);
            }
        } else if value == '\"' {
            iter.next();
            let mut word = String::new();
            while let Some(c) = iter.next() {
                if c == '\"' {
                    tokens.push(Word(WordState {
                        text: word,
                        quote: Double,
                    }));
                    break;
                }
                word.push(c);
            }
        } else if value.is_alphabetic() {
            let mut word = String::new();
            while let Some(c) = iter.next() {
                if c.is_whitespace() {
                    break;
                }
                word.push(c);
            }
            tokens.push(Word(WordState {
                text: word,
                quote: None,
            }));
        }
    }

    Ok(tokens)
}
