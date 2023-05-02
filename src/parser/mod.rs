#![allow(unused)]

use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{alpha1, char},
    IResult, Parser,
};
use nom_supreme::parser_ext::ParserExt;
use nom_supreme::{
    error::{ErrorTree, GenericErrorTree},
    tag::complete::tag,
};

#[derive(Debug)]
pub enum Definition {
    TypeDefinition,
    FunctionDefinition,
}

#[derive(Debug)]
pub enum Expression {}

fn is_valid_ident_char(inp: char) -> bool {
    inp.is_alphanumeric() || inp == '_' || inp == '\''
}

fn ident(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    // A valid identifier starts with an underscore or letter
    alt((alpha1, tag("_")))
        .context("First Character of an an identifier")
        .parse(input)?;

    let (input, ident) = take_while1(is_valid_ident_char)(input)?;

    Ok((input, ident))
}

fn function(input: &str) -> IResult<&str, &str> {
    todo!()
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
        assert_matches!(ident("wow"), Ok(("", "wow")));
        assert_matches!(ident("_ping"), Ok(("", "_ping")));
        assert_matches!(ident("_ping'"), Ok(("", "_ping'")));
        assert_matches!(
            ident("3ping"),
            Err(nom::Err::Error(GenericErrorTree::Stack { .. }))
        );
    }
}
