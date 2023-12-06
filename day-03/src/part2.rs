use super::my_parser::{line_parser, CharPart, Parts, ValuePart};

pub fn process(input: &str) -> anyhow::Result<String> {
    let parts: Vec<Parts> = input
        .lines()
        .enumerate()
        .map(|(line_number, line)| line_parser(line, line_number))
        .collect();
    // select 'gears'
    let char_parts: Vec<CharPart> =
        parts
            .iter()
            .map(|x| x.char_parts.clone())
            .fold(Vec::new(), |mut v: Vec<CharPart>, x| {
                v.extend(x.iter().filter(|part| part.value == '*'));
                v
            });
    // select all value parts touching a gear
    let value_parts: Vec<ValuePart> = parts
        .iter()
        .map(|x| x.value_parts.clone())
        .fold(Vec::new(), |mut v: Vec<ValuePart>, x| {
            v.extend(x.iter());
            v
        })
        .into_iter()
        .filter(|part| part.touches(&char_parts))
        .collect();
    // for all 'gears' check if they touch exactly 2 value parts and calculate the gear ratio
    let result: usize = char_parts
        .iter()
        .filter_map(|gear| {
            let touching: Vec<ValuePart> = value_parts
                .iter()
                .copied()
                .filter(|part| part.touches(&[*gear]))
                .collect();
            if touching.len() == 2 {
                Some(touching[0].value * touching[1].value)
            } else {
                None
            }
        })
        .sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
