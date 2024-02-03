use super::token::Token;

/// a value representing a token with extra information like the
/// location of the token  
pub struct Span<'a> {
    tok: Token<'a>,
    start: u64,
    end: u64,
}

pub fn lex(src: &str) -> Vec<Span> {
    todo!()
}

/// A state machine
#[derive(Debug)]
struct Lexer {
    state: LexerState,
}

impl Lexer {
    fn new(state: LexerState) -> Self {
        Self { state }
    }

    // todo: do the actual lexing
}

/// Represents the states that a lexer can be in while lexing
#[derive(Debug)]
enum LexerState {
    /// Is a special state because we need to mark anything as a comment till
    /// a newline
    Comment,

    /// Ordinary state
    Code,
}
