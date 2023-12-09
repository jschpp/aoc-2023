use crate::shared::*;

fn get_next_number(previous_number: Option<i32>, current_number: i32) -> i32 {
    if let Some(previous_number) = previous_number {
        current_number + previous_number
    } else {
        current_number
    }
}

fn extrapolate(sequences: Vec<Vec<i32>>) -> i32 {
    let mut previous_number: Option<i32> = None;
    sequences.iter().rev().for_each(|s| {
        previous_number = Some(get_next_number(
            previous_number,
            *s.last().expect("not empty"),
        ));
    });
    previous_number.expect("has value")
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let numbers: Vec<Vec<i32>> = input.lines().map(my_parser).collect();
    let result: i32 = numbers
        .iter()
        .map(|sequence| create_sequences(sequence))
        .map(extrapolate)
        .sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        assert_eq!("114", process(input)?);
        Ok(())
    }
}
