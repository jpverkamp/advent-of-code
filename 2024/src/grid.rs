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

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn iget(&self, x: isize, y: isize) -> Option<&T> {
        if x >= 0 && y >= 0 {
            self.get(x as usize, y as usize)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            Some(&mut self.data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> bool {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value;
            true
        } else {
            false
        }
    }
}
