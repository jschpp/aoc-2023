use std::fmt::{Debug, Display};
use colored::Colorize;

use crate::types::shared::{symbol_map, GridLike};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Flood {
    Pipe { x: char },
    Outside,
    Inside,
}

impl Flood {
    /// Flips Inside to Outside
    ///
    /// ```
    /// use day_10::types::flood::Flood::*;
    /// assert_eq!(Outside, Inside.flip());
    /// assert_eq!(Inside, Outside.flip());
    /// ```
    ///
    /// # Panics
    ///
    /// Will panic when Flood is a Pipe
    ///
    /// ```rust,should_panic
    /// use day_10::types::flood::Flood::*;
    /// // panics
    /// Pipe{x: 'x'}.flip();
    /// ```
    pub fn flip(self) -> Self {
        match self {
            Self::Inside => Self::Outside,
            Self::Outside => Self::Inside,
            Self::Pipe { x: _ } => panic!("not supported"),
        }
    }
}

impl Debug for Flood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flood::Inside => write!(f, "{}", "I".red()),
            Flood::Outside => write!(f, "O"),
            Flood::Pipe { x } => write!(f, "{:?}", x),
        }
    }
}

impl Display for Flood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flood::Inside => write!(f, "{}", "I".red()),
            Flood::Outside => write!(f, " "),
            Flood::Pipe { x } => write!(f, "{}", symbol_map(x)),
        }
    }
}

pub struct FloodGrid {
    max_line: usize,
    max_column: usize,
    pub grid: Vec<Vec<Option<Flood>>>,
}

impl FloodGrid {
    pub fn new(max_line: usize, max_column: usize) -> Self {
        let grid: Vec<Vec<Option<Flood>>> = (0..max_line).map(|_| vec![None; max_column]).collect();
        FloodGrid {
            max_line,
            max_column,
            grid,
        }
    }
}

impl GridLike for FloodGrid {
    type Output = Flood;

    fn get_item(&self, line: usize, column: usize) -> Option<Self::Output> {
        self.grid[line][column]
    }

    fn max_column(&self) -> usize {
        self.max_column
    }

    fn max_line(&self) -> usize {
        self.max_line
    }

    fn get_empty_symbol(&self) -> char {
        ' '
    }
}
