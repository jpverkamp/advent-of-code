#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

impl Point2D {
    pub fn new(x: isize, y: isize) -> Self {
        Point2D { x, y }
    }
}

impl From<&str> for Point2D {
    fn from(s: &str) -> Self {
        let coords: Vec<isize> = s
            .split(',')
            .map(|part| part.trim().parse().unwrap())
            .collect();
        Point2D {
            x: coords[0],
            y: coords[1],
        }
    }
}

impl Point2D {
    pub fn distance_squared(&self, other: &Point2D) -> isize {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }
}

mod test {
    #[test]
    fn test_point2d_distance_squared() {
        let p1 = super::Point2D::new(1, 2);
        let p2 = super::Point2D::new(4, 6);
        let dist_sq = p1.distance_squared(&p2);
        assert_eq!(dist_sq, 25);
    }
}
