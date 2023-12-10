use pathfinding::prelude::dijkstra_reach;

use crate::types::shared::GridLike;

use super::my_parser::my_parser;

pub fn process(input: &str) -> String {
    let grid = my_parser(input);
    grid.pretty_print();
    let start = grid.start.expect("start exists");
    let start_pipe = grid[start].as_ref().expect("start exists");
    let result: usize = dijkstra_reach(start_pipe, |x, _y| x.successors(&grid))
        .max_by_key(|r| r.total_cost)
        .expect("found something")
        .total_cost;
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!("4", process(input));
    }

    #[test]
    fn test_process_1_5() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!("4", process(input));
    }

    #[test]
    fn test_process_2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!("8", process(input));
    }
}
