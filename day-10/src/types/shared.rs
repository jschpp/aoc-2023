use std::fmt::Display;

pub trait GridLike {
    type Output;

    fn get_item(&self, line: usize, column: usize) -> Option<Self::Output>;
    fn max_column(&self) -> usize;
    fn max_line(&self) -> usize;
    fn get_empty_symbol(&self) -> char;

    fn pretty_print(&self)
    where
        Self::Output: Display,
    {
        for line_idx in 0..self.max_line() {
            for column_idx in 0..self.max_column() {
                let symbol = if let Some(item) = self.get_item(line_idx, column_idx) {
                    format!("{}", item)
                } else {
                    format!("{}", self.get_empty_symbol())
                };
                print!("{}", symbol)
            }
            println!();
        }
    }
}

pub fn symbol_map(symbol: &char) -> char {
    match symbol {
        '-' => '─',
        '|' => '│',
        'F' => '┌',
        '7' => '┐',
        'L' => '└',
        'J' => '┘',
        'S' => 'S',
        val => panic!("trying to match {}", val),
    }
}
