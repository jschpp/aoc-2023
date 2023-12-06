use day_05::part2::process;
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}