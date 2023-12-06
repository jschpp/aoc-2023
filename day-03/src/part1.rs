use super::my_parser::{line_parser, CharPart, Parts, ValuePart};

impl ValuePart {
    pub fn touches(&self, other: &[CharPart]) -> bool {
        let column_range =
            self.location
                .column
                .start
                .checked_sub(1)
                .unwrap_or(self.location.column.start)..=self.location.column.end + 1;
        let line_range = self
            .location
            .line
            .checked_sub(1)
            .unwrap_or(self.location.line)..=self.location.line + 1;
        other.iter().any(|char_part| {
            line_range.contains(&char_part.location.line)
                && (column_range.contains(&char_part.location.column.start))
        })
    }
    fn in_range_of(&self, other: &[Self]) -> bool {
        let column_range =
            self.location
                .column
                .start
                .checked_sub(1)
                .unwrap_or(self.location.column.start)..=self.location.column.end + 1;
        let line_range = self
            .location
            .line
            .checked_sub(1)
            .unwrap_or(self.location.line)..=self.location.line + 1;
        other
            .iter()
            .filter(|part| line_range.contains(&part.location.line))
            .filter(|part| part != &self)
            .any(|part| {
                column_range.contains(&part.location.column.start)
                    || column_range.contains(&part.location.column.end)
            })
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let parts: Vec<Parts> = input
        .lines()
        .enumerate()
        .map(|(line_number, line)| line_parser(line, line_number))
        .collect();
    let value_parts: Vec<ValuePart> = parts
        .iter()
        .map(|x| x.value_parts.clone())
        .fold(Vec::new(), |mut v: Vec<ValuePart>, x| {
            v.extend(x.iter());
            v
        })
        .into_iter()
        .collect();
    let char_parts =
        parts
            .iter()
            .map(|x| x.char_parts.clone())
            .fold(Vec::new(), |mut v: Vec<CharPart>, x| {
                v.extend(x.iter());
                v
            });
    let t = value_parts
        .iter()
        .filter(|&part| part.touches(&char_parts) || part.in_range_of(&value_parts))
        .fold(0, |acc, part| acc + part.value);

    Ok(t.to_string())
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
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
