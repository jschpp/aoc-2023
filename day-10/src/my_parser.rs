use super::types::coordinates::Coordinate;
use super::types::directions::Direction;
use super::types::grid::Grid;
use super::types::pipe::Pipe;

const SYMBOLS: [char; 7] = ['|', '-', 'L', 'J', '7', 'F', 'S'];

pub fn get_symbol_for_s(p: &Pipe) -> &'static char {
    use Direction::*;
    let conn = p.connections.expect("exists");
    match conn {
        [North, South] => &SYMBOLS[0],
        [East, West] => &SYMBOLS[1],
        [North, East] => &SYMBOLS[2],
        [North, West] => &SYMBOLS[3],
        [South, West] => &SYMBOLS[4],
        [South, East] => &SYMBOLS[5],
        val => panic!("mapping failed for {:?}", val),
    }
}

/// Parse Pipe Chars
fn pipe(input: char, coord: Coordinate) -> Pipe {
    use Direction::*;
    match input {
        '|' => Pipe {
            position: coord,
            connections: Some([North, South]),
            symbol: &SYMBOLS[0],
        },
        '-' => Pipe {
            position: coord,
            connections: Some([East, West]),
            symbol: &SYMBOLS[1],
        },
        'L' => Pipe {
            position: coord,
            connections: Some([North, East]),
            symbol: &SYMBOLS[2],
        },
        'J' => Pipe {
            position: coord,
            connections: Some([North, West]),
            symbol: &SYMBOLS[3],
        },
        '7' => Pipe {
            position: coord,
            connections: Some([South, West]),
            symbol: &SYMBOLS[4],
        },
        'F' => Pipe {
            position: coord,
            connections: Some([South, East]),
            symbol: &SYMBOLS[5],
        },
        'S' => Pipe {
            position: coord,
            connections: None,
            symbol: &SYMBOLS[6],
        },
        val => panic!("found {:?} on {:?}", val, coord),
    }
}

pub fn my_parser(input: &str) -> Grid {
    let mut x_max: usize = 0;
    let mut y_max: usize = 0;
    let pipes = input
        .lines()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            let current_line = line
                .chars()
                .enumerate()
                .flat_map(|(colum_idx, c)| {
                    x_max = x_max.max(colum_idx + 1);
                    if c != '.' {
                        let coord = Coordinate {
                            column: colum_idx,
                            line: line_idx,
                        };
                        Some(pipe(c, coord))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Pipe>>();
            y_max = y_max.max(line_idx + 1);
            current_line
        })
        .collect::<Vec<Pipe>>();

    let mut grid = Grid::new(x_max, y_max);
    for pipe in pipes.iter() {
        if pipe.symbol == &'S' {
            grid.start = Some(pipe.position);
        }
        grid[pipe.position] = Some(pipe.to_owned());
    }
    let start = grid.start.expect("by now start node should exist");
    grid[start].as_mut().expect("node exists").connections = Some(grid.get_start_connections());
    grid
}
