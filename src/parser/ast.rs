#[derive(Debug)]
pub enum RedirectionType {
    Input,
    Output,
    Append,
    Heredoc,
}
#[derive(Debug)]
pub struct Redirection {
    pub kind: RedirectionType,
    pub target: String,
}

pub struct Command {
    pub program: String,
    pub arguments: Vec<String>,
    pub redirections: Vec<Redirection>,
}
