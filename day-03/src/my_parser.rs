use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Column {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub column: Column,
    pub line: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct CharPart {
    pub value: char,
    pub location: Location,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValuePart {
    pub value: usize,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct Parts {
    pub value_parts: Vec<ValuePart>,
    pub char_parts: Vec<CharPart>,
}

fn to_u32(input: &[char]) -> u32 {
    input
        .iter()
        .map(|c| c.to_digit(10).expect("is valid digit"))
        .rev() // start with Least significant digit
        .enumerate()
        .fold(0, |acc, (power, val)| {
            acc + (u32::pow(10, power as u32) * val)
        })
}

pub fn line_parser(input: &str, line_number: usize) -> Parts {
    let mut buffer: Vec<char> = Vec::new();
    let mut token_start: Option<usize> = None;
    let mut char_parts: Vec<CharPart> = Vec::new();
    let mut value_parts: Vec<ValuePart> = Vec::new();
    for (position, c) in input.chars().enumerate() {
        if c.is_ascii_digit() {
            if token_start.is_none() {
                token_start = Some(position)
            }
            buffer.push(c)
        } else {
            if token_start.is_some() {
                // we had a token going. Time to finish it
                let loc = Location {
                    column: Column {
                        start: token_start.expect("we have some"),
                        end: position - 1,
                    },
                    line: line_number,
                };
                let val = to_u32(&buffer);
                value_parts.push(ValuePart {
                    value: val as usize,
                    location: loc,
                });
                buffer.clear();
                token_start = None
            }

            // find valid char tokens
            if c != '.' {
                char_parts.push(CharPart {
                    value: c,
                    location: Location {
                        column: Column {
                            start: position,
                            end: position,
                        },
                        line: line_number,
                    },
                })
            }
        }
    }

    // we reached the end of the line and there is still something in the buffer
    if !buffer.is_empty() {
        // we had a token going. Time to finish it
        let loc = Location {
            column: Column {
                start: token_start.expect("should still be set"),
                end: input.len() - 1,
            },
            line: line_number,
        };
        let val = to_u32(&buffer);
        value_parts.push(ValuePart {
            value: val as usize,
            location: loc,
        });
    }

    Parts {
        char_parts,
        value_parts,
    }
}
