use core::panic;
use std::{iter::Peekable, marker::PhantomData, str::Chars};

use crate::{
    span::Span,
    token::{try_into_keyword, Token, TokenType},
};

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut lexer = Lexer::new(input);
    let mut done = false;

    std::iter::from_fn(move || {
        if done {
            None
        } else {
            match lexer.advance_token() {
                t if t.typ == TokenType::EOF => {
                    done = true;
                    Some(t)
                }
                t => Some(t),
            }
        }
    })
}

#[derive(Debug)]
struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    idx: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars().peekable(),
            idx: 0,
        }
    }

    fn consume_char(&mut self) {
        self.advance_char();
    }

    fn advance_char(&mut self) -> Option<char> {
        let next = self.chars.next();
        if let Some(_) = next {
            self.idx += 1
        }

        next
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn tok_id(&self) -> usize {
        self.idx - 1
    }

    fn advance_token(&mut self) -> Token<'a> {
        loop {
            let Some(next) = self.advance_char() else {
                return Token {
                    typ: TokenType::EOF,
                    span: Span {
                        start: self.tok_id(),
                        end: self.tok_id(),
                    },
                };
            };

            match next {
                '(' => {
                    return Token {
                        typ: TokenType::Lparen,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    }
                }

                ')' => {
                    return Token {
                        typ: TokenType::RParen,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    }
                }

                '{' => {
                    return Token {
                        typ: TokenType::LBrace,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    }
                }

                '}' => {
                    return Token {
                        typ: TokenType::RBrace,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    }
                }

                ':' => {
                    return Token {
                        typ: TokenType::Colon,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    }
                }

                ',' => {
                    return Token {
                        typ: TokenType::Comma,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    }
                }

                '.' => {
                    return Token {
                        typ: TokenType::Dot,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    }
                }

                '+' => {
                    return Token {
                        typ: TokenType::Plus,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    }
                }

                '-' => {
                    if let Some('-') = self.peek_char() {
                        return self.consume_comment();
                    }

                    if let Some(t) = self.peek_char() {
                        if t.is_digit(10) {
                            return self.consume_number();
                        }
                    }

                    return Token {
                        typ: TokenType::Minus,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    };
                }

                '*' => {
                    return Token {
                        typ: TokenType::Star,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    }
                }

                '%' => {
                    return Token {
                        typ: TokenType::Percent,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    }
                }

                '/' => {
                    return Token {
                        typ: TokenType::Lparen,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    }
                }

                '\\' => {
                    return Token {
                        typ: TokenType::BackSlash,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    }
                }

                '=' => {
                    return Token {
                        typ: TokenType::Equal,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    };
                }

                '>' => {
                    return Token {
                        typ: TokenType::RAngleBracket,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    };
                }

                '<' => {
                    return Token {
                        typ: TokenType::LAngleBracket,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    };
                }

                '!' => {
                    return Token {
                        typ: TokenType::Exclam,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    };
                }

                '|' => {
                    return Token {
                        typ: TokenType::Vbar,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    };
                }

                '&' => {
                    return Token {
                        typ: TokenType::Amper,
                        span: Span::new(self.tok_id(), self.tok_id()),
                    };
                }

                // Identifiers
                'a'..='z' | 'A'..='Z' | '_' => {
                    let start = self.tok_id();
                    let mut len = 0;

                    while let Some(next) = self.peek_char() {
                        if is_valid_ident_char(next) {
                            self.consume_char();
                        } else {
                            break;
                        }
                    }

                    if let Some(typ) = try_into_keyword(&self.input[start..=self.tok_id()]) {
                        return Token {
                            typ,
                            span: Span::new(start, self.tok_id()),
                        };
                    }

                    return Token {
                        typ: TokenType::Ident(&self.input[start..=self.tok_id()]),
                        span: Span::new(start, self.tok_id()),
                    };
                }

                // NOTE: Missing support for full number parsing
                //      numbers in bases other than decimal
                //      exponential notation
                '0'..='9' => {
                    return self.consume_number();
                }

                // String literals
                '"' => {
                    panic!("Unhandled: Tokenization of string literals")
                }

                // Skip Whitespace
                t if t.is_whitespace() => {
                    continue;
                }

                _ => {
                    return Token {
                        typ: TokenType::EOF,
                        span: Span { start: 0, end: 0 },
                    }
                }
            }
        }
    }

    fn consume_comment(&mut self) -> Token<'a> {
        let start = self.tok_id();
        while let Some(c) = self.peek_char() {
            if c != '\n' {
                self.consume_char()
            } else {
                break;
            }
        }

        return Token {
            typ: TokenType::Comment(&self.input[start..self.tok_id() + 1]),
            span: Span::new(start, self.tok_id()),
        };
    }

    fn consume_number(&mut self) -> Token<'a> {
        let start = self.tok_id();
        let mut found_decimal_point = false;

        loop {
            match self.peek_char() {
                Some(t) if t.is_digit(10) => {
                    self.consume_char();
                }

                Some('.') => {
                    if !found_decimal_point {
                        found_decimal_point = true;
                        self.consume_char();
                    } else {
                        break;
                    }
                }

                _ => break,
            }
        }

        if found_decimal_point {
            // parse float
            let parsed_float =
                (&self.input[start..=self.tok_id()])
                    .parse::<f64>()
                    .expect(&format!(
                        "Error in parsing float: '{}'",
                        &self.input[start..=self.tok_id()],
                    ));
            return Token {
                typ: TokenType::FloatLit(parsed_float),
                span: Span::new(start, self.tok_id()),
            };
        } else {
            // parse int
            let parsed_int = (&self.input[start..=self.tok_id()])
                .parse::<i64>()
                .expect(&format!(
                    "Error in parsing int: {}",
                    &self.input[start..=self.tok_id()],
                ));
            return Token {
                typ: TokenType::IntegerLit(parsed_int),
                span: Span::new(start, self.tok_id()),
            };
        }
    }
}

