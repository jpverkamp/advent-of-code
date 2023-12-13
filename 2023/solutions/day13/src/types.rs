use fxhash::FxHashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn reflect_x(&self, axis: isize) -> Point {
        Point {
            x: if axis >= self.x {
                axis + (axis - self.x) + 1
            } else {
                axis - (self.x - axis) + 1
            },
            y: self.y,
        }
    }

    pub fn reflect_y(&self, axis: isize) -> Point {
        Point {
            x: self.x,
            y: if axis >= self.y {
                axis + (axis - self.y) + 1
            } else {
                axis - (self.y - axis) + 1
            },
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod point_test {
    use super::*;

    #[test]
    fn test_reflect_x() {
        // .p......r.
        // ----><----
        // 0123456789
        let p = Point { x: 1, y: 5 };
        assert_eq!(p.reflect_x(4), Point { x: 8, y: 5 });

        let p = Point { x: 8, y: 5 };
        assert_eq!(p.reflect_x(4), Point { x: 1, y: 5 });

        // ....pr....
        // ----><----
        // 0123456789
        let p = Point { x: 4, y: 7 };
        assert_eq!(p.reflect_x(4), Point { x: 5, y: 7 });

        let p = Point { x: 5, y: 7 };
        assert_eq!(p.reflect_x(4), Point { x: 4, y: 7 });
    }

    #[test]
    fn test_reflect_y() {
        // .p......r.
        // ----><----
        // 0123456789
        let p = Point { x: 5, y: 1 };
        assert_eq!(p.reflect_y(4), Point { x: 5, y: 8 });

        let p = Point { x: 5, y: 8 };
        assert_eq!(p.reflect_y(4), Point { x: 5, y: 1 });

        // ....pr....
        // ----><----
        // 0123456789
        let p = Point { x: 7, y: 4 };
        assert_eq!(p.reflect_y(4), Point { x: 7, y: 5 });

        let p = Point { x: 7, y: 5 };
        assert_eq!(p.reflect_y(4), Point { x: 7, y: 4 });
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
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

#[cfg(test)]
mod bounds_test {
    use super::*;

    #[test]
    fn test_contains() {
        let bounds = Bounds {
            min_x: 0,
            max_x: 9,
            min_y: 0,
            max_y: 9,
        };
        assert!(bounds.contains(&Point { x: 0, y: 0 }));
        assert!(bounds.contains(&Point { x: 9, y: 9 }));
        assert!(bounds.contains(&Point { x: 5, y: 5 }));
        assert!(!bounds.contains(&Point { x: -1, y: 0 }));
        assert!(!bounds.contains(&Point { x: 0, y: -1 }));
        assert!(!bounds.contains(&Point { x: 10, y: 0 }));
        assert!(!bounds.contains(&Point { x: 0, y: 10 }));
    }

    #[test]
    fn test_include() {
        let mut bounds = Bounds {
            min_x: 0,
            max_x: 9,
            min_y: 0,
            max_y: 9,
        };
        bounds.include(Point { x: 5, y: 5 });
        assert_eq!(
            bounds,
            Bounds {
                min_x: 0,
                max_x: 9,
                min_y: 0,
                max_y: 9
            }
        );
        bounds.include(Point { x: 10, y: 10 });
        assert_eq!(
            bounds,
            Bounds {
                min_x: 0,
                max_x: 10,
                min_y: 0,
                max_y: 10
            }
        );
        bounds.include(Point { x: -1, y: -1 });
        assert_eq!(
            bounds,
            Bounds {
                min_x: -1,
                max_x: 10,
                min_y: -1,
                max_y: 10
            }
        );
    }
}

#[derive(Debug)]
pub struct AshFlow {
    pub bounds: Bounds,
    pub rocks: FxHashSet<Point>,
}

impl From<&str> for AshFlow {
    fn from(input: &str) -> Self {
        let mut rocks = FxHashSet::default();
        let mut bounds = Bounds::default();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    let x = x as isize;
                    let y = y as isize;

                    rocks.insert(Point { x, y });
                    bounds.include(Point { x, y });
                }
            }
        }

        AshFlow { bounds, rocks }
    }
}
