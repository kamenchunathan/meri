#![allow(unused)]

use meri_ast::{Definition, Expression, FunctionSignature, Ident, Pattern, TypePath};
use nom::{
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
    // FunctionDefinition {
    //     ident: &'a str,
    //     parameters: Vec<&'a str>,
    //     body: Expression,
    // },
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

fn parse_function_parameter<'a, 'b>(
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

fn parse_function_inputs<'a, 'b>(
    input: &'a [Token<'b>],
) -> IResult<
    &'a [Token<'b>],
    Vec<(Pattern<'b>, Option<TypePath<'b>>)>,
    nom::error::Error<&'a [Token<'b>]>,
> {
    delimited(
        token_type(TokenType::Lparen),
        separated_list1(token_type(TokenType::Comma), parse_function_parameter),
        token_type(TokenType::RParen),
    )(input)
}

fn parse_function_signature<'a, 'b>(
    input: &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], FunctionSignature<'b>, nom::error::Error<&'a [Token<'b>]>> {
    let (input, _) = token_type(TokenType::Colon)(input)?;
    let (input, func_inputs) = parse_function_inputs(input)?;
    // println!("{input:#?}\n\n\n\n\n\n");
    let (input, _) = token_type(TokenType::Equal)(input)?;
    let (input, _) = token_type(TokenType::RAngleBracket)(input)?;
    let (input, return_type) = parse_type_path(input)?;

    Ok((
        input,
        FunctionSignature {
            inputs: func_inputs,
            return_type,
        },
    ))
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
    fn func_def_with_single_input_and_return() {
        let tokens = tokenize("id : (x: Number)  => Number =  { x }").collect::<Vec<_>>();
        let res = parse_function_definition(&tokens);

        println!("{res:#?}");
        assert!(false);
    }

    #[test]
    fn parse_func_def() {
        let tokens =
            tokenize("add : (x: Number, y: Number ) => Number =  { x }").collect::<Vec<_>>();
        let res = parse_function_definition(&tokens);

        println!("{res:#?}");
        assert!(false);
    }

    #[test]
    fn func_param_parsing() {
        let tokens: Vec<_> = tokenize(" x => String ").collect();
        let res = parse_function_inputs(&tokens);
        println!("{res:#?}");
        assert!(false);
    }

    #[test]
    fn multiple_func_param_without_typepath() {
        let tokens: Vec<_> = tokenize(" x, y  = ").collect();
        let res = parse_function_inputs(&tokens);
        println!("{res:#?}");
        assert!(false);
    }

    #[test]
    fn func_param_without_typepath() {
        let tokens: Vec<_> = tokenize(" x, y  = ").collect();
        let res = parse_function_parameter(&tokens);
        println!("{res:#?}");
        assert!(false);
    }

    #[test]
    fn func_signature_with_types() {
        let tokens: Vec<_> = tokenize(": x  => String = {}").collect();
        let res = parse_function_signature(&tokens);
        println!("{res:#?}");
        assert!(false);
    }
}
