use aoc::*;
use std::{
    collections::HashSet,
    path::Path,
};

#[derive(Debug)]
struct Rock {
    points: Vec<Point>,
}

impl Rock {
    fn nth(n: usize) -> Rock {
        match n % 5 {
            0 => Rock {
                points: vec![
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: 2, y: 0 },
                    Point { x: 3, y: 0 },
                ],
            },
            1 => Rock {
                points: vec![
                    Point { x: 1, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: 1, y: 1 },
                    Point { x: 2, y: 1 },
                    Point { x: 1, y: 2 },
                ],
            },
            2 => Rock {
                points: vec![
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: 2, y: 0 },
                    Point { x: 2, y: 1 },
                    Point { x: 2, y: 2 },
                ],
            },
            3 => Rock {
                points: vec![
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: 0, y: 2 },
                    Point { x: 0, y: 3 },
                ],
            },
            4 => Rock {
                points: vec![
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: 1, y: 1 },
                ],
            },
            _ => panic!("n % 5 somehow not 0..=4"),
        }
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    tower_height: usize,

    walls: HashSet<Point>,

    rock_count: usize,
    rock: Rock,
    rock_at: Point,
}

impl Map {
    fn new(width: usize) -> Self {
        let mut walls = HashSet::new();
        for x in 0..=(width + 2) {
            walls.insert(Point {
                x: x as isize,
                y: 0,
            });
        }
        for y in 0..=4 {
            walls.insert(Point {
                x: 0,
                y: y as isize,
            });
            walls.insert(Point {
                x: 1 + width as isize,
                y: y as isize,
            });
        }

        Map {
            width,
            tower_height: 0,
            walls,
            rock_count: 0,
            rock: Rock::nth(0),
            rock_at: Point {
                x: 3,
                y: 4 as isize,
            },
        }
    }

    fn lock_and_spawn(&mut self) {
        for p in self.rock.points.iter() {
            let p = *p + self.rock_at;
            self.tower_height = self.tower_height.max(p.y as usize);
            self.walls.insert(p);
        }

        // Inefficient, but :shrug:
        for y in 0..=(self.tower_height + 10) {
            self.walls.insert(Point {
                x: 0,
                y: y as isize,
            });
            self.walls.insert(Point {
                x: 1 + self.width as isize,
                y: y as isize,
            });
        }

        // Don't forget the extra offset for the left wall and floor
        self.rock = Rock::nth(self.rock_count);
        self.rock_at = Point {
            x: 1 + 2,
            y: 1 + 3 + self.tower_height as isize,
        };

        self.rock_count += 1;
    }

    fn step(&mut self, xd: isize) {
        // Try to move left/right
        // If we can't, just don't move
        let sidestep = Point { x: xd, y: 0 };
        if !self
            .rock
            .points
            .iter()
            .any(|p| self.walls.contains(&(*p + self.rock_at + sidestep)))
        {
            self.rock_at = self.rock_at + sidestep;
        }

        // Try to move down
        // If we can't, lock in place
        let downstep = Point { x: 0, y: -1 };
        if self
            .rock
            .points
            .iter()
            .any(|p| self.walls.contains(&(*p + self.rock_at + downstep)))
        {
            self.lock_and_spawn();
        } else {
            self.rock_at = self.rock_at + downstep;
        }
    }

