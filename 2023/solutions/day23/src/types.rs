use point::Point;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Slope {
    North,
    South,
    East,
    West,
}

impl From<Slope> for Point {
    fn from(slope: Slope) -> Self {
        match slope {
            Slope::North => Point::new(0, -1),
            Slope::South => Point::new(0, 1),
            Slope::East => Point::new(1, 0),
            Slope::West => Point::new(-1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Object {
    Wall,
    Slope(Slope),
}
