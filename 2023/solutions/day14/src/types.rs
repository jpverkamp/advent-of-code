use fxhash::FxHashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const NORTH: Point = Point { x: 0, y: -1 };
    pub const SOUTH: Point = Point { x: 0, y: 1 };
    pub const EAST: Point = Point { x: 1, y: 0 };
    pub const WEST: Point = Point { x: -1, y: 0 };
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

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

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

    fn include(&mut self, p: Point) {
        self.min_x = self.min_x.min(p.x);
        self.max_x = self.max_x.max(p.x);
        self.min_y = self.min_y.min(p.y);
        self.max_y = self.max_y.max(p.y);
    }
}

#[derive(Debug, Clone)]
pub struct Platform {
    pub bounds: Bounds,
    pub round_rocks: Vec<Point>,
    pub cube_rocks: FxHashSet<Point>,
}

impl From<&str> for Platform {
    fn from(input: &str) -> Self {
        let mut bounds = Bounds::default();
        let mut round_rocks = Vec::default();
        let mut cube_rocks = FxHashSet::default();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                bounds.include(Point {
                    x: x as isize,
                    y: y as isize,
                });
                match c {
                    'O' => {
                        round_rocks.push(Point {
                            x: x as isize,
                            y: y as isize,
                        });
                    }
                    '#' => {
                        cube_rocks.insert(Point {
                            x: x as isize,
                            y: y as isize,
                        });
                    }
                    _ => {}
                }
            }
        }

        Self {
            bounds,
            round_rocks,
            cube_rocks,
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.bounds.min_x..=self.bounds.max_y {
            for x in self.bounds.min_x..=self.bounds.max_x {
                let c = if self.round_rocks.contains(&Point { x, y }) {
                    'O'
                } else if self.cube_rocks.contains(&Point { x, y }) {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PlatformV2 {
    pub bounds: Bounds,
    pub round_rocks: Vec<Point>,
    pub occupied: FxHashSet<Point>,
}

impl From<Platform> for PlatformV2 {
    fn from(value: Platform) -> Self {
        let mut occupied = FxHashSet::default();
        for r in value.round_rocks.iter() {
            occupied.insert(*r);
        }
        for c in value.cube_rocks.iter() {
            occupied.insert(*c);
        }

        Self {
            bounds: value.bounds,
            round_rocks: value.round_rocks,
            occupied,
        }
    }
}

impl std::fmt::Display for PlatformV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.bounds.min_x..=self.bounds.max_y {
            for x in self.bounds.min_x..=self.bounds.max_x {
                let c = if self.round_rocks.contains(&Point { x, y }) {
                    'O'
                } else if self.occupied.contains(&Point { x, y }) {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
