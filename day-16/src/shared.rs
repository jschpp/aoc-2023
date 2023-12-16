use glam::IVec2;
use std::{collections::HashMap, fmt::Debug, ops::Add};
use toodee::TooDee;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Add<Direction> for IVec2 {
    type Output = IVec2;
    fn add(self, rhs: Direction) -> Self::Output {
        self + Into::<IVec2>::into(rhs)
    }
}

impl TryFrom<IVec2> for Direction {
    type Error = &'static str;
    fn try_from(value: IVec2) -> Result<Self, Self::Error> {
        let value = value.clamp(IVec2 { x: -1, y: -1 }, IVec2 { x: 1, y: 1 });
        match (value.x, value.y) {
            (0, 0) => Err("did not move"),
            (1, 0) => Ok(Direction::East),
            (-1, 0) => Ok(Direction::West),
            (0, 1) => Ok(Direction::South),
            (0, -1) => Ok(Direction::North),
            _ => Err("move not defined"),
        }
    }
}

impl From<Direction> for IVec2 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Self::new(0, -1),
            Direction::South => Self::new(0, 1),
            Direction::East => Self::new(1, 0),
            Direction::West => Self::new(-1, 0),
        }
    }
}

impl Direction {
    pub fn to_point(self) -> IVec2 {
        self.into()
    }

    /// Converts from two points into a Direction
    ///
    /// # panics
    /// Will panic when trying to move diagonal or if `from==to`
    pub fn from_points(from: IVec2, to: IVec2) -> Self {
        let movement = to - from;
        movement.try_into().unwrap()
    }
}

#[derive(Clone, Copy, Default)]
pub struct Tile {
    pub position: IVec2,
    pub mirror: Option<char>,
    pub illuminated: bool,
    passed: [bool; 4],
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.mirror.unwrap_or('.');
        write!(f, "{}", c)
    }
}

impl Tile {
    pub fn new(position: IVec2, mirror: Option<char>) -> Self {
        Self {
            position,
            mirror,
            passed: [false; 4],
            illuminated: false,
        }
    }

    pub fn pass(&mut self, to: Direction) -> Option<Vec<IVec2>> {
        use Direction::*;
        if self.passed[to as usize] {
            return None;
        }
        self.passed[to as usize] = true;
        self.illuminated = true;
        if self.mirror.is_none() {
            Some(vec![self.position + to.to_point()])
        } else {
            let mirror = self.mirror.expect("mirror is not None here");
            match mirror {
                '|' => match to {
                    North | South => Some(vec![self.position + to]),
                    East | West => Some(vec![
                        self.position + North.to_point(),
                        self.position + South.to_point(),
                    ]),
                },
                '/' => match to {
                    North => Some(vec![self.position + East]),
                    South => Some(vec![self.position + West]),
                    West => Some(vec![self.position + South]),
                    East => Some(vec![self.position + North]),
                },
                '\\' => match to {
                    North => Some(vec![self.position + West]),
                    South => Some(vec![self.position + East]),
                    West => Some(vec![self.position + North]),
                    East => Some(vec![self.position + South]),
                },
                '-' => match to {
                    North | South => Some(vec![
                        self.position + East.to_point(),
                        self.position + West.to_point(),
                    ]),
                    East | West => Some(vec![self.position + to]),
                },
                _ => unreachable!(),
            }
        }
    }
}

pub fn parse_into_grid(input: &str) -> TooDee<Tile> {
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
    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn tile(#[default(None)] mirror: Option<char>) -> Tile {
        Tile::new(IVec2::new(0, 0), mirror)
    }

    #[rstest]
    #[case::going_north(Direction::North, Some(vec![Direction::East.to_point(), Direction::West.to_point()]))]
    #[case::going_south(Direction::South, Some(vec![Direction::East.to_point(), Direction::West.to_point()]))]
    #[case::going_east(Direction::East, Some(vec![Direction::East.to_point()]))]
    #[case::going_west(Direction::West, Some(vec![Direction::West.to_point()]))]
    fn test_horizontal_mirror(
        #[case] to: Direction,
        #[with(Some('-'))] tile: Tile,
        #[case] expected: Option<Vec<IVec2>>,
    ) {
        let mut tile: Tile = tile.clone();
        assert_eq!(tile.pass(to), expected)
    }

    #[rstest]
    #[case::going_north(Direction::North, Some(vec![Direction::North.to_point()]))]
    #[case::going_south(Direction::South, Some(vec![Direction::South.to_point()]))]
    #[case::going_east(Direction::East, Some(vec![Direction::North.to_point(), Direction::South.to_point()]))]
    #[case::going_west(Direction::West, Some(vec![Direction::North.to_point(), Direction::South.to_point()]))]
    fn test_vertical_mirror(
        #[case] to: Direction,
        #[with(Some('|'))] tile: Tile,
        #[case] expected: Option<Vec<IVec2>>,
    ) {
        let mut tile: Tile = tile.clone();
        assert_eq!(tile.pass(to), expected)
    }

    #[rstest]
    #[case::going_north(Direction::North, Some(vec![Direction::East.to_point()]))]
    #[case::going_south(Direction::South, Some(vec![Direction::West.to_point()]))]
    #[case::going_east(Direction::East, Some(vec![Direction::North.to_point()]))]
    #[case::going_west(Direction::West, Some(vec![Direction::South.to_point()]))]
    fn test_slash_mirror(
        #[case] to: Direction,
        #[with(Some('/'))] tile: Tile,
        #[case] expected: Option<Vec<IVec2>>,
    ) {
        let mut tile: Tile = tile.clone();
        assert_eq!(tile.pass(to), expected)
    }

    #[rstest]
    #[case::going_north(Direction::North, Some(vec![Direction::West.to_point()]))]
    #[case::going_south(Direction::South, Some(vec![Direction::East.to_point()]))]
    #[case::going_east(Direction::East, Some(vec![Direction::South.to_point()]))]
    #[case::going_west(Direction::West, Some(vec![Direction::North.to_point()]))]
    fn test_backslash_mirror(
        #[case] to: Direction,
        #[with(Some('\\'))] tile: Tile,
        #[case] expected: Option<Vec<IVec2>>,
    ) {
        let mut tile: Tile = tile.clone();
        assert_eq!(tile.pass(to), expected)
    }

    #[rstest]
    #[case(Direction::North, 0)]
    #[case(Direction::South, 1)]
    #[case(Direction::East, 2)]
    #[case(Direction::West, 3)]
    fn test_enum_conversion(#[case] direction: Direction, #[case] expected: usize) {
        // this is a sanity check since the order will matter for the future
        assert_eq!(direction as usize, expected)
    }
}
