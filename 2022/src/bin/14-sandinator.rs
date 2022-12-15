use aoc::*;
use image::{ImageBuffer, RgbImage};
use std::{collections::HashSet, env, fmt::Display, path::Path};

#[derive(Debug)]
struct Sandbox {
    active_sand: HashSet<Point>,
    settled_sand: HashSet<Point>,
    walls: HashSet<Point>,

    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl<I> From<&mut I> for Sandbox
where
    I: Iterator<Item = String>,
{
    fn from(iter: &mut I) -> Self {
        let mut walls = HashSet::new();
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;

        for line in iter {
            let mut p = Point { x: 0, y: 0 };
            let mut first = true;

            for part in line.split(" -> ") {
                let mut xy = part.split(",");
                let new_p = Point {
                    x: xy
                        .next()
                        .expect("must have x")
                        .parse::<isize>()
                        .expect("x must be numeric"),
                    y: xy
                        .next()
                        .expect("must have x")
                        .parse::<isize>()
                        .expect("y must be numeric"),
                };

                if !first {
                    let delta = Point {
                        x: (new_p.x - p.x).signum(),
                        y: (new_p.y - p.y).signum(),
                    };

                    // Rather than while p != new_p, do this to get both edge cases
                    let mut done = false;
                    loop {
                        walls.insert(p);
                        min_x = isize::min(min_x, p.x - 1);
                        max_x = isize::max(max_x, p.x + 1);
                        min_y = isize::min(min_y, p.y - 1);
                        max_y = isize::max(max_y, p.y + 1);

                        if done {
                            break;
                        }

                        p = p + delta;
                        if p == new_p {
                            done = true;
                        }
                    }
                }

                p = new_p;
                first = false;
            }
        }

        Sandbox {
            active_sand: HashSet::new(),
            settled_sand: HashSet::new(),
            walls,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

impl Display for Sandbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();

        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let p = Point { x, y };

                buffer.push(if self.walls.contains(&p) {
                    '#'
                } else if self.settled_sand.contains(&p) {
                    'o'
                } else if self.active_sand.contains(&p) {
                    '*'
                } else {
                    '.'
                })
            }
            buffer.push('\n');
        }

        writeln!(f, "{}", buffer)
    }
}

impl Sandbox {
    fn occupied(&self, p: Point) -> bool {
        self.walls.contains(&p) || self.settled_sand.contains(&p) || self.active_sand.contains(&p)
    }

    fn drop(&mut self, p: Point) {
        self.min_x = isize::min(self.min_x, p.x - 1);
        self.max_x = isize::max(self.max_x, p.x + 1);
        self.min_y = isize::min(self.min_y, p.y - 1);
        self.max_y = isize::max(self.max_y, p.y + 1);

        self.active_sand.insert(p);
    }

    fn step(&mut self) -> bool {
        let mut next_active_sand = HashSet::new();

        for p in self.active_sand.iter() {
            // If we're past the lowest value, drop this point
            if p.y > self.max_y {
                return true;
            }

            // Otherwise, try to fall (first left than right)
            // If we can't fall, settle
            if !self.occupied(*p + Point::DOWN) {
                next_active_sand.insert(*p + Point::DOWN);
            } else if !self.occupied(*p + Point::DOWN + Point::LEFT) {
                next_active_sand.insert(*p + Point::DOWN + Point::LEFT);
            } else if !self.occupied(*p + Point::DOWN + Point::RIGHT) {
                next_active_sand.insert(*p + Point::DOWN + Point::RIGHT);
            } else {
                self.settled_sand.insert(*p);
            }
        }

        self.active_sand = next_active_sand;
        false
    }

    fn render(&self) -> RgbImage {
        let width = (self.max_x - self.min_x) as u32;
        let height = (self.max_y - self.min_y) as u32;

        ImageBuffer::from_fn(width, height, |x, y| {
            let p = Point {
                x: (x as isize) + self.min_x,
                y: (y as isize) + self.min_y,
            };
            if self.walls.contains(&p) {
                image::Rgb([127, 127, 127])
            } else if self.settled_sand.contains(&p) {
                image::Rgb([194, 178, 128])
            } else if self.active_sand.contains(&p) {
                image::Rgb([255, 255, 255])
            } else {
                image::Rgb([0, 0, 0])
            }
        })
    }
}

fn part1(filename: &Path) -> String {
    let mut sandbox = Sandbox::from(&mut iter_lines(filename));
    let drop = Point { x: 500, y: 0 };

    for _i in 1.. {
        if sandbox.active_sand.is_empty() {
            sandbox.drop(drop);
        }

        let done = sandbox.step();
        if done {
            break;
        }

        if env::var("AOC14_RENDER").is_ok() {
            sandbox
                .render()
                .save(format!("{:08}.png", _i))
                .expect("failed to save frame");
        }
    }

    if env::var("AOC14_RENDER").is_ok() {
        make_gif();
    }
    if cfg!(debug_assertions) {
        println!("[{}]\n{}", "final", sandbox);
    }

    sandbox.settled_sand.len().to_string()
}

fn part2(filename: &Path) -> String {
    let mut sandbox = Sandbox::from(&mut iter_lines(filename));
    let drop = Point { x: 500, y: 0 };

    // We want a line from -infinity,max_y+1 to +infinity,max_y
    // We don't actually need that though, just out at a 45 angle from min_x,min_y and max_x,min_y
    // Add some buffer for the extra offsets we're dealing with
    let height = sandbox.max_y - sandbox.min_y;
    let left_x = sandbox.min_x - height - 10;
    let right_x = sandbox.max_x + height + 10;

    for x in left_x..=right_x {
        sandbox.walls.insert(Point {
            x,
            y: sandbox.max_y + 1,
        });
    }
    sandbox.min_x = left_x - 1;
    sandbox.max_x = right_x + 1;
    sandbox.max_y += 2;

    for _i in 1.. {
        if sandbox.active_sand.is_empty() {
            sandbox.drop(drop);
        }

        let done = sandbox.step();
        if done || sandbox.occupied(drop) {
            break;
        }

        if env::var("AOC14_RENDER").is_ok() {
            sandbox
                .render()
                .save(format!("{:08}.png", _i))
                .expect("failed to save frame");
        }
    }

    if env::var("AOC14_RENDER").is_ok() {
        make_gif();
    }
    if cfg!(debug_assertions) {
        println!("[{}]\n{}", "final", sandbox);
    }
    sandbox.settled_sand.len().to_string()
}

fn make_gif() {
    use std::process::Command;

    let commands = vec![
        "ffmpeg -y -framerate 240 -i %08d.png -vf scale=iw*4:ih*4:flags=neighbor -c:v libx264 -r 30 sandbox.mp4",
        "find . -name '*.png' | xargs rm"
    ];

    for cmd in commands.into_iter() {
        println!("$ {}", cmd);
        let mut child = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .spawn()
            .expect("command failed");
        child.wait().expect("process didn't finish");
    }
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
        aoc_test("14", part1, "698")
    }

    #[test]
    fn test2() {
        aoc_test("14", part2, "28594")
    }
}
