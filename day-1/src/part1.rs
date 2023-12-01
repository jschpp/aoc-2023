use anyhow::{Context, Result};
use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};

pub fn process(input: &str) -> Result<String> {
    let mut sum: u32 = 0;
    input.lines().for_each(|line| {
        let result = [
            line.chars()
                .find(char::is_ascii_digit)
                .unwrap()
                .to_digit(10)
                .unwrap(),
            line.chars()
                .rfind(char::is_ascii_digit)
                .unwrap()
                .to_digit(10)
                .unwrap(),
        ];
        sum += result[0] * 10 + result[1];
    });
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_part_1() {
        let result = "142";
        assert_eq!(process(INPUT).unwrap(), result)
    }

    #[test]
    fn test_part_1_real_data() {
        let data = include_str!("../input.txt");
        assert_eq!(process(data).unwrap(), "54338")
    }
}
