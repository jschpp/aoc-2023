use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    line_idx: usize,
    column_idx: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Galaxy {
    pub position: Position,
}

impl Galaxy {
    pub fn distance_to(&self, other: &Self) -> usize {
        (self.position.column_idx as i32 - other.position.column_idx as i32).unsigned_abs() as usize
            + (self.position.line_idx as i32 - other.position.line_idx as i32).unsigned_abs()
                as usize
    }

    pub fn expand(&mut self, empty_lines: &HashSet<usize>, empty_columns: &HashSet<usize>) {
        self.position.line_idx += empty_lines
            .iter()
            .filter(|x| x < &&self.position.line_idx)
            .count();
        self.position.column_idx += empty_columns
            .iter()
            .filter(|x| x < &&self.position.column_idx)
            .count();
    }
}

pub fn find_empty_lines(image: &[Galaxy], max: usize) -> HashSet<usize> {
    let mut map: HashSet<usize> = HashSet::new();
    (0..max).for_each(|x| {
        map.insert(x);
    });
    image.iter().for_each(|galaxy| {
        map.remove(&galaxy.position.line_idx);
    });
    map
}

pub fn find_empty_columns(image: &[Galaxy], max: usize) -> HashSet<usize> {
    let mut map: HashSet<usize> = HashSet::new();
    for x in 0..max {
        let _ = map.insert(x);
    }
    image.iter().for_each(|galaxy| {
        map.remove(&galaxy.position.column_idx);
    });
    map
}

pub fn my_parser(input: &str) -> (Vec<Galaxy>, usize, usize) {
    let mut line_max: usize = 0;
    let mut column_max: usize = 0;
    let image = input
        .lines()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line_max = line_max.max(line_idx);
            column_max = column_max.max(line.len() - 1);
            line.chars()
                .enumerate()
                .flat_map(move |(column_idx, c)| match c {
                    '#' => Some(Galaxy {
                        position: Position {
                            line_idx,
                            column_idx,
                        },
                    }),
                    '.' => None,
                    val => panic!("error parsing {}", val),
                })
        })
        .collect();
    (image, line_max, column_max)
}

pub fn process(input: &str) -> String {
    let (image, line_max, column_max) = my_parser(input);
    let empty_columns: HashSet<usize> = find_empty_columns(&image, column_max);
    let empty_lines: HashSet<usize> = find_empty_lines(&image, line_max);

    let image: usize = image
        .into_iter()
        .map(|mut g| {
            g.expand(&empty_lines, &empty_columns);
            g
        })
        .combinations(2)
        .map(|x: Vec<Galaxy>| x[0].distance_to(&x[1]))
        .sum();
    image.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to() {
        let a = Galaxy {
            position: Position {
                line_idx: 6,
                column_idx: 1,
            },
        };
        let b = Galaxy {
            position: Position {
                line_idx: 11,
                column_idx: 5,
            },
        };
        assert_eq!(a.distance_to(&b), 9);
        assert_eq!(b.distance_to(&a), 9);
    }

    #[test]
    fn test_process() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("374", process(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!("9918828", process(input));
    }
}
