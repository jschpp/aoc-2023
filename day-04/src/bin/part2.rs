use day_04::part2::process;
use anyhow::Context;
use colored::control;

fn main() -> anyhow::Result<()> {
    control::set_virtual_terminal(false).unwrap();
    let file = include_str!("../../input.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}