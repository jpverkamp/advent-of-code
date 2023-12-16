use point::Point;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
pub struct Bounds {
    pub min_x: isize,
    pub max_x: isize,
    pub min_y: isize,
    pub max_y: isize,
}

impl Bounds {
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.min_x
            && point.x <= self.max_x
            && point.y >= self.min_y
            && point.y <= self.max_y
    }

    pub fn include(&mut self, p: Point) {
        self.min_x = self.min_x.min(p.x);
        self.max_x = self.max_x.max(p.x);
        self.min_y = self.min_y.min(p.y);
        self.max_y = self.max_y.max(p.y);
    }
}
