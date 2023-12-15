use colored::control;
use day_10::part2::process;

fn main() {
    control::set_virtual_terminal(true).unwrap();
    let file = include_str!("../../input.txt");
    let result = process(file);
    println!("{}", result);
}
