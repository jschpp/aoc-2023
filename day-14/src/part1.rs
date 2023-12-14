use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult, Parser,
};

fn rotate(block: &[String]) -> Vec<String> {
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

pub fn parse_input(input: &str) -> IResult<&str, Vec<String>> {
    separated_list1(
        line_ending,
        many1(one_of("#.O")).map(|x| x.iter().collect::<String>()),
    )(input)
}

pub fn tilt(grid: &[String]) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(grid.len());
    for line in grid.iter() {
        let mut tmp_line: Vec<char> = vec!['.'; line.len()];
        let mut o_idx: usize = 0;
        line.chars().enumerate().for_each(|(c_idx, c)| match c {
            '.' => {}
            '#' => {
                tmp_line[c_idx] = '#';
                o_idx = c_idx + 1;
            }
            'O' => {
                tmp_line[o_idx] = 'O';
                o_idx += 1;
            }
            val => unreachable!("found {}", val),
        });
        result.push(tmp_line.iter().rev().collect())
    }
    result
}

pub fn process(input: &str) -> String {
    let (_, lines) = parse_input(input).unwrap();
    let cols = rotate(&lines);
    tilt(&cols)
        .iter()
        .fold(0, |acc, line| {
            acc + line
                .chars()
                .enumerate()
                .map(|(idx, c)| if c == 'O' { idx + 1 } else { 0 })
                .sum::<usize>()
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!("136", process(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!("110779", process(input));
    }
}
