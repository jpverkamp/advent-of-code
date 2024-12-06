use crate::direction::Direction;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn neighbors(&self) -> [Self; 4] {
        [
            *self + Direction::Up,
            *self + Direction::Down,
            *self + Direction::Left,
            *self + Direction::Right,
        ]
    }
}

// Conversions

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Point {
        Point::new(x, y)
    }
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Point {
        Point::new(x as i32, y as i32)
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Point {
        Point::new(x as i32, y as i32)
    }
}

// Arithmetic operations on points

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::MulAssign<i32> for Point {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

// Interop between points and directions

impl From<Direction> for Point {
    fn from(direction: Direction) -> Point {
        match direction {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }
}

impl std::ops::Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        self + std::convert::Into::<Point>::into(rhs)
    }
}
