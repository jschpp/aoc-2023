fn get_next_number(previous_number: Option<i32>, current_number: &i32) -> i32 {
    if previous_number.is_none() {
        return *current_number;
    }
    current_number - previous_number.expect("has value")
}

fn extrapolate(sequences: Vec<Vec<i32>>) -> i32 {
    let mut previous_number: Option<i32> = None;
    sequences.iter().rev().for_each(|s| {
        let new_number = get_next_number(previous_number, s.first().expect("not empty"));
        previous_number = Some(new_number);
    });
    previous_number.expect("has value")
}

fn create_sequences(numbers: &[i32]) -> Vec<Vec<i32>> {
    let mut sequences: Vec<Vec<i32>> = Default::default();
    sequences.push(numbers.to_vec());
    loop {
        let current = sequences.last().expect("at least one vec").clone();
        let tmp: Vec<i32> = current.windows(2).map(|c| c[1] - c[0]).collect();
        if tmp.iter().all(|x| x == &0) {
            break;
        }
        sequences.push(tmp);
    };
    sequences
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let numbers: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|number| {
                    let n = number.parse::<i32>();
                    match n {
                        Err(e) => panic!("err {} on {:?}", e, number),
                        Ok(n) => n,
                    }
                })
                .collect::<Vec<i32>>()
        })
        .collect();
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
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
