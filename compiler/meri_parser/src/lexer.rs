use std::{iter::Peekable, marker::PhantomData, str::Chars};

use nom::AsChar;

use crate::{span::Span, token::Token};

use super::token::TokenType;

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
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    }
                }

                ')' => {
                    return Token {
                        typ: TokenType::RParen,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    }
                }

                '{' => {
                    return Token {
                        typ: TokenType::LBrace,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    }
                }

                '}' => {
                    return Token {
                        typ: TokenType::RBrace,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    }
                }

                ':' => {
                    return Token {
                        typ: TokenType::Colon,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    }
                }

                ',' => {
                    return Token {
                        typ: TokenType::Comma,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    }
                }

                '.' => {
                    return Token {
                        typ: TokenType::Dot,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    }
                }

                '+' => {
                    return Token {
                        typ: TokenType::Plus,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
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
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    };
                }

                '*' => {
                    return Token {
                        typ: TokenType::Star,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    }
                }

                '%' => {
                    return Token {
                        typ: TokenType::Percent,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    }
                }

                '/' => {
                    return Token {
                        typ: TokenType::Lparen,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    }
                }

                '\\' => {
                    return Token {
                        typ: TokenType::BackSlash,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    }
                }

                '=' => {
                    return Token {
                        typ: TokenType::Equal,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    };
                }

                '>' => {
                    return Token {
                        typ: TokenType::RAngleBracket,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    };
                }

                '<' => {
                    return Token {
                        typ: TokenType::LAngleBracket,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    };
                }

                '!' => {
                    return Token {
                        typ: TokenType::Exclam,
                        span: Span::new(self.tok_id(), self.tok_id() + 1),
                    };
                }

                '|' => {
                    return Token {
                        typ: TokenType::Vbar,
                        span: Span::new(self.tok_id(), self.tok_id() + 2),
                    };
                }

                '&' => {
                    return Token {
                        typ: TokenType::Amper,
                        span: Span::new(self.tok_id(), self.tok_id() + 2),
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
            typ: TokenType::Comment(&self.input[start..self.tok_id()]),
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
    c.is_alphanum() || c == '_'
}

struct TokenIter<'a> {
    dat: PhantomData<&'a ()>,
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
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
                typ: TokenType::Ident("module"),
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
    fn punctuation() {
        let inp = "<>!{}wow%&";
        let mut lexer = Lexer::new(inp);

        loop {
            let tok = lexer.advance_token();
            println!("{tok:?}");
            if tok.typ == TokenType::EOF {
                break;
            }
        }

        assert!(false)
    }

    #[test]
    fn negative_int() {
        let inp = "-23";
        let mut lexer = Lexer::new(inp);

        loop {
            let tok = lexer.advance_token();
            println!("{tok:?}");
            if tok.typ == TokenType::EOF {
                break;
            }
        }

        assert!(false)
    }

    #[test]
    fn positive_int() {
        let inp = "23";
        let mut lexer = Lexer::new(inp);

        loop {
            let tok = lexer.advance_token();
            println!("{tok:?}");
            if tok.typ == TokenType::EOF {
                break;
            }
        }

        assert!(false)
    }

    #[test]
    fn negative_float() {
        let inp = "-23.9";
        let mut lexer = Lexer::new(inp);

        loop {
            let tok = lexer.advance_token();
            println!("{tok:?}");
            if tok.typ == TokenType::EOF {
                break;
            }
        }

        assert!(false)
    }

    #[test]
    fn positive_float() {
        let inp = "23.3";
        let mut lexer = Lexer::new(inp);

        loop {
            let tok = lexer.advance_token();
            println!("{tok:?}");
            if tok.typ == TokenType::EOF {
                break;
            }
        }

        assert!(false)
    }

    #[test]
    fn comments() {
        let inp = r#"-- TODO: Add parsing for numbers
        parse = (x: String) => { } 
        "#;
        let mut lexer = Lexer::new(inp);

        loop {
            let tok = lexer.advance_token();
            println!("{tok:?}");
            if tok.typ == TokenType::EOF {
                break;
            }
        }

        assert!(false)
    }
}
