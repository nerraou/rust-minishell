#[derive(Debug)]
pub enum QuoteState {
    None,
    Single,
    Double,
}
#[derive(Debug)]
pub struct WordState {
    pub text: String,
    pub quote: QuoteState,
}
#[derive(Debug)]
pub enum Token {
    Word(WordState),
    Pipe,
    RedirectIn,
    RedirectOut,
    RedirectAppend,
    Heredoc,
}
