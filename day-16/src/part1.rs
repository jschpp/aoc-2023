use super::shared::*;
use glam::IVec2;

pub fn process(input: &str) -> String {
    let mut grid = parse_into_grid(input);
    illuminate_grid(&mut grid, (IVec2::new(0, 0), Direction::East));
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
