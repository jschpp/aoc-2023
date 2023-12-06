use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::value,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub number: u32,
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Blue,
    Red,
    Green,
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub moves: Vec<Vec<Move>>,
    pub max: MaxVal,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct MaxVal {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Game {
    pub fn is_possible(&self, max: MaxVal) -> bool {
        self.max.red <= max.red && self.max.green <= max.green && self.max.blue <= max.blue
    }

    pub fn initialize_max(&mut self) {
        let mut new_max: MaxVal = Default::default();
        for moveset in &self.moves {
            for single_move in moveset {
                match single_move.color {
                    Color::Red => {
                        if single_move.number > new_max.red {
                            new_max.red = single_move.number
                        }
                    }
                    Color::Blue => {
                        if single_move.number > new_max.blue {
                            new_max.blue = single_move.number
                        }
                    }
                    Color::Green => {
                        if single_move.number > new_max.green {
                            new_max.green = single_move.number
                        }
                    }
                }
            }
        }
        self.max = new_max
    }
}

fn color_parser(input: &str) -> IResult<&str, Color> {
    alt((
        value(Color::Red, tag("red")),
        value(Color::Green, tag("green")),
        value(Color::Blue, tag("blue")),
    ))(input)
}

fn move_parser(input: &str) -> IResult<&str, Move> {
    let (input, mv) = separated_pair(complete::u32, tag(" "), color_parser)(input)?;
    Ok((
        input,
        Move {
            number: mv.0,
            color: mv.1,
        },
    ))
}

pub fn line_parser(input: &str) -> IResult<&str, Game> {
    let (input, number): (&str, u32) =
        terminated(preceded(tag("Game "), complete::u32), tag(": "))(input)?;
    let (input, moves): (&str, Vec<Vec<Move>>) =
        separated_list1(tag("; "), separated_list1(tag(", "), move_parser))(input)?;
    Ok((
        input,
        Game {
            id: number,
            moves,
            max: Default::default(),
        },
    ))
}
