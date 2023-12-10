use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, EnumIter)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Self::East => Self::West,
            Self::West => Self::East,
            Self::North => Self::South,
            Self::South => Self::North,
        }
    }
}
