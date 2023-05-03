use std::fmt::Display;

/// This is a list of all the tokens that are recognized in the Meri language
#[derive(Debug)]
pub enum Token<'a> {
    /// An identifier. The value for a name
    Ident(&'a str),
    /// Token for an Integer
    Integer(i64),
    /// Token for floating point number
    Number(f64),
    /// Token for a string literal, represented as a set of characters delimited by quotes
    String(&'a str),
    /// Token for Comments. These will be filtered out during lexing
    Comment(&'a str),
    /// Token for a left parenthesis `(`
    Lparen,
    /// Token for a right parenthesis `)`
    RParen,
    /// Token for a right parenthesis `:`
    Colon,
    /// Token for a left braces `{`
    LBrace,
    /// Token for a right braces `}`
    RBrace,
    /// Token for a Fat arrow `=>`
    FatArrow,
    /// Token for a single equal sign `=`
    Equal,
    /// Token for a comma `,`
    Comma,
    /// Token for a dot `.`
    Dot,
    /// Token for plus sign `+`
    Plus,
    /// Token for the minus sign `-`
    Minus,
    /// Token for star sign `*`
    Star,
    /// Token for the slash  `/`
    Slash,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(ident) => {
                write!(f, "{ident}")
            }
            Token::Integer(int) => {
                write!(f, "{int}")
            }
            Token::Number(num) => {
                write!(f, "{num}")
            }
            Token::String(val) => {
                write!(f, "\"{val}\"")
            }
            Token::Comment(comment) => {
                write!(f, "\"{comment}\"")
            }
            Token::Lparen => {
                write!(f, "(")
            }
            Token::RParen => {
                write!(f, ")")
            }
            Token::Colon => {
                write!(f, ":")
            }
            Token::LBrace => {
                write!(f, "{{")
            }
            Token::RBrace => {
                write!(f, "}}")
            }
            Token::FatArrow => {
                write!(f, "=>")
            }
            Token::Equal => {
                write!(f, "=")
            }
            Token::Comma => {
                write!(f, ",")
            }
            Token::Dot => {
                write!(f, ".")
            }
            Token::Plus => {
                write!(f, "+")
            }
            Token::Minus => {
                write!(f, "-")
            }
            Token::Star => {
                write!(f, "*")
            }
            Token::Slash => {
                write!(f, "/")
            }
        }
    }
}
