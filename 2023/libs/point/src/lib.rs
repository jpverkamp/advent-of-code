#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const ORIGIN: Point = Point { x: 0, y: 0 };
    pub const NORTH: Point = Point { x: 0, y: -1 };
    pub const SOUTH: Point = Point { x: 0, y: 1 };
    pub const EAST: Point = Point { x: 1, y: 0 };
    pub const WEST: Point = Point { x: -1, y: 0 };

    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn neighbors(&self) -> IterNeighbors {
        IterNeighbors {
            point: *self,
            index: 0,
        }
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<Point> for isize {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct IterNeighbors {
    point: Point,
    index: usize,
}

impl Iterator for IterNeighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(self.point + Point::NORTH),
            1 => Some(self.point + Point::SOUTH),
            2 => Some(self.point + Point::EAST),
            3 => Some(self.point + Point::WEST),
            _ => None,
        };

        self.index += 1;

        result
    }
}
