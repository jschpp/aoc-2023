use pathfinding::prelude::dijkstra_reach;

use super::my_parser::{get_symbol_for_s, my_parser};
use crate::types::flood::*;
use crate::types::pipe::Pipe;
use crate::types::shared::GridLike;

pub fn process(input: &str) -> String {
    // parse grid
    let mut grid = my_parser(input);

    // set start to correct pipe symbol
    let start = grid.start.expect("start exists");
    grid[start].as_mut().expect("exists").symbol =
        get_symbol_for_s(grid[start].as_ref().expect("exists"));

    // get start for search
    let start_pipe = grid[start].as_ref().expect("start exists");

    // create "flood grid" for ray tracing
    let mut flood_grid: FloodGrid = FloodGrid::new(grid.line_max, grid.column_max);

    // create loop and fill in flood gird
    dijkstra_reach(start_pipe, |pipe: &Pipe, _y| pipe.successors(&grid))
        .map(|r| r.node)
        .for_each(|pipe| {
            flood_grid.grid[pipe.position.line][pipe.position.column] =
                Some(Flood::Pipe { x: *pipe.symbol })
        });
    // ray tracing
    // start at the edge (hopefully save outside)
    let mut next = Flood::Outside;
    for line in flood_grid.grid.iter_mut() {
        for node in line.iter_mut() {
            if let Some(node) = node {
                // when encountering an uncrossable pipe going East to West flip from inside to outside
                if let Flood::Pipe { x } = node {
                    match x {
                        '|' => next = next.flip(),
                        'F' => next = next.flip(),
                        '7' => next = next.flip(),
                        _ => {}
                    }
                }
            } else {
                *node = Some(next);
            }
        }
        next = Flood::Outside;
    }

    // print result for sanity checking
    flood_grid.pretty_print();

    // count number of Inside Nodes
    flood_grid
        .grid
        .iter()
        .fold(0, |acc, line| {
            acc + line.iter().fold(0, |acc, node| {
                acc + node
                    .as_ref()
                    .map(|x| if x == &Flood::Inside { 1 } else { 0 })
                    .expect("Value")
            })
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!("4", process(input));
    }

    #[test]
    fn test_process_1_5() {
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!("4", process(input));
    }

    #[test]
    fn test_process_2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!("8", process(input));
    }
}
