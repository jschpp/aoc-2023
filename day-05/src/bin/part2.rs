#![feature(iter_array_chunks)]
use day_05::part2::process;
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}