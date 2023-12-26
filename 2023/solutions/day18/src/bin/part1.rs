use anyhow::Result;
use std::io;

use day18::{parse, types::*};

use grid::Grid;
use point::Point;

// #[aoc_test("data/test/18.txt", "62")]
// #[aoc_test("data/18.txt", "52055")]
fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, commands) = parse::commands(input).unwrap();
    assert_eq!(s.trim(), "");

    let mut hole = Grid::default();
    let mut current = Point::ORIGIN;

    hole.insert(current, true);

    commands.iter().for_each(|c| {
        for _ in 0..c.steps {
            current = current + Point::from(c.direction);
            hole.insert(current, true);
        }
    });

    // Find the first point 'inside' the hole
    let inside = Point::new(
        commands
            .iter()
            .find_map(|c| match c.direction {
                Direction::Left => Some(-1),
                Direction::Right => Some(1),
                _ => None,
            })
            .unwrap(),
        commands
            .iter()
            .find_map(|c| match c.direction {
                Direction::Up => Some(-1),
                Direction::Down => Some(1),
                _ => None,
            })
            .unwrap(),
    );

    hole.flood_fill(inside, true);

    Ok(hole.len().to_string())
}
