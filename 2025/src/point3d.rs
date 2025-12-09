#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point3D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point3D {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Point3D { x, y, z }
    }
}

impl From<&str> for Point3D {
    fn from(s: &str) -> Self {
        let coords: Vec<isize> = s
            .split(',')
            .map(|part| part.trim().parse().unwrap())
            .collect();
        Point3D {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

impl Point3D {
    pub fn distance_squared(&self, other: &Point3D) -> isize {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

mod test {
    #[test]
    fn test_point3d_distance_squared() {
        let p1 = super::Point3D::new(1, 2, 3);
        let p2 = super::Point3D::new(4, 6, 8);
        let dist_sq = p1.distance_squared(&p2);
        assert_eq!(dist_sq, 50);
    }
}
