use std::fmt::{Debug, Display, Error, Formatter};
use std::hash::{Hash, Hasher};

use super::coordinates::Coordinate;
use super::directions::Direction;
use super::grid::Grid;
use super::shared::symbol_map;

#[derive(Clone, Copy, Eq)]
/// A Pipe. xD
pub struct Pipe {
    pub position: Coordinate,
    pub connections: Option<[Direction; 2]>,
    pub symbol: &'static char,
}

// only one node per coordinate
impl PartialEq for Pipe {
    fn eq(&self, other: &Pipe) -> bool {
        self.position == other.position
    }
}

/// Uses UTF Chars to prettyfy the pipes
impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", symbol_map(self.symbol))?;
        Ok(())
    }
}

/// Uses same symbols as given by the puzzle input
impl Debug for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.symbol)?;
        Ok(())
    }
}

// needed for pathfinding
impl Hash for Pipe {
    fn hash<H>(&self, s: &mut H)
    where
        H: Hasher,
    {
        self.position.hash(s)
    }
}

impl Pipe {
    /// creates coordinates of possible neighboring Pipes
    ///
    /// This will only check if the coordinates are sane (not negative)
    /// not if they are on the board or not
    ///
    /// # panics
    ///
    /// Will panic if the given pipe has no connections
    pub fn get_neighbor_coordinates(&self) -> Vec<Coordinate> {
        if let Some(connections) = self.connections {
            connections
                .iter()
                .flat_map(|conn| self.position.calculate_coordinates(*conn))
                .collect::<Vec<Coordinate>>()
        } else {
            // should not get here...
            panic!("tried getting neigbors for {:?}", self);
        }
    }

    /// returns neighboring pipes and a "cost" of 1
    pub fn successors(&self, grid: &Grid) -> Vec<(Pipe, usize)> {
        self.get_neighbor_coordinates()
            .iter()
            .filter(|coord| coord.is_inside(grid))
            .flat_map(|coord| grid[*coord].as_ref().map(|p| (*p, 1usize)))
            .collect()
    }
}
