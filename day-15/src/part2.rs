use std::{collections::HashMap, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    combinator::value,
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};

use super::shared::my_hash;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    Set,
    Remove,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x: char = match self {
            Operation::Remove => '-',
            Operation::Set => '=',
        };
        write!(f, "{}", x)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub label: String,
    pub op: Operation,
    pub focal_len: Option<usize>,
    hash: usize,
}

impl Instruction {
    pub fn hash(&self) -> usize {
        self.hash
    }
}

fn parse_set(input: &str) -> IResult<&str, Instruction> {
    tuple((alpha1, value(Operation::Set, tag("=")), complete::u32))
        .parse(input)
        .map(|(rest, (name, op, val))| {
            (
                rest,
                Instruction {
                    focal_len: Some(val as usize),
                    hash: my_hash(name) as usize,
                    label: name.to_string(),
                    op,
                },
            )
        })
}

fn parse_remove(input: &str) -> IResult<&str, Instruction> {
    tuple((alpha1, value(Operation::Remove, tag("-"))))
        .parse(input)
        .map(|(rest, (name, op))| {
            (
                rest,
                Instruction {
                    label: name.to_string(),
                    op,
                    focal_len: None,
                    hash: my_hash(name) as usize,
                },
            )
        })
}

fn my_parser(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(tag(","), alt((parse_remove, parse_set)))(input)
}

pub fn process(input: &str) -> String {
    let (_, parsed) = my_parser(input).unwrap();

    let mut boxes: Vec<Vec<String>> = vec![Vec::new(); 256];
    let mut focal_lengths: HashMap<String, usize> = HashMap::new();

    parsed.iter().for_each(|instruction| {
        let idx = boxes[instruction.hash()]
            .iter()
            .position(|l| l == &instruction.label);
        match instruction.op {
            Operation::Set => {
                if idx.is_none() {
                    boxes[instruction.hash()].push(instruction.label.clone());
                }
                let val = instruction.focal_len.expect("value should exist");
                focal_lengths
                    .entry(instruction.label.clone())
                    .and_modify(|x| *x = val)
                    .or_insert(val);
            }
            Operation::Remove => {
                if let Some(idx) = idx {
                    boxes[instruction.hash()].remove(idx);
                }
            }
        }
    });

    let result = boxes
        .into_iter()
        .zip(1..)
        .fold(0, |acc, (lens_box, box_number)| {
            acc + lens_box
                .into_iter()
                .zip(1..)
                .fold(0, |acc, (lens, lens_number)| {
                    acc + box_number * lens_number * focal_lengths[&lens]
                })
        });
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove() {
        let input = "cm-";
        let (rest, parsed) = parse_remove(input).unwrap();
        assert_eq!(rest.len(), 0);
        assert_eq!(
            Instruction {
                focal_len: None,
                hash: my_hash("cm") as usize,
                label: "cm".to_string(),
                op: Operation::Remove
            },
            parsed
        )
    }

    #[test]
    fn test_set() {
        let input = "rn=1";
        let (rest, parsed) = parse_set(input).unwrap();
        assert_eq!(rest.len(), 0);
        assert_eq!(
            Instruction {
                focal_len: Some(1),
                hash: my_hash("rn") as usize,
                label: "rn".to_string(),
                op: Operation::Set
            },
            parsed
        )
    }

    #[test]
    fn test_parser() {
        let input = "rn=1,cm-";
        let result: Vec<Instruction> = vec![
            Instruction {
                focal_len: Some(1),
                hash: my_hash("rn") as usize,
                label: "rn".to_owned(),
                op: Operation::Set,
            },
            Instruction {
                focal_len: None,
                hash: my_hash("cm") as usize,
                label: "cm".to_owned(),
                op: Operation::Remove,
            },
        ];
        let parsed = match my_parser(input) {
            Ok((_rest, parsed)) => parsed,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(result, parsed);
    }

    #[test]
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("145", process(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!("303404", process(input));
    }
}
