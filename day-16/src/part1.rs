use super::shared::*;
use glam::IVec2;
use std::collections::{HashMap, VecDeque};
use toodee::TooDee;

pub fn process(input: &str) -> String {
    let mut rows: usize = 0;
    let mut cols: usize = 0;
    let tiles: HashMap<IVec2, Tile> = input
        .lines()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            rows = rows.max(line_idx + 1);
            line.chars()
                .enumerate()
                .map(|(c_idx, c)| {
                    cols = cols.max(c_idx + 1);
                    let pos = IVec2::new(c_idx as i32, line_idx as i32);
                    (
                        pos,
                        Tile::new(
                            pos,
                            match c {
                                '.' => None,
                                val => Some(val),
                            },
                        ),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let mut grid: TooDee<Tile> = TooDee::new(cols, rows);
    for x in 0..cols {
        for y in 0..rows {
            grid[x][y] = *tiles.get(&IVec2::new(x as i32, y as i32)).expect("exists")
        }
    }
    // at this point my grid is ready to be traversed by light ^^'
    let mut work_queue: VecDeque<(IVec2, Direction)> = VecDeque::new();
    work_queue.push_back((IVec2::new(0, 0), Direction::East));
    while !work_queue.is_empty() {
        let (pos, to) = work_queue.pop_front().expect("not empty yet");
        if let Some(new_positions) = grid[pos.x as usize][pos.y as usize].pass(to) {
            for new_pos in new_positions.into_iter() {
                if new_pos.x >= 0 && new_pos.y >= 0 {
                    let x = new_pos.x as usize;
                    let y = new_pos.y as usize;
                    let new_direction: Direction = Direction::from_points(pos, new_pos);
                    if x < cols && y < rows {
                        work_queue.push_back((new_pos, new_direction))
                    }
                }
            }
        }
    }
    grid.into_iter()
        .filter(|tile| tile.illuminated)
        .count()
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
        assert_eq!("46", process(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!("7498", process(input));
    }
}
