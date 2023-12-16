use bounds::Bounds;
use fxhash::FxHashMap;
use point::Point;

#[derive(Debug)]
pub struct Grid<T> {
    pub bounds: Bounds,
    data: FxHashMap<Point, T>,
}

impl<T: Default> Default for Grid<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self {
            bounds: Bounds::default(),
            data: FxHashMap::default(),
        }
    }

    pub fn read(s: &str, from_c: impl Fn(char) -> Option<T>) -> Self {
        let mut grid = Self::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(c) = from_c(c) {
                    grid.insert(
                        Point {
                            x: x as isize,
                            y: y as isize,
                        },
                        c,
                    );
                }
            }
        }
        grid
    }

    pub fn get(&self, point: &Point) -> Option<&T> {
        self.data.get(point)
    }

    pub fn get_mut(&mut self, point: &Point) -> Option<&mut T> {
        self.data.get_mut(point)
    }

    pub fn insert(&mut self, point: Point, value: T) {
        self.bounds.include(point);
        self.data.insert(point, value);
    }

    pub fn remove(&mut self, point: &Point) -> Option<T> {
        self.data.remove(point)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Point, &T)> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Point, &mut T)> {
        self.data.iter_mut()
    }

    pub fn iter_points(&self) -> impl Iterator<Item = &Point> {
        self.data.keys()
    }

    pub fn iter_values(&self) -> impl Iterator<Item = &T> {
        self.data.values()
    }

    pub fn iter_values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.values_mut()
    }
}
