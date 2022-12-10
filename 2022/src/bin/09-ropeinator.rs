use aoc::*;
use im::HashSet;
use std::path::Path;

trait Followable {
    fn follow(self, other: Self) -> Self;
}

impl Followable for Point {
    fn follow(self, other: Point) -> Self {
        if self == other || self.adjacent_to(&other) {
            self
        } else {
            let xd = (other.x - self.x).signum();
            let yd = (other.y - self.y).signum();

            self + Point { x: xd, y: yd }
        }
    }
}

/* ----- Implement orthogonal directions ----- */
#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("unknown direction char"),
        }
    }
}

impl Direction {
    fn delta(self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

/* ----- Store a single instruction: direction + distance ----- */
#[derive(Copy, Clone, Debug)]
struct Instruction {
    direction: Direction,
    distance: usize,
}

impl From<String> for Instruction {
    fn from(line: String) -> Self {
        let mut parts = line.split_ascii_whitespace();

        let direction = Direction::from(
            parts
                .next()
                .expect("must have a direction part")
                .chars()
                .nth(0)
                .expect("must have a "),
        );

        let distance = parts
            .next()
            .expect("must have a number part")
            .parse::<usize>()
            .expect("number must be a number");

        Instruction {
            direction,
            distance,
        }
    }
}

/* ----- Implement a one link rope ----- */
#[derive(Clone, Debug)]
struct Rope {
    head: Point,
    tail: Point,
    visited: HashSet<Point>,
}

impl Rope {
    // Initialize a new rope coiled at the origin
    fn new() -> Self {
        Rope {
            head: Point::ORIGIN,
            tail: Point::ORIGIN,
            visited: HashSet::unit(Point::ORIGIN),
        }
    }

    // Step n times in one direction
    fn step_by(self, instruction: Instruction) -> Self {
        let mut current = self;

        for _ in 0..instruction.distance {
            let new_head = current.head + instruction.direction.delta();
            let new_tail = current.tail.follow(new_head);

            let mut new_visited = current.visited;
            new_visited.insert(new_tail);

            current = Rope {
                head: new_head,
                tail: new_tail,
                visited: new_visited,
            };
        }

        current
    }
}

/* ----- For part 2, generalize to chains of any length ----- */
#[derive(Clone, Debug)]
struct Chain {
    points: Vec<Point>,
    tail_visited: HashSet<Point>,
}

impl Chain {
    fn new(size: usize) -> Self {
        Chain {
            points: vec![Point::ORIGIN; size],
            tail_visited: HashSet::unit(Point::ORIGIN),
        }
    }

    fn step_by(self, instruction: Instruction) -> Self {
        let mut current = self.clone();

        for _ in 0..instruction.distance {
            let mut new_points = Vec::new();
            let mut previous = current.points[0] + instruction.direction.delta();

            new_points.push(previous);

            for point in current.points.iter().skip(1) {
                let next = point.follow(previous);
                new_points.push(next);
                previous = next;
            }

            let mut new_tail_visited = current.tail_visited;
            new_tail_visited.insert(previous);

            current = Chain {
                points: new_points,
                tail_visited: new_tail_visited,
            }
        }

        current
    }
}

fn part1(filename: &Path) -> String {
    let mut rope = Rope::new();

    for line in iter_lines(filename) {
        let instruction = Instruction::from(line);
        rope = rope.step_by(instruction);
    }

    if cfg!(debug_assertions) {
        let mut chain = Chain::new(2);

        for line in iter_lines(filename) {
            let instruction = Instruction::from(line);
            chain = chain.step_by(instruction);
        }

        println!("using a chain(2): {}", chain.tail_visited.len());
    }

    rope.visited.len().to_string()


}

fn part2(filename: &Path) -> String {
    let mut chain = Chain::new(10);

    for line in iter_lines(filename) {
        let instruction = Instruction::from(line);
        chain = chain.step_by(instruction);
    }

    chain.tail_visited.len().to_string()
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
        aoc_test("09", part1, "6339")
    }

    #[test]
    fn test2() {
        aoc_test("09", part2, "2541")
    }
}
