#![allow(unused)]

use meri_ast::{Definition, Expression, FunctionSignature, Ident, Pattern, TypePath};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    error::{ErrorKind, ParseError},
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};
use nom_supreme::ParserExt;

use crate::{
    span::Span,
    token::{Token, TokenType},
};

mod lexer;
mod span;
mod token;

fn parse_function_definition<'a, 'b>(
    input: &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], Definition<'b>, nom::error::Error<&'a [Token<'b>]>> {
    let (input, ident) = ident(input)?;
    let (input, sig) = parse_function_signature(input)?;
    let (input, _) = token_type(TokenType::Equal)(input)?;
    let (input, body) = delimited(
        token_type(TokenType::LBrace),
        parse_expr,
        token_type(TokenType::RBrace),
    )(input)?;

    Ok((input, Definition::FunctionDefinition { ident, sig, body }))
}

// TODO: actually parse expressions
fn parse_expr<'a, 'b>(
    input: &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], Expression, nom::error::Error<&'a [Token<'b>]>> {
    map(ident, |_| Expression::Unit)(input)
}

fn parse_single_parameter<'a, 'b>(
    input: &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], (Pattern<'b>, Option<TypePath<'b>>), nom::error::Error<&'a [Token<'b>]>>
{
    let (input, pattern) = parse_pattern(input)?;
    let Ok((input, _)) = token_type(TokenType::Colon)(input) else {
        return Ok((input, (pattern, None)));
    };
    let (input, typ_path) = parse_type_path(input)?;

    Ok((input, (pattern, Some(typ_path))))
}

fn parse_function_params<'a, 'b>(
    input: &'a [Token<'b>],
) -> IResult<
    &'a [Token<'b>],
    Vec<(Pattern<'b>, Option<TypePath<'b>>)>,
    nom::error::Error<&'a [Token<'b>]>,
> {
    delimited(
        token_type(TokenType::Lparen),
        separated_list1(token_type(TokenType::Comma), parse_single_parameter),
        token_type(TokenType::RParen),
    )(input)
}

fn parse_function_signature<'a, 'b>(
    input: &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], FunctionSignature<'b>, nom::error::Error<&'a [Token<'b>]>> {
    let (input, _) = token_type(TokenType::Colon)(input)?;

    let with_params = |input| {
        let (input, func_params) = parse_function_params(input)?;
        let (input, _) = token_type(TokenType::Equal)(input)?;
        let (input, _) = token_type(TokenType::RAngleBracket)(input)?;
        let (input, return_type) = parse_type_path(input)?;

        Ok((
            input,
            FunctionSignature {
                params: func_params,
                return_type,
            },
        ))
    };

    let without_params = |input| {
        let (input, return_type) = parse_type_path(input)?;

        Ok((
            input,
            FunctionSignature {
                params: Vec::new(),
                return_type,
            },
        ))
    };

    alt((with_params, without_params))(input)
}

// TODO: only parses as identifiers currently. Add other items like generics etc.
fn parse_type_path<'a, 'b>(
    input: &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], TypePath<'b>, nom::error::Error<&'a [Token<'b>]>> {
    map(ident, |ident| TypePath { ident })(input)
}

/// A  pattern used in match statements and in binding fucntion arguments
// TODO: currently only handles binding to a name and not destructuring
//      add destructuring and biding with more complex pattern matching
fn parse_pattern<'a, 'b>(
    input: &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], Pattern<'b>, nom::error::Error<&'a [Token<'b>]>> {
    map(ident, |b| Pattern::Binding(b))(input)
}

fn token_type<'a, 'b: 'a>(
    tok_typ: TokenType<'_>,
) -> impl Fn(
    &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], Token<'b>, nom::error::Error<&'a [Token<'b>]>>
       + '_ {
    return move |input| match input.split_first() {
        None => Err(nom::Err::Error(nom::error::Error::from_error_kind(
            input,
            nom::error::ErrorKind::Eof,
        ))),

        Some((t @ Token { typ, .. }, rest)) if *typ == tok_typ => Ok((rest, *t)),

        Some((t, rest)) => Err(nom::Err::Error(nom::error::Error::from_error_kind(
            input,
            ErrorKind::Tag,
        ))),
    };
}

