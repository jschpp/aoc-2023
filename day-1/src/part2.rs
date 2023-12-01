use anyhow::{Context, Result};
use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};

fn numberword(input: &str) -> IResult<&str, u32> {
    let (rest, number): (&str, u32) = alt((
        value(1, alt((tag("1"), tag("one")))),
        value(2, alt((tag("2"), tag("two")))),
        value(3, alt((tag("3"), tag("three")))),
        value(4, alt((tag("4"), tag("four")))),
        value(5, alt((tag("5"), tag("five")))),
        value(6, alt((tag("6"), tag("six")))),
        value(7, alt((tag("7"), tag("seven")))),
        value(8, alt((tag("8"), tag("eight")))),
        value(9, alt((tag("9"), tag("nine")))),
    ))(input)?;
    Ok((rest, number))
}

const NEEDLE: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];
fn parse_line(line: &str) -> u32 {
    let (_, left) = NEEDLE
        .iter()
        .filter_map(|needle| {
            let matches: Vec<(usize, &str)> = line.match_indices(needle).collect();
            if !matches.is_empty() {
                Some(matches[0])
            } else {
                None
            }
        })
        .min_by_key(|x| x.0)
        .context("Finding first number")
        .expect("there should be at least one number");
    let (_, left) = numberword(left).expect("All Needles are known");

    // since the words can be overlapping a rmatch is needed to find the last number
    let (_, right) = NEEDLE
        .iter()
        .filter_map(|needle| {
            let matches: Vec<(usize, &str)> = line.rmatch_indices(needle).collect();
            if !matches.is_empty() {
                Some(matches[0])
            } else {
                None
            }
        })
        .max_by_key(|x| x.0)
        .context("Finding last number")
        .expect("there should be at least one number");
    let (_, right) = numberword(right).expect("All Needles are known");
    left * 10 + right
}

pub fn process_part2(input: &str) -> Result<String> {
    Ok(input.lines().map(parse_line).sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_part_2() {
        let input2 = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
eighthree";
        let result = "364";
        assert_eq!(process_part2(input2).unwrap(), result)
    }

    #[test]
    fn test_part_2_real_data() {
        let data = include_str!("../input.txt");
        assert_eq!(process_part2(data).unwrap(), "53389")
    }
}
