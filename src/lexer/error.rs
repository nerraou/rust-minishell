#[derive(Debug)]
pub enum LexerError {
    UnclosedQuote(char),
    UnexpectedCharacter(char),
}
