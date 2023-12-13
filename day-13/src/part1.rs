use super::shared::*;

// used solution from part2 with 0 smudges instead of this
// fn find_symmetry(block: &[String]) -> usize {
//     for x in 1..block.len() {
//         let mut above = block[..x].to_vec();
//         above.reverse();
//         let below = &block[x..];
//         let shorter = above.len().min(below.len());
//         if above[..shorter] == below[..shorter] {
//             return x;
//         }
//     }
//     0
// }

pub fn process(input: &str) -> String {
    let (_, lines) = parse_input(input).unwrap();
    let mut result: usize = 0;
    for block in lines.iter() {
        // rows
        // dbg!(block);
        result += find_symmetry(block, 0) * 100;

        // columns
        let new_block = rotate(block);
        // dbg!("rotated block", &new_block);
        result += find_symmetry(&new_block, 0);
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
        assert_eq!("405", process(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!("28651", process(input));
    }
}
