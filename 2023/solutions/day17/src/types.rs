use point::Point;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Default for Direction {
    fn default() -> Self {
        Self::South
    }
}

impl Direction {
    pub fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }
}

impl From<Direction> for Point {
    fn from(d: Direction) -> Self {
        match d {
            Direction::North => Point::NORTH,
            Direction::South => Point::SOUTH,
            Direction::East => Point::EAST,
            Direction::West => Point::WEST,
        }
    }
}
