enum ParseError {
    UnterminatedSingleQuote,
    UnterminatedDoubleQuote,
}

enum QuoteState {
    None,
    Single,
    Double,
}

pub enum Token {
    Word(String),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {}
