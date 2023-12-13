use super::shared::*;

pub fn process(input: &str) -> String {
    let (_, lines) = parse_input(input).unwrap();
    let mut result: usize = 0;
    for block in lines.iter() {
        // rows
        // dbg!(block);
        result += find_symmetry(block, 1) * 100;

        // columns
        let new_block = rotate(block);
        // dbg!("rotated block", &new_block);
        result += find_symmetry(&new_block, 1);
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!("400", process(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!("25450", process(input));
    }
}
