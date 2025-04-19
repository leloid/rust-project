#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    Science,
}

impl Cell {
    pub fn to_symbol(&self) -> &'static str {
        match self {
            Cell::Empty => " E ",
            Cell::Obstacle => " O ",
            Cell::Energy => " P ",
            Cell::Mineral => " M ",
            Cell::Science => " S ",
        }
    }

    pub fn to_colored_symbol(&self) -> &'static str {
        match self {
            Cell::Empty => " E ",
            Cell::Obstacle => "\x1b[90m O \x1b[0m ",
            Cell::Energy => "\x1b[33m P \x1b[0m ",
            Cell::Mineral => "\x1b[35m M \x1b[0m ",
            Cell::Science => "\x1b[36m S \x1b[0m ",
        }
    }
}
