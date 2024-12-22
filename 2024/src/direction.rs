#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, ()> {
        match c.to_ascii_uppercase() {
            'N' | 'U' | '^' => Ok(Direction::Up),
            'S' | 'D' | 'V' => Ok(Direction::Down),
            'W' | 'L' | '<' => Ok(Direction::Left),
            'E' | 'R' | '>' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[allow(dead_code)]
impl Direction {
    pub fn rotate_cw(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn rotate_right(&self) -> Direction {
        self.rotate_cw()
    }

    pub fn rotate_ccw(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn rotate_left(&self) -> Direction {
        self.rotate_ccw()
    }

    pub fn flip(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }

    pub fn is_horizontal(&self) -> bool {
        !self.is_vertical()
    }

    pub fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}
