use super::my_parser::{line_parser, Game, MaxVal};
use nom::{character::complete::line_ending, multi::separated_list1};

pub fn process(input: &str) -> anyhow::Result<String> {
    let (_input, mut games): (&str, Vec<Game>) =
        separated_list1(line_ending, line_parser)(input).unwrap();

    let test_case = MaxVal {
        red: 12,
        green: 13,
        blue: 14,
    };

    let result: u32 = games
        .iter_mut()
        .filter_map(|game| {
            game.initialize_max();
            if game.is_possible(test_case) {
                println!("{:?}", game);
                Some(game.id)
            } else {
                None
            }
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT: &str = include_str!("../test1.txt");

    #[test]
    fn test_process() -> anyhow::Result<()> {
        assert_eq!("8", process(TESTINPUT)?);
        Ok(())
    }
}
