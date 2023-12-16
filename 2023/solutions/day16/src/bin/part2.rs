use anyhow::Result;
use std::io;

use day16::types::*;

use grid::Grid;
use point::Point;

mod part1;
use part1::illuminate;

// #[aoc_test("data/test/16.txt", "")]
// #[aoc_test("data/16.txt", "")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    use Direction::*;

    let input = io::read_to_string(stdin.lock())?;
    let mirrors = Grid::read(&input, |c| match c {
        '|' => Some(Mirror::VerticalSplitter),
        '-' => Some(Mirror::HorizontalSplitter),
        '/' => Some(Mirror::ForwardReflector),
        '\\' => Some(Mirror::BackwardReflector),
        _ => None,
    });

    let mut starts = Vec::new();
    for x in mirrors.bounds.min_x..=mirrors.bounds.max_x {
        starts.push((Point::new(x, mirrors.bounds.min_y), South));
        starts.push((Point::new(x, mirrors.bounds.max_y), North));
    }
    for y in mirrors.bounds.min_y..=mirrors.bounds.max_y {
        starts.push((Point::new(mirrors.bounds.min_x, y), East));
        starts.push((Point::new(mirrors.bounds.max_x, y), West));
    }

    let result = starts
        .iter()
        .map(|start| illuminate(&mirrors, *start).iter().count())
        .max()
        .unwrap();

    println!("{result}");
    Ok(())
}
