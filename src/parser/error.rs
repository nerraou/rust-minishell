#[derive(Debug)]
pub enum ParseError {
    UnexpectedProgram(String),
    ExpectedRedirectionTarget,
}
