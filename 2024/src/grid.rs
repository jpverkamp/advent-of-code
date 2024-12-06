use crate::point::Point;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    pub(crate) width: usize,
    pub(crate) height: usize,
    data: Vec<T>,
}

#[allow(dead_code)]
impl<T> Grid<T>
where
    T: Default + Clone + Sized,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Default::default(); width * height],
        }
    }

    pub fn read(input: &str, f: &dyn Fn(char) -> T) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut data = Vec::new();

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            width = line.len();
            height += 1;

            for c in line.chars() {
                data.push(f(c));
            }
        }

        Self {
            width,
            height,
            data,
        }
    }

    pub fn to_string(&self, f: &dyn Fn(&T) -> char) -> String {
        let mut s = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                s.push(f(&self.data[y * self.width + x]));
            }
            s.push('\n');
        }

        s
    }

    fn index(&self, p: &Point) -> usize {
        (p.y * self.width as i32 + p.x)
            .try_into()
            .expect("Index out of bounds")
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_enumerate(&self) -> impl Iterator<Item = (Point, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, v)| ((i % self.width, i / self.width).into(), v))
    }

    pub fn in_bounds(&self, p: impl Into<Point>) -> bool {
        let p = p.into();

        p.x >= 0 && p.x < (self.width as i32) && p.y >= 0 && p.y < (self.height as i32)
    }

    pub fn get(&self, p: impl Into<Point>) -> Option<&T> {
        let p = p.into();

        if !self.in_bounds(p) {
            return None;
        }

        Some(&self.data[self.index(&p)])
    }

    pub fn get_mut(&mut self, p: impl Into<Point>) -> Option<&mut T> {
        let p = p.into();

        if !self.in_bounds(p) {
            None
        } else {
            let index = self.index(&p);
            Some(&mut self.data[index])
        }
    }

    pub fn set(&mut self, p: impl Into<Point>, value: T) -> bool {
        let p = p.into();

        if !self.in_bounds(p) {
            return false;
        }

        let index = self.index(&p);
        self.data[index] = value;
        true
    }
}
