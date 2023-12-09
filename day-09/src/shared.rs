pub fn create_sequences(numbers: &[i32]) -> Vec<Vec<i32>> {
    let mut sequences: Vec<Vec<i32>> = Default::default();
    sequences.push(numbers.to_vec());
    loop {
        let current = sequences.last().expect("at least one vec").clone();
        let tmp: Vec<i32> = current.windows(2).map(|c| c[1] - c[0]).collect();
        if tmp.iter().all(|x| x == &0) {
            break;
        }
        sequences.push(tmp);
    }
    sequences
}

pub fn my_parser(line: &str) -> Vec<i32> {
    line.split(' ')
        .map(|number| {
            let n = number.parse::<i32>();
            match n {
                Err(e) => panic!("err {} on {:?}", e, number),
                Ok(n) => n,
            }
        })
        .collect::<Vec<i32>>()
}