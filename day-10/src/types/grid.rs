use std::fmt::{Debug, Error, Formatter};
use std::ops::{Index, IndexMut};
use strum::IntoEnumIterator;

use super::coordinates::Coordinate;
use super::directions::Direction;
use super::pipe::Pipe;
use super::shared::GridLike;

pub struct Grid {
    pub column_max: usize,
    pub line_max: usize,
    pub start: Option<Coordinate>,
    grid: Vec<Vec<Option<Pipe>>>,
}

impl Index<Coordinate> for Grid {
    type Output = Option<Pipe>;
    fn index(&self, index: Coordinate) -> &Self::Output {
        &self.grid[index.line][index.column]
    }
}

impl IndexMut<Coordinate> for Grid {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        &mut self.grid[index.line][index.column]
    }
}

// pretty printing for dummys
impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for line_idx in 0..self.line_max {
            for column_idx in 0..self.column_max {
                if let Some(pipe) = &self[Coordinate {
                    column: column_idx,
                    line: line_idx,
                }] {
                    write!(f, "{:?}", pipe)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl self::Grid {
    pub fn new(column_max: usize, line_max: usize) -> Self {
        let mut grid = vec![Vec::with_capacity(column_max); line_max];
        for line in grid.iter_mut().take(line_max) {
            for _ in 0..column_max {
                line.push(None);
            }
        }
        Self {
            column_max,
            line_max,
            start: None,
            grid,
        }
    }
    /// Tests all neighbors of the starting location and returns the connection "S" can make
    pub fn get_start_connections(&self) -> [Direction; 2] {
        let mut connections: Vec<Direction> = Vec::with_capacity(2);
        if let Some(start) = self.start {
            for direction in Direction::iter() {
                if let Some(dir_coord) = start.calculate_coordinates(direction) {
                    if let Some(pipe) = self[dir_coord] {
                        // pipe exists
                        if pipe
                            .connections
                            .expect("exists")
                            .iter()
                            .any(|conn| conn == &direction.opposite())
                        {
                            connections.push(direction);
                        }
                    }
                }
            }
        } else {
            panic!("no start position")
        }
        [connections[0], connections[1]]
    }

    pub fn contains(&self, c: Coordinate) -> bool {
        c.is_inside(self)
    }
}

impl GridLike for Grid {
    type Output = Pipe;

    fn get_empty_symbol(&self) -> char {
        ' '
    }

    fn get_item(&self, line: usize, column: usize) -> Option<Self::Output> {
        self[Coordinate { line, column }]
    }

    fn max_column(&self) -> usize {
        self.column_max
    }

    fn max_line(&self) -> usize {
        self.line_max
    }
}
