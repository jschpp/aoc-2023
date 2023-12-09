use {{crate_name}}::part2::process;

fn main() {
    let file = include_str!("../../input.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
}