#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point3D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
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
