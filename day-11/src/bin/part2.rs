use day_11::part2::process;

fn main() {
    let file = include_str!("../../input.txt");
    let result = process(file, 1_000_000);
    println!("{}", result);
}
