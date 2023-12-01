use anyhow::{Ok, Result};
use day_1::process_part1;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");
    let result = process_part1(input)?;
    println!("{}", result);
    Ok(())
}
