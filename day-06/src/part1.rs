use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace1},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Race {
    pub time: u32,
    pub distance: u32,
}

pub fn my_parser(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times): (&str, Vec<u32>) = preceded(
        tuple((tag("Time:"), multispace1)),
        separated_list1(multispace1, complete::u32),
    )(input)?;
    let (input, _) = line_ending(input)?;
    let (input, distances): (&str, Vec<u32>) = preceded(
        tuple((tag("Distance:"), multispace1)),
        separated_list1(multispace1, complete::u32),
    )(input)?;
    let races: Vec<Race> = times
        .iter()
        .zip(distances)
        .map(|(&time, distance)| Race { time, distance })
        .collect();
    Ok((input, races))
}

impl Race {
    pub fn alternatives(&self) -> Vec<u32> {
        (1..self.time)
            .into_iter()
            .map(|hold| (self.time - hold) * hold)
            .collect::<Vec<u32>>()
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let (_, races) = my_parser(input).unwrap();
    let result: usize = races
        .iter()
        .map(|race| {
            let alternatives = race.alternatives();
            alternatives
                .iter()
                .filter(|&alternative| *alternative > race.distance)
                // .inspect(|f| println!("race:{:?}, alt: {:?}", race, f))
                .count()
        })
        .product();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", process(input)?);
        Ok(())
    }
}
