#[derive(Debug)]
pub struct Race {
    pub time: usize,
    pub distance: usize,
}

impl Race {
    pub fn alternatives(&self) -> Vec<usize> {
        (1..self.time)
            .into_iter()
            .map(|hold| (self.time - hold) * hold)
            .collect::<Vec<usize>>()
    }
}

fn my_parser(input: &str) -> Race {
    let r: Vec<usize> = input
        .lines()
        .map(|x| {
            let digit = x.chars().filter(|x| x.is_ascii_digit()).collect::<String>();
            digit.parse::<usize>().expect("Ascii Digits")
        })
        .collect();
    assert!(r.len() == 2, "wrong parsing");
    Race {
        time: r[0],
        distance: r[1],
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let race = my_parser(input);
    let alternatives = race.alternatives();
    let result: usize = alternatives
        .iter()
        .filter(|&alternative| *alternative > race.distance)
        // .inspect(|f| println!("race:{:?}, alt: {:?}", race, f))
        .count();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}
