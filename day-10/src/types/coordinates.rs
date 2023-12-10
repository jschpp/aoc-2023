use super::directions::Direction;
use super::grid::Grid;

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coordinate {
    pub column: usize,
    pub line: usize,
}

impl Coordinate {
    /// checks whether a Coordinate is in the grid
    pub fn is_inside(&self, grid: &Grid) -> bool {
        self.column < grid.column_max && self.line < grid.line_max
    }

    ///
    pub fn calculate_coordinates(&self, direction: Direction) -> Option<Coordinate> {
        use Direction::*;
        let mut new_column = Some(self.column);
        let mut new_line = Some(self.line);
        match direction {
            North => {
                new_line = self.line.checked_sub(1);
            }
            South => {
                new_line = Some(self.line + 1);
            }
            West => {
                new_column = self.column.checked_sub(1);
            }
            East => {
                new_column = Some(self.column + 1);
            }
        };
        if let (Some(column), Some(line)) = (new_column, new_line) {
            Some(Coordinate { column, line })
        } else {
            None
        }
    }
}
