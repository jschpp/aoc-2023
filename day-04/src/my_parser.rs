use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub struct Card {
    pub number: u32,
    pub winning: Vec<u32>,
    pub numbers: Vec<u32>,
}

pub fn line_parser(input: &str) -> IResult<&str, Card> {
    let (input, number) = preceded(
        terminated(tag("Card"), multispace1),
        terminated(complete::u32, tuple((tag(":"), multispace1))),
    )(input)?;
    let (input, (winning, numbers)) = separated_pair(
        separated_list1(multispace1, complete::u32),
        tuple((multispace1, tag("|"), multispace1)),
        separated_list1(multispace1, complete::u32),
    )(input)?;
    Ok((
        input,
        Card {
            number,
            winning,
            numbers,
        },
    ))
}

pub fn my_parser(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, line_parser)(input)
}
