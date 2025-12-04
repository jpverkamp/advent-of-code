#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    width: isize,
    height: isize,
    data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn read(s: &str, f: impl Fn(char) -> T) -> Grid<T> {
        let mut width = 0;
        let mut height = 0;
        let mut data = vec![];

        for line in s.lines() {
            if line.is_empty() {
                continue;
            }

            height += 1;

            let mut line_width = 0;
            for c in line.chars() {
                data.push(f(c));
                line_width += 1;
            }

            if height == 1 {
                width = line_width
            } else {
                assert_eq!(width, line_width, "Line {line} is the wrong width");
            }
        }

        Grid {
            width,
            height,
            data,
        }
    }

    pub fn width(&self) -> isize {
        self.width
    }

    pub fn height(&self) -> isize {
        self.height
    }

    pub fn get(&self, x: isize, y: isize) -> Option<T> {
        if self.in_bounds(x, y) {
            Some(self.data[(x + y * self.width) as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: isize, y: isize, v: T) {
        if self.in_bounds(x, y) {
            self.data[(x + y * self.width) as usize] = v;
        } else {
            panic!("Set out of bounds");
        }
    }

    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn iter(&self) -> impl Iterator<Item = (isize, isize, T)> {
        (0..self.width)
            .flat_map(move |x| (0..self.height).map(move |y| (x, y, self.get(x, y).unwrap())))
    }

    pub fn map(&self, f: impl Fn(isize, isize, T) -> T) -> Grid<T> {
        Grid {
            width: self.width,
            height: self.height,
            data: self.iter().map(|(x, y, v)| f(x, y, v)).collect::<Vec<_>>(),
        }
    }

    pub fn neighbors(&self, x: isize, y: isize) -> impl Iterator<Item = Option<T>> {
        [
            (-1_isize, -1_isize),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
        .into_iter()
        .map(move |(xd, yd)| self.get(x + xd, y + yd))
    }

    pub fn ortho_neighbors(&self, x: isize, y: isize) -> impl Iterator<Item = Option<T>> {
        [(0_isize, -1_isize), (-1, 0), (1, 0), (0, 1)]
            .into_iter()
            .map(move |(xd, yd)| self.get(x + xd, y + yd))
    }
}
