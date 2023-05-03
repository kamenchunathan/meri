#![allow(unused)]

mod ast;
mod combinators;

use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{alpha1, char},
    combinator::opt,
    multi::separated_list0,
    sequence::delimited,
    IResult, Parser,
};
use nom_supreme::parser_ext::ParserExt;
use nom_supreme::{
    error::{ErrorTree, GenericErrorTree},
    tag::complete::tag,
};

use crate::parser::combinators::optional_space_delim;

#[derive(Debug)]
pub enum Definition<'a> {
    TypeDefinition,
    FunctionDefinition {
        ident: &'a str,
        parameters: Vec<&'a str>,
        body: Expression,
    },
}

#[derive(Debug)]
pub enum Expression {
    Unit,
}

fn is_valid_ident_char(inp: char) -> bool {
    inp.is_alphanumeric() || inp == '_' || inp == '\''
}

/// Parses a valid identifer in the meri language
///
/// A valid identifier starts with an underscore or letter
/// and contains alphabetic characters, numbers, underscores
/// or the apostrophe
fn parse_ident(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    alt((alpha1, tag("_")))
        .context("First Character of an an identifier")
        .parse(input)?;
    let (input, ident) = take_while1(is_valid_ident_char)(input)?;

    Ok((input, ident))
}

/// Parses an expression
fn expression(input: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    opt(optional_space_delim(tag("void")))(input).map(|(inp, res)| {
        (
            inp,
            res.map(|_| Expression::Unit).unwrap_or(Expression::Unit),
        )
    })
}

fn function_body(input: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    delimited(tag("{"), expression, tag("}"))(input)
}

fn parse_function(input: &str) -> IResult<&str, Definition, ErrorTree<&str>> {
    let (input, ident) = parse_ident(input)?;
    let (input, _) = optional_space_delim(tag("="))(input)?;
    let (input, parameters) = delimited(
        tag("("),
        // TODO: add support for annotating types of parameters
        separated_list0(tag(","), optional_space_delim(parse_ident)),
        tag(")"),
    )(input)?;
    let (input, _) = optional_space_delim(tag("=>"))(input)?;
    let (input, body) = optional_space_delim(function_body)(input)?;

    Ok((
        input,
        Definition::FunctionDefinition {
            ident,
            parameters,
            body,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cool_asserts::assert_matches;

    #[test]
    fn functions() {
        let empty_function = "func = () => {}";
    }

    #[test]
    fn ident_parses() {
        assert_matches!(parse_ident("wow"), Ok(("", "wow")));
        assert_matches!(parse_ident("_ping"), Ok(("", "_ping")));
        assert_matches!(parse_ident("_ping'"), Ok(("", "_ping'")));
        assert_matches!(
            parse_ident("3ping"),
            Err(nom::Err::Error(GenericErrorTree::Stack { .. }))
        );
    }

    #[test]
    fn fn_no_params_and_empty_body() {
        let src = "f = () => {}";
        assert_eq!(
            assert_matches!(
                parse_function(src),
                Ok((
                    "",
                    Definition::FunctionDefinition {
                        ident: "f",
                        parameters,
                        body: Expression::Unit
                    }
                )) => parameters
            ),
            Vec::<&str>::new()
        )
    }
}