    // A currently unneeded function used to remove all walls lower than a certain threshold
    // We'll never collide with them anyways
    // Unfortunately, the drain_filter function that would have made this more performant isn't in stable
    #[allow(dead_code)]
    fn cleanup(&mut self, threshold: usize) {
        self.walls = self
            .walls
            .iter()
            .filter_map(|p| {
                if p.y >= (self.tower_height - threshold) as isize {
                    Some(*p)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for y in (0..=(self.tower_height + 10)).rev() {
            for x in 0..=(self.width + 1) {
                let p = Point {
                    x: x as isize,
                    y: y as isize,
                };
                if self.walls.contains(&p) {
                    buffer.push('#');
                } else if self.rock.points.iter().any(|rp| p == *rp + self.rock_at) {
                    buffer.push('@');
                } else {
                    buffer.push('.');
                }
            }
            buffer.push('\n');
        }
        write!(f, "{buffer}")
    }
}

fn part1(filename: &Path) -> String {
    let lines = read_lines(filename);
    let winds = lines[0].as_bytes();

    // Create a custom infinite iterator for the wind
    let mut first = true;
    let mut index = 0;
    let mut wind_iter = std::iter::from_fn(move || {
        if first {
            first = false;
        } else {
            index += 1;
        }
        Some(winds[index % winds.len()] as char)
    });

    // Build a new map and iterate until we hit the target
    let mut map = Map::new(7);
    loop {
        match wind_iter.next() {
            Some('<') => {
                map.step(-1);
            }
            Some('>') => {
                map.step(1);
            }
            _ => panic!("unexpected char in wind_iter"),
        }

        if map.rock_count >= 2022 {
            break;
        }
    }

    map.tower_height.to_string()
}

fn part2(filename: &Path) -> String {
    let lines = read_lines(filename);
    let winds = lines[0].as_bytes();

    let target: usize = 1000000000000;

    let mut first = true;
    let mut index = 0;
    let mut wind_iter = std::iter::from_fn(move || {
        if first {
            first = false;
        } else {
            index += 1;
        }
        Some(winds[index % winds.len()] as char)
    });

    let mut map = Map::new(7);
    let mut last_rock_count = usize::MAX;
    let mut last_height = 0;
    let mut deltas = Vec::new();

    // Attempt to detect cycles in the delta between heights
    #[derive(Debug)]
    struct Cycle {
        length: usize,
        value: usize,
    }
    let mut cycle = None;

    // Loop until we find a cycle
    // Then loop until the current height and the target are at the same point in the cycle
    // Then add enough cycles to jump ahead to the end
    'cycle: loop {
        match wind_iter.next() {
            Some('<') => {
                map.step(-1);
            }
            Some('>') => {
                map.step(1);
            }
            _ => panic!("unexpected char in wind_iter"),
        }

        // Update the count and height as before, but also calculate delta (change in height)
        if map.rock_count != last_rock_count {
            let count = map.rock_count;
            let height = map.tower_height;
            let delta = height - last_height;
            deltas.push(delta);

            if cfg!(debug_assertions) {
                println!("{count}\t{delta}\t{height}");
            }

            last_rock_count = count;
            last_height = height;

            // Try to detect cycles in delta by:
            // - for each length from a small value up to the full list
            // - test if [prefix][data of length][data of length] repeats the data sections
            // Once we've detected a cycle, stop looking for one (but keep iterating)
            // The offsets are completely random at this point, I'm not sure what to base it off
            if cycle.is_none() && deltas.len() > 2000 {
                for length in 1000..(deltas.len() / 2) {
                    let seq1 = deltas.iter().rev().take(length).collect::<Vec<_>>();
                    let seq2 = deltas
                        .iter()
                        .rev()
                        .skip(length)
                        .take(length)
                        .collect::<Vec<_>>();

                    if seq1 == seq2 {
                        cycle = Some(Cycle {
                            length: length,
                            value: seq1.into_iter().sum::<usize>(),
                        });

                        if cfg!(debug_assertions) {
                            println!("cycle detected: {cycle:?}");
                        }
                    }
                }
            }

            // If we have a cycle, we need the current rock and the target to be at the same offset
            // Otherwise we'd have to add partial cycles; we can do it, this is just easier
            match cycle {
                Some(Cycle { length, value, .. }) => {
                    if map.rock_count % length == target % length {
                        // Hacky, but technically correct? Doesn't update walls
                        // We certainly could, but no point for this problem
                        let jumps = (target - map.rock_count) / length;
                        map.rock_count += jumps * length;
                        map.tower_height += jumps * value;
                        break 'cycle;
                    }
                }
                _ => {}
            }
        }

        // Edge case: If we never found a cycle but hit the target anyways, be done
        if map.rock_count >= target {
            break 'cycle;
        }
    }

    map.tower_height.to_string()
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
        aoc_test("17", part1, "3112")
    }

    #[test]
    fn test2() {
        aoc_test("17", part2, "1540804597681")
    }
}
