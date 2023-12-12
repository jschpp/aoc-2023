use std::fmt::Debug;

use itertools::{repeat_n, Itertools};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecordType {
    Broken,
    Unknown,
    Empty,
}

impl TryFrom<char> for RecordType {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(RecordType::Broken),
            '?' => Ok(RecordType::Unknown),
            '.' => Ok(RecordType::Empty),
            _ => Err("unknown type"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Record {
    pub typ: RecordType,
    pub position: Position,
}

#[derive(Debug)]
pub struct Configuration {
    pub groups: Vec<usize>,
}

#[derive(Debug)]
pub struct Line {
    pub records: Vec<Record>,
    pub configuration: Vec<usize>,
}

pub fn parse_record(input: &str, line_idx: usize) -> Vec<Record> {
    input
        .chars()
        .enumerate()
        .map(|(char_idx, c)| Record {
            typ: c.try_into().expect("should work"),
            position: Position {
                line: line_idx,
                column: char_idx,
            },
        })
        .collect()
}

pub fn line_parser(input: &str, line_idx: usize) -> Line {
    let parts = input.split(' ').collect::<Vec<&str>>();
    Line {
        records: parse_record(parts[0], line_idx),
        configuration: parts[1]
            .split(',')
            .flat_map(|x| x.parse::<usize>().ok())
            .collect(),
    }
}

fn get_options_for_unknown(len: usize, offset: usize) -> Vec<Vec<(usize, RecordType)>> {
    repeat_n(vec![RecordType::Broken, RecordType::Empty].into_iter(), len)
        .multi_cartesian_product()
        .unique()
        .map(|v| {
            v.into_iter()
                .enumerate()
                .map(|(idx, elem)| (idx + offset, elem))
                .collect::<Vec<(usize, RecordType)>>()
        })
        .filter(|v| v.len() == len)
        // .inspect(|x| println!("{:?}", x))
        .collect()
}

// fn pretty_print(records: &[Record]) {
//     for record in records {
//         match record.typ {
//             RecordType::Broken => print!("#"),
//             RecordType::Empty => print!("."),
//             RecordType::Unknown => print!("?"),
//         }
//     }
//     println!()
// }

fn new_line(
    old_line: &[Record],
    options_for_unknown: Vec<Vec<(usize, RecordType)>>,
    configuration: &[usize],
) -> usize {
    assert!(!old_line.is_empty());
    // let mut options: Vec<Vec<Record>> = Vec::with_capacity(options_for_unknown.len() * 2);
    let mut count: usize = 0;
    let line_number = old_line[0].position.line;
    for v in options_for_unknown {
        let mut tmp_rec = old_line.to_vec();
        for (idx, option) in v {
            tmp_rec[idx] = Record {
                position: Position {
                    line: line_number,
                    column: idx,
                },
                typ: option,
            };
        }
        if filter_option(configuration, &tmp_rec) {
            count += 1
        }
    }
    count
}

pub fn filter_option(configuration: &[usize], options: &[Record]) -> bool {
    let mut ranges: Vec<usize> = Vec::new();
    let mut range_count: usize = 0;
    for record in options {
        match record.typ {
            RecordType::Empty => {
                if range_count != 0 {
                    ranges.push(range_count);
                    range_count = 0;
                }
            }
            RecordType::Broken => range_count += 1,
            RecordType::Unknown => panic!("shouldn't exist here any longer"),
        }
    }
    if range_count != 0 {
        ranges.push(range_count);
    }
    // println!("conf: {:?} seen: {:?}", self.configuration, ranges);
    configuration == ranges
}

/// append each new set of unknowns to each existing set
/// this will do this:
///
/// ```rust
/// use crate::day_12::part1::RecordType;
///
/// let original_unknowns = [[(0,RecordType::Unknown)], [(0,RecordType::Broken)]];
/// let new_unknowns = [[(1,RecordType::Unknown)],[(0,RecordType::Broken)]];
/// let result_of_merge_options =[
///     [(0,RecordType::Unknown), (1,RecordType::Unknown)],
///     [(0,RecordType::Unknown),(1,RecordType::Broken)],
///     [(0,RecordType::Broken),(1,RecordType::Unknown)],
///     [(0,RecordType::Broken),(1,RecordType::Broken)]
/// ];
/// ```
fn merge_options(
    original: &[Vec<(usize, RecordType)>],
    options: &[Vec<(usize, RecordType)>],
) -> Vec<Vec<(usize, RecordType)>> {
    if original.is_empty() {
        // first time finding unknowns
        options.to_vec()
    } else {
        let new_options: Vec<Vec<(usize, RecordType)>> = options
            .iter()
            // .inspect(|x| println!("{:?}", x))
            .flat_map(|unknown_option| {
                original
                    .iter()
                    // .inspect(|x| println!("{:?}", x))
                    .map(|original_options| {
                        let mut unknown_option = unknown_option.clone();
                        let mut original_options = original_options.clone();
                        original_options.append(&mut unknown_option);
                        original_options
                    })
                    .collect::<Vec<Vec<(usize, RecordType)>>>()
            })
            // .inspect(|x| println!("{:?}", x))
            .collect();
        new_options
    }
}

impl Line {
    pub fn find_options(&self) -> usize {
        let mut unknown_range: Vec<usize> = Vec::new();
        let mut options: Vec<Vec<(usize, RecordType)>> = Vec::new();
        for record in self.records.iter() {
            match record.typ {
                RecordType::Unknown => unknown_range.push(record.position.column),
                _ => {
                    if unknown_range.is_empty() {
                        // no new unknowns found
                        continue;
                    }

                    let options_for_unknown = get_options_for_unknown(
                        unknown_range.len(),
                        record.position.column - unknown_range.len(),
                    );

                    if !unknown_range.is_empty() {
                        unknown_range.clear();
                    }

                    options = merge_options(&options, &options_for_unknown);
                }
            };
        }
        if !unknown_range.is_empty() {
            println!("here");
            let options_for_unknown = get_options_for_unknown(
                unknown_range.len(),
                self.records.len() - unknown_range.len(),
            );
            options = merge_options(&options, &options_for_unknown);
        }

        let new_line_options: usize = new_line(&self.records, options, &self.configuration);
        // new_line_options.iter().for_each(|o| pretty_print(o));
        println!("{:?}", new_line_options);
        new_line_options
    }

    pub fn calculate_options(&self) -> usize {
        self.find_options()
        // .iter()
        // .inspect(|x| pretty_print(x))
        // .filter(|option| self.filter_option(option))
        // .inspect(|x| pretty_print(x))
        // .count()
    }
}

pub fn process(input: &str) -> String {
    input
        .lines()
        .enumerate()
        .map(|(line_idx, line)| line_parser(line, line_idx))
        .map(|line| line.calculate_options())
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options() {
        let input = "???.#?# 1,1,3";
        assert_eq!("1", process(input));
    }

    #[test]
    fn test_options_1() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!("4", process(input));
    }

    // #[test]
    // fn test_options_2() {
    //     let input = "?????????? 1,1";
    //     assert_eq!("45", process(input));
    // }

    #[test]
    fn test_process() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("21", process(input));
    }

    #[test]
    // #[ignore = "slow"]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!("7771", process(input));
    }
}
