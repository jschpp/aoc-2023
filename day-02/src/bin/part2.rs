use anyhow::Context;
use day_02::part2::process;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}
