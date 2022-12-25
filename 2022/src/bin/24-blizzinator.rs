use aoc::*;
use image::{ImageBuffer, RgbImage};
use priority_queue::PriorityQueue;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    env,
    path::Path,
    rc::Rc,
};

struct Map {
    width: usize,
    height: usize,
    blizzards: Vec<(Point, Point)>,
    occupied: Rc<RefCell<HashMap<(usize, usize, usize), bool>>>,
}

impl From<&Path> for Map {
    fn from(filename: &Path) -> Self {
        let lines = read_lines(filename);

        let width = lines[0].len();
        let height = lines.len();

        let mut blizzards = Vec::new();

        for (y, line) in lines.into_iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let p = Point::new(x as isize, y as isize);
                match c {
                    '.' | '#' => {}
                    '^' => blizzards.push((p, Point::new(0, -1))),
                    'v' => blizzards.push((p, Point::new(0, 1))),
                    '<' => blizzards.push((p, Point::new(-1, 0))),
                    '>' => blizzards.push((p, Point::new(1, 0))),
                    _ => panic!("unknown map character {c}"),
                }
            }
        }

        Map {
            width,
            height,
            blizzards,
            occupied: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl Map {
    fn occupied(&self, x: usize, y: usize, t: usize) -> bool {
        // Constant walls
        if x == 0 || x == self.width - 1 {
            return true;
        }
        if y == 0 {
            return x != 1;
        }
        if y == self.height - 1 {
            return x != self.width - 2;
        }

        // Check cache
        if let Some(result) = self.occupied.borrow().get(&(x, y, t)) {
            return *result;
        }

        // Calculate blizzard positions, find if any is at that point and time
        // Top left is offset by 1 (at beginning and end) to account for top/left
        // Modulus is offset by 2 to account for both walls in each direction
        let mut is_occupied = false;

        let x_loop_fix = ((self.width - 2) * (1 + t / (self.width - 2))) as isize;
        let y_loop_fix = ((self.height - 2) * (1 + t / (self.height - 2))) as isize;

        for (origin, delta) in self.blizzards.iter() {
            if x == 1
                + (x_loop_fix + origin.x - 1 + delta.x * t as isize) as usize % (self.width - 2)
                && y == 1
                    + (y_loop_fix + origin.y - 1 + delta.y * t as isize) as usize
                        % (self.height - 2)
            {
                is_occupied = true;
                break;
            }
        }

        // Update cache and return
        self.occupied.borrow_mut().insert((x, y, t), is_occupied);
        is_occupied
    }

    #[allow(dead_code)]
    fn render(&self, t: usize) -> RgbImage {
        ImageBuffer::from_fn(self.width as u32, self.height as u32, |x, y| {
            if (x == 0 || x as usize == self.width - 1)
                || (y == 0 && x != 1)
                || (y as usize == self.height - 1 && x as usize != self.width - 2)
            {
                image::Rgb([127, 127, 127])
            } else if self.occupied(x as usize, y as usize, t) {
                image::Rgb([200, 233, 233])
            } else {
                image::Rgb([0, 0, 0])
            }
        })
    }

    fn render_path(&self, t: usize, path: &VecDeque<(usize, usize)>) -> RgbImage {
        ImageBuffer::from_fn(self.width as u32, self.height as u32, |x, y| {
            let on_path = path
                .iter()
                .enumerate()
                .find(|(pt, (px, py))| *pt <= t && *px == x as usize && *py == y as usize)
                .is_some();

            if (x == 0 || x as usize == self.width - 1)
                || (y == 0 && x != 1)
                || (y as usize == self.height - 1 && x as usize != self.width - 2)
            {
                image::Rgb([127, 127, 127])
            } else if self.occupied(x as usize, y as usize, t) {
                if on_path {
                    // Snow + path
                    image::Rgb([255, 233, 233])
                } else {
                    // Just snow
                    image::Rgb([200, 233, 233])
                }
            } else {
                if path[t] == (x as usize, y as usize) {
                    image::Rgb([255, 0, 0])
                } else if on_path {
                    image::Rgb([127, 0, 0])
                } else {
                    image::Rgb([0, 0, 0])
                }
            }
        })
    }
}

fn part1(filename: &Path) -> String {
    let map = Map::from(filename);

    type Point3u = (usize, usize, usize);

    let mut open = PriorityQueue::new();
    let mut closed = HashSet::new();
    let mut previous: HashMap<Point3u, Point3u> = HashMap::new();

    #[allow(unused_assignments)]
    let mut final_time = 0;

    open.push((1 as usize, 0 as usize, 0 as usize), 0 as isize);

    loop {
        let ((x, y, t), _) = open.pop().unwrap();
        closed.insert((x, y, t));

        // Solved
        if x == map.width - 2 && y == map.height - 1 {
            final_time = t;
            break;
        }

        for (xd, yd) in [(0 as isize, -1 as isize), (0, 1), (-1, 0), (1, 0), (0, 0)] {
            // Skip out of bounds cases
            if y == 0 && yd == -1 || (x == map.width - 2 && y == map.height - 1 && yd == 1) {
                continue;
            }

            let xp = (x as isize + xd) as usize;
            let yp = (y as isize + yd) as usize;
            let tp = t + 1;

            if closed.contains(&(xp, yp, tp)) {
                continue;
            }

            if map.occupied(xp, yp, tp) {
                continue;
            }

            if !previous.contains_key(&(xp, yp, tp)) || t < previous.get(&(xp, yp, tp)).unwrap().2 {
                previous.insert((xp, yp, tp), (x, y, t));
            }

            let d_remaining = map.width - xp - 2 + map.height - yp - 1;
            let t_guess = (tp as isize + d_remaining as isize) * -1;
            open.push((xp, yp, tp), t_guess);
        }
    }

    // Rebuild path
    if cfg!(debug_assertions) || env::var("AOC24_RENDER").is_ok() {
        let mut path = VecDeque::new();
        {
            let mut x = map.width - 2;
            let mut y = map.height - 1;
            let mut t = final_time;

            while !(x == 1 && y == 0) {
                path.push_front((x, y));
                (x, y, t) = previous[&(x, y, t)];
            }
        }

        if cfg!(debug_assertions) {
            println!("Final path: {path:?}");
        }

        if env::var("AOC24_RENDER").is_ok() {
            for t in 0..path.len() {
                map.render_path(t, &path)
                    .save(format!("{:08}.png", t))
                    .expect("failed to save frame");
            }
            make_mp4(10, String::from("aoc24-1-path"));
        }
    }

    final_time.to_string()
}

fn part2(filename: &Path) -> String {
    let map = Map::from(filename);

    type Point4u = (usize, usize, usize, usize);

    let mut open = PriorityQueue::new();
    let mut closed = HashSet::new();
    let mut previous: HashMap<Point4u, Point4u> = HashMap::new();

    #[allow(unused_assignments)]
    let mut final_time = 0;

    open.push((1 as usize, 0 as usize, 0 as usize, 0 as usize), 0 as isize);

    loop {
        let ((x, y, t, phase), _) = open.pop().unwrap();
        closed.insert((x, y, t));

        // Solved when we reach phase 3 and are at the exit
        // Phase 0 -> exit, 1 -> entrance, 2 -> exit, 3 is at exit
        if phase == 3 && x == map.width - 2 && y == map.height - 1 {
            final_time = t;
            break;
        }

        for (xd, yd) in [(0 as isize, -1 as isize), (0, 1), (-1, 0), (1, 0), (0, 0)] {
            // Skip out of bounds cases
            if y == 0 && yd == -1 || (x == map.width - 2 && y == map.height - 1 && yd == 1) {
                continue;
            }

            let xp = (x as isize + xd) as usize;
            let yp = (y as isize + yd) as usize;
            let tp = t + 1;

            if closed.contains(&(xp, yp, tp)) {
                continue;
            }

            if map.occupied(xp, yp, tp) {
                continue;
            }

            let mut d_remaining = 0;

            // Distance to next phase
            // On even phases, go to the exit; on odd, the entrance
            if phase % 2 == 0 {
                d_remaining += map.width - xp - 2 + map.height - yp - 1
            } else {
                d_remaining += xp - 1 + yp;
            };

            // Next phase if d is 0
            let pp = if d_remaining == 0 { phase + 1 } else { phase };

            if !previous.contains_key(&(xp, yp, tp, pp))
                || t < previous.get(&(xp, yp, tp, pp)).unwrap().2
            {
                previous.insert((xp, yp, tp, pp), (x, y, t, phase));
            }

            // Distance for unreached phases
            d_remaining += (2 - phase) * (map.width - 2 + map.height - 1);

            // Guessed best time for a*
            let t_guess = (tp as isize + d_remaining as isize) * -1;
            open.push((xp, yp, tp, pp), t_guess);
        }
    }

    // Rebuild path
    if cfg!(debug_assertions) || env::var("AOC24_RENDER").is_ok() {
        let mut path = VecDeque::new();
        {
            let mut x = map.width - 2;
            let mut y = map.height - 1;
            let mut t = final_time;
            let mut p = 3;

            while !(x == 1 && y == 0 && p == 0) {
                path.push_front((x, y));
                (x, y, t, p) = previous[&(x, y, t, p)];
            }
        }

        if cfg!(debug_assertions) {
            println!("Final path: {path:?}");
        }

        if env::var("AOC24_RENDER").is_ok() {
            for t in 0..path.len() {
                map.render_path(t, &path)
                    .save(format!("{:08}.png", t))
                    .expect("failed to save frame");
            }
            make_mp4(10, String::from("aoc24-2-path"));
        }
    }

    final_time.to_string()
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc::aoc_test;

    #[test]
    fn test1() {
        aoc_test("24", part1, "238")
    }

    #[test]
    fn test2() {
        aoc_test("24", part2, "751")
    }
}
