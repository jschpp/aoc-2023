use std::collections::HashMap;

use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult, Parser,
};

pub fn parse_input(input: &str) -> IResult<&str, Vec<String>> {
    separated_list1(
        line_ending,
        many1(one_of("#.O")).map(|x| x.iter().collect::<String>()),
    )(input)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    West,
    East,
    South,
}

fn roll(dir: Direction, grid: Vec<String>) -> Vec<String> {
    use Direction::*;
    let mut result: Vec<String> = Vec::new();
    match dir {
        West => {
            for line in grid.iter() {
                let mut tmp_line = vec!['.'; line.len()];
                let mut next_stone_idx: usize = 0;
                for (c_idx, c) in line.chars().enumerate() {
                    match c {
                        '.' => {
                            continue;
                        }
                        '#' | 'N' => {
                            tmp_line[c_idx] = c;
                            next_stone_idx = c_idx + 1;
                        }
                        'O' => {
                            tmp_line[next_stone_idx] = 'O';
                            next_stone_idx += 1;
                        }
                        val => unreachable!("found {}", val),
                    }
                }
                result.push(tmp_line.into_iter().collect());
            }
        }
        North => {
            let mut tmp_grid = vec![vec!['.'; grid[0].len()]; grid.len()];
            let mut next_stone_per_col: Vec<usize> = vec![0; grid[0].len()];
            for (line_idx, line) in grid.iter().enumerate() {
                for (c_idx, c) in line.chars().enumerate() {
                    match c {
                        '.' => {
                            continue;
                        }
                        '#' | 'N' => {
                            tmp_grid[line_idx][c_idx] = c;
                            next_stone_per_col[c_idx] = line_idx + 1;
                        }
                        'O' => {
                            tmp_grid[next_stone_per_col[c_idx]][c_idx] = 'O';
                            next_stone_per_col[c_idx] += 1;
                        }
                        val => unreachable!("found {}", val),
                    }
                }
                result = tmp_grid
                    .iter()
                    .map(|line| line.iter().collect::<String>())
                    .collect();
            }
        }
        East => {
            for line in grid.iter() {
                let mut tmp_line = vec!['.'; line.len()];
                let mut next_stone_idx: usize = 0;
                for (c_idx, c) in line.chars().rev().enumerate() {
                    match c {
                        '.' => {
                            continue;
                        }
                        '#' | 'N' => {
                            tmp_line[c_idx] = c;
                            next_stone_idx = c_idx + 1;
                        }
                        'O' => {
                            tmp_line[next_stone_idx] = 'O';
                            next_stone_idx += 1;
                        }
                        val => unreachable!("found {}", val),
                    }
                }
                result.push(tmp_line.into_iter().rev().collect());
            }
        }
        South => {
            let mut tmp_grid: Vec<Vec<char>> = vec![vec!['.'; grid[0].len()]; grid.len()];
            let mut next_stone_per_col: Vec<usize> = vec![0; grid[0].len()];
            for (line_idx, line) in grid.iter().rev().enumerate() {
                for (c_idx, c) in line.chars().enumerate() {
                    match c {
                        '.' => {
                            continue;
                        }
                        '#' | 'N' => {
                            tmp_grid[line_idx][c_idx] = c;
                            next_stone_per_col[c_idx] = line_idx + 1;
                        }
                        'O' => {
                            tmp_grid[next_stone_per_col[c_idx]][c_idx] = 'O';
                            next_stone_per_col[c_idx] += 1;
                        }
                        val => unreachable!("found {}", val),
                    }
                }
                result = tmp_grid
                    .iter()
                    .rev()
                    .map(|line| line.iter().collect::<String>())
                    .collect();
            }
        }
    }
    result
}

fn count(grid: &[String]) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (line_idx, line)| {
            acc + line
                .chars()
                .map(|c| if c == 'O' { line_idx + 1 } else { 0 })
                .sum::<usize>()
        })
}

fn rotate(grid: Vec<String>) -> Vec<String> {
    let grid = roll(Direction::North, grid);
    let grid = roll(Direction::West, grid);
    let grid = roll(Direction::South, grid);
    roll(Direction::East, grid)
}

