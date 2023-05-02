use nom::{
    bytes::complete::{tag, take_while},
    character::complete::char,
    error::ParseError,
    sequence::delimited,
    Compare, IResult, InputIter, InputLength, InputTake, InputTakeAtPosition, Parser,
};

/// Optional Whitespace delimited
pub fn optional_space_delim<I, O, E>(
    parser: impl Parser<I, O, E>,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: InputTakeAtPosition<Item = char>,
    E: nom::error::ParseError<I>,
{
    delimited(
        take_while(|c: char| c == ' '),
        parser,
        take_while(|c: char| c == ' '),
    )
}
