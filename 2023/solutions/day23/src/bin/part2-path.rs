use anyhow::Result;
use std::{cell::RefCell, io, rc::Rc};

use grid::Grid;
use point::Point;

// #[aoc_test("data/test/23.txt", "154")]
// #[aoc_test("data/23.txt", "")]
fn main() {
    env_logger::init();
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let grid = Grid::read(input, |c| match c {
        '#' => Some(true),
        _ => None,
    });

    #[derive(Debug)]
    struct State {
        position: Point,
        path: Path,
    }

    let mut queue = Vec::new();

    let start = Point::new(1, 0);
    queue.push(State {
        position: start,
        path: Path::new(start),
    });

    let mut complete = Vec::new();

    let start = std::time::Instant::now();
    let mut count = 0;
    while let Some(mut state) = queue.pop() {
        count += 1;
        if count % 100_000 == 0 {
            log::info!("{:?} {:?}", count, start.elapsed());
        }

        for direction in &[
            Point::new(0, 1),
            Point::new(0, -1),
            Point::new(1, 0),
            Point::new(-1, 0),
        ] {
            let next_position = state.position + *direction;

            // If we're at the exit, we've found a complete path
            if next_position == Point::new(grid.bounds.max_x - 1, grid.bounds.max_y) {
                complete.push(state.path.clone());
                continue;
            }

            // If we're out of bounds, we've found an invalid path
            if !grid.bounds.contains(&next_position) {
                continue;
            }

            // Cannot go through walls
            if grid.get(&next_position).is_some() {
                continue;
            }

            // Cannot visit the same point more than once
            if state.path.contains(next_position) {
                continue;
            }

            // Otherwise, queue it up
            let new_state = State {
                position: next_position,
                path: state.path.extend(next_position),
            };
            queue.push(new_state);
        }
    }

    // Find the longest path
    Ok((complete
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len())
    .to_string())
}

/* Custom implementation of path */

#[derive(Debug)]
struct PathData {
    points: Vec<Point>,
    froms: Vec<Option<usize>>,
}

#[derive(Debug, Clone)]
pub struct Path {
    path: Rc<RefCell<PathData>>,
    index: usize,
    length: usize,
}

impl Path {
    pub fn new(p: Point) -> Self {
        Path {
            path: Rc::new(RefCell::new(PathData {
                points: vec![p],
                froms: vec![None],
            })),
            index: 0,
            length: 1,
        }
    }

    pub fn extend(&mut self, p: Point) -> Path {
        self.path.borrow_mut().points.push(p);
        self.path.borrow_mut().froms.push(Some(self.index));

        Path {
            path: self.path.clone(),
            index: self.path.borrow().points.len() - 1,
            length: self.length + 1,
        }
    }

    pub fn len(&self) -> usize {
        // // Count the number of points in the path
        // // Current point is why this starts at 1
        // let mut index = self.index;
        // let mut len = 1;

        // while let Some(from) = self.path.borrow().froms[index] {
        //     index = from;
        //     len += 1;
        // }

        // len

        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn contains(&self, p: Point) -> bool {
        // Check the current point
        if self.path.borrow().points[self.index] == p {
            return true;
        }

        // Check previous points until we reach the start
        let mut index = self.index;
        while let Some(from) = self.path.borrow().froms[index] {
            if self.path.borrow().points[index] == p {
                return true;
            }
            index = from;
        }
        false
    }
}

#[cfg(test)]
mod path_test {
    #[test]
    fn test_create_path() {
        use super::Path;
        use point::Point;

        let p = Point::new(0, 0);
        let path = Path::new(p);
        assert_eq!(path.len(), 1);
        assert!(path.contains(p));
    }

    #[test]
    fn test_longer_path() {
        use super::Path;
        use point::Point;

        let p = Point::new(0, 0);
        let mut path = Path::new(p);
        let p = Point::new(1, 0);
        path = path.extend(p);
        let p = Point::new(2, 0);
        path = path.extend(p);
        assert_eq!(path.len(), 3);
        assert!(path.contains(p));
    }

    #[test]
    fn test_branching_path() {
        use super::Path;
        use point::Point;

        let p = Point::new(0, 0);
        let mut path = Path::new(p);
        let p = Point::new(1, 0);
        path = path.extend(p);

        let p = Point::new(2, 0);
        let mut path_a = path.extend(p);
        let p = Point::new(2, 1);
        path_a = path_a.extend(p);

        let p = Point::new(1, 1);
        let mut path_b = path.extend(p);
        let p = Point::new(1, 2);
        path_b = path_b.extend(p);
        let p = Point::new(1, 3);
        path_b = path_b.extend(p);

        assert_eq!(path_a.len(), 4);

        assert!(path_a.contains(Point::new(1, 0)));
        assert!(path_a.contains(Point::new(2, 1)));
        assert!(!path_b.contains(Point::new(2, 1)));

        assert_eq!(path_b.len(), 5);
        assert!(path_b.contains(Point::new(1, 0)));
        assert!(path_b.contains(Point::new(1, 2)));
        assert!(!path_a.contains(Point::new(1, 2)));
    }
}
