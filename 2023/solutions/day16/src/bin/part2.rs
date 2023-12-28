use anyhow::Result;
use std::io;

use day16::types::*;

use grid::Grid;
use point::Point;

mod part1;
use part1::illuminate;

aoc_test::generate!{day16_part2_test_16 as "test/16.txt" => "51"}
aoc_test::generate!{day16_part2_16 as "16.txt" => "7488"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    use Direction::*;

    let mirrors = Grid::read(input, |c| match c {
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

    Ok(starts
        .iter()
        .map(|start| illuminate(&mirrors, *start).iter().count())
        .max()
        .unwrap()
        .to_string())
}
