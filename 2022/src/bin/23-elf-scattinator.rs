use aoc::*;
use image::{ImageBuffer, RgbImage};
use std::{collections::HashMap, env, fmt::Display, path::Path};

#[derive(Clone, Debug)]
struct Elves {
    locations: HashMap<Point, usize>,
}

impl From<&Path> for Elves {
    fn from(filename: &Path) -> Self {
        let mut points = HashMap::new();

        for (y, line) in iter_lines(filename).enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    points.insert(Point::new(x as isize, y as isize), 0);
                }
            }
        }

        Elves { locations: points }
    }
}

impl Display for Elves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [min_x, max_x, min_y, max_y] = self.bounds();

        let mut buffer = String::new();
        buffer.push_str(
            format!(
                "Elves<count: {}, bounds: {min_x}..{max_x}, {min_y}..{max_y}, data:\n",
                self.locations.len()
            )
            .as_str(),
        );

        buffer.push(' ');
        for x in (min_x - 1)..=(max_x + 1) {
            if x == 0 {
                buffer.push('0');
            } else {
                buffer.push(' ');
            }
        }
        buffer.push('\n');

        for y in (min_y - 1)..=(max_y + 1) {
            if y == 0 {
                buffer.push('0');
            } else {
                buffer.push(' ');
            }

            for x in (min_x - 1)..=(max_x + 1) {
                if self.locations.contains_key(&Point::new(x, y)) {
                    buffer.push('#');
                } else {
                    buffer.push('.');
                }
            }
            buffer.push('\n');
        }
        buffer.push('>');

        write!(f, "{}\n", buffer)
    }
}

impl Elves {
    fn bounds(&self) -> [isize; 4] {
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;

        for (p, _) in self.locations.iter() {
            min_x = min_x.min(p.x);
            max_x = max_x.max(p.x);
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
        }

        [min_x, max_x, min_y, max_y]
    }

    fn step(&mut self, round: usize) -> bool {
        // First, calculate an updated set of points
        let mut moves = Vec::new();

        'next_elf: for (elf, _) in self.locations.iter() {
            // If an elf doesn't have any neighbors, don't move
            // This is important, I forgot it and got really confused
            // Counts self, so neighbors will always >= 1
            let mut neighbors = 0;
            for xd in -1..=1 {
                for yd in -1..=1 {
                    if self.locations.contains_key(&(*elf + Point::new(xd, yd))) {
                        neighbors += 1;
                    }
                }
            }
            if neighbors == 1 {
                moves.push((*elf, *elf));
                continue 'next_elf;
            }

            // Try to move each direction until we find an empty on
            for check in 0..4 {
                let direction = Direction::proposal(round, check);

                // All three checks in this direction must be empty
                if direction
                    .check()
                    .iter()
                    .any(|p| self.locations.contains_key(&(*elf + *p)))
                {
                    continue;
                }

                moves.push((*elf, *elf + direction.delta()));
                continue 'next_elf;
            }

            // If we make it this far, add a self move to avoid collisions with elves that can't move
            moves.push((*elf, *elf));
        }

        // Second, remove any duplicates
        let dedup_moves = moves
            .iter()
            .filter(|(p1, p2)| !moves.iter().any(|(q1, q2)| p1 != q1 && p2 == q2))
            .collect::<Vec<_>>();

        self.locations.iter_mut().for_each(|(_, v)| *v += 1);

        // Perform the moves
        let mut changed = false;
        for (src, dst) in dedup_moves.iter() {
            if src != dst {
                self.locations.remove(src);
                self.locations.insert(*dst, 1);
                changed = true;
            }
        }

        changed
    }

    fn render(&self, bounds: Option<[isize; 4]>) -> RgbImage {
        let [min_x, max_x, min_y, max_y] = if bounds.is_some() {
            bounds.unwrap()
        } else {
            self.bounds()
        };

        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;

        ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
            let p = Point::new(x as isize + min_x, y as isize + min_y);

            if let Some(age) = self.locations.get(&p) {
                if *age > 255 {
                    image::Rgb([0, 127, 127])
                } else if *age > 128 {
                    image::Rgb([0, 127, (*age - 128) as u8])
                } else {
                    image::Rgb([0, (255 - *age) as u8, 0])
                }
            } else if p.x == 0 || p.y == 0 {
                image::Rgb([63, 63, 63])
            } else {
                image::Rgb([0, 0, 0])
            }
        })
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn proposal(round: usize, check: usize) -> Self {
        match (round + check) % 4 {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::East,
            _ => panic!("something weird happened"),
        }
    }

    fn delta(self) -> Point {
        match self {
            Direction::North => Point::new(0, -1),
            Direction::South => Point::new(0, 1),
            Direction::West => Point::new(-1, 0),
            Direction::East => Point::new(1, 0),
        }
    }

    fn check(self) -> [Point; 3] {
        match self {
            Direction::North => [Point::new(-1, -1), Point::new(0, -1), Point::new(1, -1)],
            Direction::South => [Point::new(-1, 1), Point::new(0, 1), Point::new(1, 1)],
            Direction::West => [Point::new(-1, -1), Point::new(-1, 0), Point::new(-1, 1)],
            Direction::East => [Point::new(1, -1), Point::new(1, 0), Point::new(1, 1)],
        }
    }
}

fn part1(filename: &Path) -> String {
    let mut elves = Elves::from(filename);

    if cfg!(debug_assertions) {
        println!("== Initial State ==\n{elves}");
    }

    for frame in 0..10 {
        elves.step(frame);

        if cfg!(debug_assertions) {
            println!("== End of Round {} ==\n{elves}", frame + 1);
        }
    }

    let [min_x, max_x, min_y, max_y] = elves.bounds();

    (((max_x - min_x + 1) * (max_y - min_y + 1)) as usize - elves.locations.len()).to_string()
}

fn part2(filename: &Path) -> String {
    let mut elves = Elves::from(filename);

    if cfg!(debug_assertions) {
        println!("== Initial State ==\n{elves}");
    }

    let mut final_frame = 0;
    for frame in 0.. {
        let changed = elves.step(frame);
        if !changed {
            final_frame = frame + 1;
            // break;
        }
        if frame == 1200 {
            break;
        }

        if cfg!(debug_assertions) {
            println!("== End of Round {} ==\n{elves}", frame + 1);
        }

        if env::var("AOC23_RENDER").is_ok() {
            println!("Rendering {frame}");
            elves
                .render(Some([-13, 127, -13, 125]))
                .save(format!("{:08}.png", frame))
                .expect("failed to save frame");
        }
    }

    if env::var("AOC23_RENDER").is_ok() {
        println!("Rendering mp4");

        use std::process::Command;

        let commands = vec![
            "ffmpeg -y -framerate 240 -i %08d.png -vf scale=iw*4:ih*4:flags=neighbor -c:v libx264 -r 30 aoc23.raw.mp4",
            "find . -name '*.png' | xargs rm",
            "ffmpeg -y -i aoc23.raw.mp4 -c:v libx264 -preset slow -crf 20 -vf format=yuv420p -movflags +faststart aoc23.mp4",
            "rm aoc23.raw.mp4",
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

    final_frame.to_string()
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
        aoc_test("23", part1, "4241")
    }

    #[test]
    fn test2() {
        aoc_test("23", part2, "1079")
    }
}
