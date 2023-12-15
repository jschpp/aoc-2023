use anyhow::Context;
use day_08::part1::process;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}
