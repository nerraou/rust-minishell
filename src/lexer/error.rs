#[derive(Debug)]
pub enum LexerError {
    UnterminatedSingleQuote,
    UnterminatedDoubleQuote,
}