pub fn process(input: &str) -> String {
    let (_, lines) = parse_input(input).unwrap();

    let mut map: HashMap<Vec<String>, usize> = HashMap::new();
    let mut grid = lines.clone();
    let mut x: usize = 0;
    while x < 1_000_000_000_usize {
        if map.contains_key(&grid) {
            let start_period = map.get(&grid).expect("key exists");
            let period_len = map.len() - start_period;
            x = (1_000_000_000 - start_period) % period_len + start_period;
            break;
        } else {
            map.insert(grid.clone(), x);
            let new_grid = rotate(grid.clone());
            x += 1;
            grid = new_grid;
        }
    }
    let grid = map.into_iter().find(|(_, v)| v == &x).expect("key exists");
    count(&grid.0).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_west() {
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
        let (_, grid) = parse_input(input).unwrap();
        assert_eq!(
            vec![
                "O....#....",
                "OOO.#....#",
                ".....##...",
                "OO.#OO....",
                "OO......#.",
                "O.#O...#.#",
                "O....#OO..",
                "O.........",
                "#....###..",
                "#OO..#....",
            ],
            roll(Direction::West, grid)
        );
    }

    #[test]
    fn test_roll_east() {
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
        let (_, grid) = parse_input(input).unwrap();
        assert_eq!(
            vec![
                "....O#....",
                ".OOO#....#",
                ".....##...",
                ".OO#....OO",
                "......OO#.",
                ".O#...O#.#",
                "....O#..OO",
                ".........O",
                "#....###..",
                "#..OO#....",
            ],
            roll(Direction::East, grid)
        );
    }

    #[test]
    fn test_roll_north() {
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
        let (_, grid) = parse_input(input).unwrap();
        assert_eq!(
            vec![
                "OOOO.#.O..",
                "OO..#....#",
                "OO..O##..O",
                "O..#.OO...",
                "........#.",
                "..#....#.#",
                "..O..#.O.O",
                "..O.......",
                "#....###..",
                "#....#....",
            ],
            roll(Direction::North, grid)
        );
    }

    #[test]
    fn test_roll_south() {
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
        let (_, grid) = parse_input(input).unwrap();
        assert_eq!(
            vec![
                ".....#....",
                "....#....#",
                "...O.##...",
                "...#......",
                "O.O....O#O",
                "O.#..O.#.#",
                "O....#....",
                "OO....OO..",
                "#OO..###..",
                "#OO.O#...O",
            ],
            roll(Direction::South, grid)
        );
    }

    #[test]
    fn test_cycles() {
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

        let results = vec![
            vec![
                ".....#....",
                "....#...O#",
                "...OO##...",
                ".OO#......",
                ".....OOO#.",
                ".O#...O#.#",
                "....O#....",
                "......OOOO",
                "#...O###..",
                "#..OO#....",
            ],
            vec![
                ".....#....",
                "....#...O#",
                ".....##...",
                "..O#......",
                ".....OOO#.",
                ".O#...O#.#",
                "....O#...O",
                ".......OOO",
                "#..OO###..",
                "#.OOO#...O",
            ],
            vec![
                ".....#....",
                "....#...O#",
                ".....##...",
                "..O#......",
                ".....OOO#.",
                ".O#...O#.#",
                "....O#...O",
                ".......OOO",
                "#...O###.O",
                "#.OOO#...O",
            ],
        ];

        let (_, grid) = parse_input(input).unwrap();

        assert_eq!(rotate(grid.clone()), results[0]);
        assert_eq!(rotate(rotate(grid.clone())), results[1]);
        assert_eq!(rotate(rotate(rotate(grid.clone()))), results[2]);
    }

    #[test]
    fn test_count() {
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
        let (_, grid) = parse_input(input).unwrap();
        let grid = roll(Direction::North, grid);
        assert_eq!(136, count(&grid));
    }

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
        assert_eq!("64", process(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!("86069", process(input));
    }
}
