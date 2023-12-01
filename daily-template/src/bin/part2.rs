use {{crate_name}}::part2::process;
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}