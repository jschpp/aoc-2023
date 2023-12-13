use std::usize;

use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};

pub fn parse_input(input: &str) -> IResult<&str, Vec<Vec<String>>> {
    separated_list1(
        tuple((line_ending, line_ending)),
        separated_list1(
            line_ending,
            many1(one_of("#.")).map(|x| x.iter().collect::<String>()),
        ),
    )(input)
}

pub fn rotate(block: &[String]) -> Vec<String> {
    let mut result: Vec<Vec<char>> = vec![vec!['\0'; block.len()]; block[0].len()];
    for (line_idx, line) in block.iter().enumerate() {
        for (char_idx, c) in line.chars().enumerate() {
            result[char_idx][line_idx] = c;
        }
    }
    let result: Vec<String> = result
        .into_iter()
        .map(|x| x.iter().collect::<String>())
        .collect();
    result
}

pub fn find_symmetry(block: &[String], number_of_smudges: usize) -> usize {
    find_symmetry_fold(block, number_of_smudges)
}

pub fn find_symmetry_map(block: &[String], number_of_smudges: usize) -> usize {
    for x in 1..block.len() {
        let above = &block[..x];
        let below = &block[x..];

        if below
            .iter()
            .zip(above.iter().rev())
            .map(|(row_below, row_above)| {
                row_below
                    .chars()
                    .zip(row_above.chars())
                    .map(
                        |(below_char, above_char)| {
                            if below_char == above_char {
                                0
                            } else {
                                1
                            }
                        },
                    )
                    .sum::<usize>()
            })
            .sum::<usize>()
            == number_of_smudges
        {
            return x;
        }
    }
    0
}

pub fn find_symmetry_fold(block: &[String], number_of_smudges: usize) -> usize {
    for x in 1..block.len() {
        let above = &block[..x];
        let below = &block[x..];

        if below
            .iter()
            .zip(above.iter().rev())
            .fold(0, |line_acc: usize, lines| {
                line_acc
                    + lines
                        .0
                        .chars()
                        .zip(lines.1.chars())
                        .fold(0, |acc: usize, chars| {
                            acc + (if chars.0 == chars.1 { 0 } else { 1 })
                        })
            })
            == number_of_smudges
        {
            return x;
        }
    }
    0
}
