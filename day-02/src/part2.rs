use super::my_parser::{line_parser, Game, MaxVal};
use nom::{character::complete::line_ending, multi::separated_list1};

impl MaxVal {
    fn power(self) -> u32 {
        self.blue * self.green * self.red
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let (_input, mut games): (&str, Vec<Game>) =
        separated_list1(line_ending, line_parser)(input).unwrap();

    let result: u32 = games
        .iter_mut()
        .map(|game| {
            game.initialize_max();
            game.max.power()
        })
        .inspect(|x| println!("{:?}", x))
        .sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