fn is_valid_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        span::Span,
        token::{Token, TokenType},
    };

    use super::tokenize;

    #[test]
    fn single_ident() {
        let inp = "func";
        let mut lexer = Lexer::new(inp);

        assert_eq!(
            lexer.advance_token(),
            Token {
                typ: TokenType::Ident("func"),
                span: Span::new(0, 3)
            }
        );
        assert_eq!(
            lexer.advance_token(),
            Token {
                typ: TokenType::EOF,
                span: Span::new(3, 3)
            }
        );
    }

    #[test]
    fn space_separated_idents() {
        let inp = "module Main";
        let mut lexer = Lexer::new(inp);

        assert_eq!(
            lexer.advance_token(),
            Token {
                typ: TokenType::Module,
                span: Span::new(0, 5)
            }
        );
        assert_eq!(
            lexer.advance_token(),
            Token {
                typ: TokenType::Ident("Main"),
                span: Span::new(7, 10)
            }
        );
    }

    #[test]
    fn keywords() {
        let inp = "module type typealias exposing import";
        let tokens = tokenize(inp).collect::<Vec<_>>();
        assert_eq!(
            tokens,
            [
                Token {
                    typ: TokenType::Module,
                    span: Span::new(0, 5)
                },
                Token {
                    typ: TokenType::Type,
                    span: Span::new(7, 10)
                },
                Token {
                    typ: TokenType::TypeAlias,
                    span: Span::new(12, 20)
                },
                Token {
                    typ: TokenType::Exposing,
                    span: Span::new(22, 29)
                },
                Token {
                    typ: TokenType::Import,
                    span: Span::new(31, 36)
                },
                Token {
                    typ: TokenType::EOF,
                    span: Span::new(36, 36)
                },
            ]
        );
    }

    #[test]
    fn punctuation() {
        let inp = "<>!{}wow%&";
        let tokens = tokenize(inp).collect::<Vec<_>>();
        assert_eq!(
            tokens[0..tokens.len() - 1],
            [
                Token {
                    typ: TokenType::LAngleBracket,
                    span: Span { start: 0, end: 0 }
                },
                Token {
                    typ: TokenType::RAngleBracket,
                    span: Span { start: 1, end: 1 }
                },
                Token {
                    typ: TokenType::Exclam,
                    span: Span { start: 2, end: 2 }
                },
                Token {
                    typ: TokenType::LBrace,
                    span: Span { start: 3, end: 3 }
                },
                Token {
                    typ: TokenType::RBrace,
                    span: Span { start: 4, end: 4 }
                },
                Token {
                    typ: TokenType::Ident("wow"),
                    span: Span { start: 5, end: 7 }
                },
                Token {
                    typ: TokenType::Percent,
                    span: Span { start: 8, end: 8 }
                },
                Token {
                    typ: TokenType::Amper,
                    span: Span { start: 9, end: 9 }
                },
            ]
        );
    }

    #[test]
    fn negative_int() {
        let inp = "-23";
        let tokens = tokenize(inp).collect::<Vec<_>>();
        assert_eq!(
            tokens[0..tokens.len() - 1],
            [Token {
                typ: TokenType::IntegerLit(-23),
                span: Span { start: 0, end: 2 }
            }]
        );
    }

    #[test]
    fn positive_int() {
        let inp = "23";
        let tokens = tokenize(inp).collect::<Vec<_>>();
        assert_eq!(
            tokens[0..tokens.len() - 1],
            [Token {
                typ: TokenType::IntegerLit(23),
                span: Span { start: 0, end: 1 }
            }]
        );
    }

    #[test]
    fn negative_float() {
        let inp = "-23.9";
        let tokens = tokenize(inp).collect::<Vec<_>>();
        assert_eq!(
            tokens[0..tokens.len() - 1],
            [Token {
                typ: TokenType::FloatLit(-23.9),
                span: Span { start: 0, end: 4 }
            }]
        );
    }

    #[test]
    fn positive_float() {
        let inp = "23.3";
        let tokens = tokenize(inp).collect::<Vec<_>>();
        assert_eq!(
            tokens[0..tokens.len() - 1],
            [Token {
                typ: TokenType::FloatLit(23.3),
                span: Span { start: 0, end: 3 }
            }]
        );
    }

    #[test]
    fn func() {
        let inp = r#"sum = (a: Int, b: Int) => { a + b }"#;
        let tokens = tokenize(inp).collect::<Vec<_>>();
        assert_eq!(
            tokens[0..tokens.len() - 1],
            [
                Token {
                    typ: TokenType::Ident("sum"),
                    span: Span { start: 0, end: 2 },
                },
                Token {
                    typ: TokenType::Equal,
                    span: Span { start: 4, end: 4 },
                },
                Token {
                    typ: TokenType::Lparen,
                    span: Span { start: 6, end: 6 },
                },
                Token {
                    typ: TokenType::Ident("a"),
                    span: Span { start: 7, end: 7 },
                },
                Token {
                    typ: TokenType::Colon,
                    span: Span { start: 8, end: 8 },
                },
                Token {
                    typ: TokenType::Ident("Int"),
                    span: Span { start: 10, end: 12 },
                },
                Token {
                    typ: TokenType::Comma,
                    span: Span { start: 13, end: 13 },
                },
                Token {
                    typ: TokenType::Ident("b"),
                    span: Span { start: 15, end: 15 },
                },
                Token {
                    typ: TokenType::Colon,
                    span: Span { start: 16, end: 16 },
                },
                Token {
                    typ: TokenType::Ident("Int"),
                    span: Span { start: 18, end: 20 },
                },
                Token {
                    typ: TokenType::RParen,
                    span: Span { start: 21, end: 21 },
                },
                Token {
                    typ: TokenType::Equal,
                    span: Span { start: 23, end: 23 },
                },
                Token {
                    typ: TokenType::RAngleBracket,
                    span: Span { start: 24, end: 24 },
                },
                Token {
                    typ: TokenType::LBrace,
                    span: Span { start: 26, end: 26 },
                },
                Token {
                    typ: TokenType::Ident("a"),
                    span: Span { start: 28, end: 28 },
                },
                Token {
                    typ: TokenType::Plus,
                    span: Span { start: 30, end: 30 },
                },
                Token {
                    typ: TokenType::Ident("b"),
                    span: Span { start: 32, end: 32 },
                },
                Token {
                    typ: TokenType::RBrace,
                    span: Span { start: 34, end: 34 },
                },
            ]
        );
    }

    #[test]
    fn comments() {
        let inp = r#"-- TODO: Add parsing for numbers
parse = (x: String) => { }"#;
        let tokens = tokenize(inp).collect::<Vec<_>>();

        assert_eq!(
            tokens[0..tokens.len() - 1],
            [
                Token {
                    typ: TokenType::Comment("-- TODO: Add parsing for numbers"),
                    span: Span { start: 0, end: 31 }
                },
                Token {
                    typ: TokenType::Ident("parse"),
                    span: Span { start: 33, end: 37 }
                },
                Token {
                    typ: TokenType::Equal,
                    span: Span { start: 39, end: 39 }
                },
                Token {
                    typ: TokenType::Lparen,
                    span: Span { start: 41, end: 41 }
                },
                Token {
                    typ: TokenType::Ident("x"),
                    span: Span { start: 42, end: 42 }
                },
                Token {
                    typ: TokenType::Colon,
                    span: Span { start: 43, end: 43 }
                },
                Token {
                    typ: TokenType::Ident("String"),
                    span: Span { start: 45, end: 50 }
                },
                Token {
                    typ: TokenType::RParen,
                    span: Span { start: 51, end: 51 }
                },
                Token {
                    typ: TokenType::Equal,
                    span: Span { start: 53, end: 53 }
                },
                Token {
                    typ: TokenType::RAngleBracket,
                    span: Span { start: 54, end: 54 }
                },
                Token {
                    typ: TokenType::LBrace,
                    span: Span { start: 56, end: 56 }
                },
                Token {
                    typ: TokenType::RBrace,
                    span: Span { start: 58, end: 58 }
                },
            ]
        );
    }
}