fn ident<'a, 'b>(
    input: &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], Ident<'b>, nom::error::Error<&'a [Token<'b>]>> {
    match input.split_first() {
        None => Err(nom::Err::Error(nom::error::Error::from_error_kind(
            input,
            nom::error::ErrorKind::Eof,
        ))),

        Some((
            Token {
                typ: TokenType::Ident(ident),
                ..
            },
            rest,
        )) => Ok((rest, Ident(ident))),

        Some((t, rest)) => Err(nom::Err::Error(nom::error::Error::from_error_kind(
            input,
            ErrorKind::Tag,
        ))),
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    use super::*;

    #[test]
    fn func_def_typed() {
        let tokens = tokenize("id : (x: Number)  => Number =  { x }").collect::<Vec<_>>();
        let res = parse_function_definition(&tokens);

        assert_eq!(
            res,
            Ok((
                &[Token {
                    typ: TokenType::EOF,
                    span: Span::new(35, 35)
                }][..],
                Definition::FunctionDefinition {
                    ident: Ident("id"),
                    sig: FunctionSignature {
                        params: vec![(
                            Pattern::Binding(Ident("x")),
                            Some(TypePath {
                                ident: Ident("Number")
                            })
                        )],
                        return_type: TypePath {
                            ident: Ident("Number")
                        }
                    },
                    body: Expression::Unit
                }
            ))
        )
    }

    #[test]
    fn func_single_param() {
        let tokens: Vec<_> = tokenize("(x)").collect();
        let res = parse_function_params(&tokens);
        assert_eq!(
            res,
            Ok((
                &[Token {
                    typ: TokenType::EOF,
                    span: Span::new(2, 2)
                }][..],
                vec![(Pattern::Binding(Ident("x")), None)]
            ))
        );
    }

    #[test]
    fn func_single_param_with_typepath() {
        let tokens: Vec<_> = tokenize("(x: String)").collect();
        let res = parse_function_params(&tokens);
        assert_eq!(
            res,
            Ok((
                &[Token {
                    typ: TokenType::EOF,
                    span: Span::new(10, 10)
                }][..],
                vec![(
                    Pattern::Binding(Ident("x")),
                    Some(TypePath {
                        ident: Ident("String")
                    })
                )]
            ))
        );
    }

    #[test]
    fn func_mutliple_param() {
        let tokens: Vec<_> = tokenize("(x, y, z)").collect();
        let res = parse_function_params(&tokens);
        assert_eq!(
            res,
            Ok((
                &[Token {
                    typ: TokenType::EOF,
                    span: Span::new(8, 8)
                }][..],
                vec![
                    (Pattern::Binding(Ident("x")), None),
                    (Pattern::Binding(Ident("y")), None),
                    (Pattern::Binding(Ident("z")), None)
                ],
            ))
        );
    }

    #[test]
    fn func_mutliple_param_with_typepaths() {
        let tokens: Vec<_> = tokenize("(x: Int, y: Int, z: Int)").collect();
        let res = parse_function_params(&tokens);
        assert_eq!(
            res,
            Ok((
                &[Token {
                    typ: TokenType::EOF,
                    span: Span::new(23, 23)
                }][..],
                vec![
                    (
                        Pattern::Binding(Ident("x")),
                        Some(TypePath {
                            ident: Ident("Int")
                        })
                    ),
                    (
                        Pattern::Binding(Ident("y")),
                        Some(TypePath {
                            ident: Ident("Int")
                        })
                    ),
                    (
                        Pattern::Binding(Ident("z")),
                        Some(TypePath {
                            ident: Ident("Int")
                        })
                    )
                ],
            ))
        );
    }

    #[test]
    fn func_sig_constant() {
        let tokens: Vec<_> = tokenize(": String").collect();
        let (tokens, signature) = parse_function_signature(&tokens).unwrap();
        assert_eq!(
            signature,
            FunctionSignature {
                params: vec![],
                return_type: TypePath {
                    ident: Ident("String")
                }
            }
        );
    }

    #[test]
    fn func_signature_with_return() {
        let tokens: Vec<_> = tokenize(":(x) => String = {}").collect();
        let res = parse_function_signature(&tokens);
        assert_eq!(
            res,
            Ok((
                &[
                    Token {
                        typ: TokenType::Equal,
                        span: Span { start: 15, end: 15 },
                    },
                    Token {
                        typ: TokenType::LBrace,
                        span: Span { start: 17, end: 17 },
                    },
                    Token {
                        typ: TokenType::RBrace,
                        span: Span { start: 18, end: 18 },
                    },
                    Token {
                        typ: TokenType::EOF,
                        span: Span { start: 18, end: 18 },
                    },
                ][..],
                FunctionSignature {
                    params: vec![(Pattern::Binding(Ident("x",)), None)],
                    return_type: TypePath {
                        ident: Ident("String",),
                    },
                },
            ),)
        );
    }

    #[test]
    fn func_signature_typed_with_return() {
        let tokens: Vec<_> = tokenize(":(x: String) => String = {}").collect();
        let res = parse_function_signature(&tokens);
        assert_eq!(
            res,
            Ok((
                &[
                    Token {
                        typ: TokenType::Equal,
                        span: Span { start: 23, end: 23 },
                    },
                    Token {
                        typ: TokenType::LBrace,
                        span: Span { start: 25, end: 25 },
                    },
                    Token {
                        typ: TokenType::RBrace,
                        span: Span { start: 26, end: 26 },
                    },
                    Token {
                        typ: TokenType::EOF,
                        span: Span { start: 26, end: 26 },
                    },
                ][..],
                FunctionSignature {
                    params: vec![(
                        Pattern::Binding(Ident("x",)),
                        Some(TypePath {
                            ident: Ident("String")
                        })
                    )],
                    return_type: TypePath {
                        ident: Ident("String",),
                    },
                },
            ),)
        );
    }
}
