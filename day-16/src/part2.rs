use super::shared::*;
use glam::IVec2;
use std::collections::VecDeque;
use toodee::{TooDee, TooDeeOps};

fn illuminate_grid(grid: &mut TooDee<Tile>, start: (IVec2, Direction)) -> usize {
    let mut work_queue: VecDeque<(IVec2, Direction)> = VecDeque::new();
    work_queue.push_back(start);
    while !work_queue.is_empty() {
        let (pos, to) = work_queue.pop_front().expect("not empty yet");
        if let Some(new_positions) = grid[pos.x as usize][pos.y as usize].pass(to) {
            for new_pos in new_positions.into_iter() {
                if new_pos.x >= 0 && new_pos.y >= 0 {
                    let x = new_pos.x as usize;
                    let y = new_pos.y as usize;
                    let new_direction: Direction = Direction::from_points(pos, new_pos);
                    if x < grid.num_cols() && y < grid.num_rows() {
                        work_queue.push_back((new_pos, new_direction))
                    }
                }
            }
        }
    }
    grid.into_iter().filter(|tile| tile.illuminated).count()
}

fn get_starting_positions(num_rows: i32, num_cols: i32) -> Vec<(IVec2, Direction)> {
    assert!(num_cols >= 0 && num_rows >= 0);
    // convert to idx
    let max_row_idx = num_rows - 1;
    let max_col_idx = num_cols - 1;

    // corners
    let mut result: Vec<(IVec2, Direction)> = vec![
        (IVec2::new(0, 0), Direction::East),
        (IVec2::new(0, 0), Direction::South),
        (IVec2::new(max_col_idx, 0), Direction::West),
        (IVec2::new(max_col_idx, 0), Direction::South),
        (IVec2::new(0, max_row_idx), Direction::East),
        (IVec2::new(0, max_row_idx), Direction::North),
        (IVec2::new(max_col_idx, 0), Direction::West),
        (IVec2::new(max_col_idx, 0), Direction::North),
    ];

    // left & right edges
    for y_pos in 1..max_row_idx {
        result.push((IVec2::new(0, y_pos), Direction::East));
        result.push((IVec2::new(max_col_idx, y_pos), Direction::West));
    }

    // top and bottom row
    for x_pos in 1..max_col_idx {
        result.push((IVec2::new(x_pos, 0), Direction::South));
        result.push((IVec2::new(x_pos, max_row_idx), Direction::North));
    }

    result
}

pub fn process(input: &str) -> String {
    let grid = parse_into_grid(input);
    get_starting_positions(grid.num_rows() as i32, grid.num_cols() as i32)
        .iter()
        .map(|s| illuminate_grid(&mut grid.clone(), *s))
        .max()
        .expect("some result should exists")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!("51", process(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!("7846", process(input));
    }
}
