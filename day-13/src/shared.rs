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
    for x in 1..block.len() {
        let mut above = block[..x].to_vec();
        above.reverse();
        let below = &block[x..];

        if below.iter().zip(above.clone())
        // at this point we have tuples of (row_x, row_y) for each row above & below
        .map(|(x, y)| {
            // for each line check each chars
            x.chars()
                .zip(y.chars())
                .map(|(c1, c2)| if c1 == c2 { 0 } else { 1 })
                .sum::<usize>() // number of mismatches between row x and row y
        })
        .sum::<usize>() // number of all mismatches 
        == number_of_smudges
        {
            return x;
        }
    }
    0
}
