use itertools::{repeat_n, Itertools};
use memoize::memoize;

/// since part1 was waaaaaaaaaaay to slow to work
///
/// this on is recursive. It's still not fast but
/// at least this doesn't crash my pc oO
/// welp memoize fixes the slow oO'
#[memoize]
pub fn count(input: String, nums: Vec<usize>) -> usize {
    if input.is_empty() {
        if nums.is_empty() {
            return 1;
        }
        // no more gears but still some needed
        return 0;
    }

    if nums.is_empty() {
        if input.contains('#') {
            // no more numbers but still broken gears
            return 0;
        }
        return 1;
    }

    let mut result: usize = 0;

    let first_char = input.chars().next().expect("input not empty");

    if ".?".contains(first_char) {
        // treat the ? as a . and remove it
        result += count(input.chars().skip(1).collect::<String>(), nums.clone())
    }

    if "#?".contains(first_char)
        && nums[0] <= input.len() // input is longer than next number
        && !input[..nums[0]].contains('.') // next num chars contain a . so that will be an invalid range
        && (
            nums[0] == input.len() // we have exactly num[0] tokens left
            || input.chars().nth(nums[0]).expect("exists") != '#' // the next token after this block must not be a #.
                                                                        // There should be a new block after this one
        )
    {
        result += count(
                // skipping nums[0] + 1 tokens will either
                // skip the beginning of the next block as to not open a new one when parsing a dot
                // empty the existing slice in the case of nums[0] == input.len()
            input.chars().skip(nums[0] + 1).collect::<String>(),
            nums[1..].to_vec(),
        );
    }
    result
}

pub fn line_parser(input: String) -> (String, Vec<usize>) {
    input
        .split(' ')
        .tuple_windows::<(_, _)>()
        .map(|(puzzle, nums)| {
            (
                puzzle.to_owned(),
                nums.split(',')
                    .map(|n| n.parse::<usize>().expect("parseable"))
                    .collect(),
            )
        })
        .take(1)
        .exactly_one()
        .unwrap()
}

pub fn unfold(input: &str) -> String {
    #![allow(unstable_name_collisions)]
    // ignore intersperse warnings
    let parts = input.split(' ').collect::<Vec<&str>>();
    [
        repeat_n(parts[0], 5).intersperse("?").collect::<String>(),
        repeat_n(parts[1], 5).intersperse(",").collect::<String>(),
    ]
    .join(" ")
}

pub fn process(input: &str) -> String {
    let lines: Vec<(String, Vec<usize>)> = input
        .lines()
        .map(unfold)
        .map(line_parser)
        // .inspect(|x| println!("{:?}", x))
        .collect();
    let mut result = 0;
    for line in lines.iter() {
        result += count(line.0.clone(), line.1.clone());
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[test]
    fn test_unfold() {
        let input = "???.### 1,1,3";
        assert_eq!(
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3",
            unfold(input)
        );

        let input = ".# 1";
        assert_eq!(".#?.#?.#?.#?.# 1,1,1,1,1", unfold(input));
    }


    
    #[rstest]
    #[case("???.### 1,1,3", "1")]
    #[case(".??..??...?##. 1,1,3", "16384")]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", "1")]
    #[case("????.#...#... 4,1,1", "16")]
    #[case("????.######..#####. 1,6,5", "2500")]
    #[case("?###???????? 3,2,1", "506250")]
    fn test_single_line(#[case] input: String, #[case] expected: String) {
        assert_eq!(process(&input), expected);
    }

    #[test]
    fn test_process() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("525152", process(input));
    }

    #[test]
    #[ignore = "slowish"]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!("10861030975833", process(input));
    }
}
