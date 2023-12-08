use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::line_ending,
    combinator::value,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Movement {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub left: String,
    pub right: String,
}

#[derive(Debug, Clone)]
pub struct MoveSet {
    moves: Vec<Movement>,
    current: usize,
}

impl MoveSet {
    pub fn new(moves: Vec<Movement>) -> Self {
        Self { moves, current: 0 }
    }
}

impl Iterator for MoveSet {
    type Item = Movement;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(self.moves[self.current]);
        self.current = (self.current + 1) % self.moves.len();
        result
    }
}

pub fn my_parser(input: &str) -> IResult<&str, (MoveSet, Vec<Node>)> {
    let (input, movements): (&str, Vec<Movement>) = terminated(
        many1(alt((
            value(Movement::Left, tag("L")),
            value(Movement::Right, tag("R")),
        ))),
        line_ending,
    )(input)?;
    let (input, _) = line_ending(input)?;

    let (input, nodes): (&str, Vec<Node>) = separated_list1(
        line_ending,
        separated_pair(
            take(3usize),
            tag(" = "),
            separated_pair(
                preceded(tag("("), take(3usize)),
                tag(", "),
                terminated(take(3usize), tag(")")),
            ),
        )
        .map(|(name, (left, right)): (&str, (&str, &str))| Node {
            name: name.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        }),
    )(input)?;
    Ok((input, (MoveSet::new(movements), nodes)))
}
