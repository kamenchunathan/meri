use std::fmt::{write, Display};

use crate::span::Span;

/// A representation of a token
#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub typ: TokenType<'a>,
    pub span: Span,
}

/// A set of all the tokens that are recognized in the Meri language
#[derive(Debug, PartialEq)]
pub enum TokenType<'a> {
    /// An identifier. The value for a name
    Ident(&'a str),
    /// Token for an Integer
    IntegerLit(i64),
    /// Token for floating point number
    FloatLit(f64),
    /// Token for a string literal, represented as a set of characters delimited by quotes
    StringLit(&'a str),
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
    /// Token for Left angle bracket / less than sign `<`
    LAngleBracket,
    /// Token for right angle bracket / grater than sign `>`
    RAngleBracket,

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
    /// Token for percent sign `%`
    Percent,
    ///Token for a vertical bar '|'
    Vbar,
    ///Token for an ampersand '&'
    Amper,
    ///Token for logical not '!'
    Exclam,

    /// Token for the slash  `/`
    Slash,
    /// Token for the slash  `\`
    BackSlash,

    // Keywords
    /// Token for the `type` keyword
    Type,
    /// Token for the `alias` keyword
    TypeAlias,

    /// EOF
    // Not a token but should signal the end of parsing
    EOF,
}

impl<'a> Display for TokenType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;

        match self {
            Ident(ident) => {
                write!(f, "{ident}")
            }
            IntegerLit(int) => {
                write!(f, "{int}")
            }
            FloatLit(num) => {
                write!(f, "{num}")
            }
            StringLit(val) => {
                write!(f, "\"{val}\"")
            }
            Comment(comment) => {
                write!(f, "\"{comment}\"")
            }

            Lparen => {
                write!(f, "(")
            }
            RParen => {
                write!(f, ")")
            }
            Colon => {
                write!(f, ":")
            }
            LBrace => {
                write!(f, "{{")
            }
            RBrace => {
                write!(f, "}}")
            }
            LAngleBracket => {
                write!(f, "<")
            }
            RAngleBracket => {
                write!(f, ">")
            }

            Equal => {
                write!(f, "=")
            }
            Comma => {
                write!(f, ",")
            }
            Dot => {
                write!(f, ".")
            }
            Plus => {
                write!(f, "+")
            }
            Minus => {
                write!(f, "-")
            }
            Star => {
                write!(f, "*")
            }
            Percent => {
                write!(f, "%")
            }
            Vbar => {
                write!(f, "|")
            }
            Amper => {
                write!(f, "&")
            }

            Exclam => {
                write!(f, "!")
            }

            Slash => {
                write!(f, "/")
            }

            BackSlash => write!(f, "\\"),

            Type => {
                write!(f, "type")
            }
            TypeAlias => {
                write!(f, "alias")
            }

            EOF => write!(f, "EOF"),
        }
    }
}
