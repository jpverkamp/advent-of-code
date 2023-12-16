use point::Point;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Mirror {
    VerticalSplitter,
    HorizontalSplitter,
    ForwardReflector,
    BackwardReflector,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
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
