use anyhow::{Ok, Result};
use day_1::part1::process;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");
    let result = process(input)?;
    println!("{}", result);
    Ok(())
}
