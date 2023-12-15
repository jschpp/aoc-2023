use anyhow::Context;
use day_05::part2::process;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}
