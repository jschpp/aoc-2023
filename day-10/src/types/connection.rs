#[derive(Debug, Clone, Copy)]
pub struct Connection {
    pub direction: Direction,
    pub connected: bool,
}

impl Connection {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            connected: true,
        }
    }
}

