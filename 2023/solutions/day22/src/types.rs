#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn manhattan_distance(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

#[cfg(test)]
mod point_test {
    use super::*;

    #[test]
    fn test_point() {
        let point = Point { x: 1, y: 2, z: 3 };
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
        assert_eq!(point.z, 3);
    }

    #[test]
    fn test_manhattan_distance() {
        let p1 = Point { x: 1, y: 2, z: 3 };
        let p2 = Point { x: 2, y: 3, z: 4 };
        assert_eq!(p1.manhattan_distance(&p2), 3);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Block {
    pub min: Point,
    pub max: Point,
}

impl Block {
    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, point: Point) -> bool {
        self.min.x <= point.x
            && self.min.y <= point.y
            && self.min.z <= point.z
            && self.max.x >= point.x
            && self.max.y >= point.y
            && self.max.z >= point.z
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.min.y <= other.max.y
            && self.min.z <= other.max.z
            && self.max.x >= other.min.x
            && self.max.y >= other.min.y
            && self.max.z >= other.min.z
    }

    pub fn name(&self, blocks: &[Block]) -> String {
        let index = blocks.iter().position(|b| b == self).unwrap();
        if index <= 26 {
            format!("{}({index})", (b'A' + index as u8) as char)
        } else if index <= 52 {
            format!("{}({index})", (b'a' + index as u8) as char)
        } else {
            format!("{index}")
        }
    }
}

impl std::ops::Add<Point> for Block {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Self {
            min: Point::new(self.min.x + rhs.x, self.min.y + rhs.y, self.min.z + rhs.z),
            max: Point::new(self.max.x + rhs.x, self.max.y + rhs.y, self.max.z + rhs.z),
        }
    }
}

#[cfg(test)]
mod block_test {
    #[test]
    fn test_block_contains() {
        use super::*;
        let block = Block::new(Point::new(0, 0, 0), Point::new(1, 1, 1));
        assert!(block.contains(Point::new(0, 0, 0)));
        assert!(block.contains(Point::new(1, 1, 1)));
        assert!(block.contains(Point::new(0, 0, 1)));
        assert!(block.contains(Point::new(1, 1, 0)));
        assert!(!block.contains(Point::new(2, 2, 2)));
        assert!(!block.contains(Point::new(-1, -1, -1)));
    }

    #[test]
    fn test_block_intersects() {
        use super::*;
        let block = Block::new(Point::new(0, 0, 0), Point::new(2, 3, 4));
        assert!(block.intersects(&block));
        assert!(block.intersects(&Block::new(Point::new(1, 1, 1), Point::new(3, 4, 5))));
        assert!(block.intersects(&Block::new(Point::new(-1, -1, -1), Point::new(1, 2, 3))));
        assert!(block.intersects(&Block::new(Point::new(0, 0, 0), Point::new(0, 0, 0))));
        assert!(!block.intersects(&Block::new(Point::new(3, 4, 5), Point::new(4, 5, 6))));
        assert!(!block.intersects(&Block::new(Point::new(-3, -4, -5), Point::new(-2, -3, -4))));
    }

    #[test]
    fn test_block_add_point() {
        use super::*;
        let block = Block::new(Point::new(0, 0, 0), Point::new(2, 3, 4));
        assert_eq!(
            block + Point::new(1, 1, 1),
            Block::new(Point::new(1, 1, 1), Point::new(3, 4, 5))
        );
    }
}
