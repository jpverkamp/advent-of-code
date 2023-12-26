use anyhow::Result;
use itertools::Itertools;
use std::io;

use day18::{parse, types::*};

use point::Point;

// #[aoc_test("data/test/18.txt", "952408144115")]
// #[aoc_test("data/18.txt", "67622758357096")]
// 67622694397113 is too low
fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, mut commands) = parse::commands(input).unwrap();
    assert_eq!(s.trim(), "");

    commands.iter_mut().for_each(|c| {
        let s = c.color.to_hex();
        c.steps = u64::from_str_radix(&s[1..6], 16).unwrap();
        c.direction = match s.chars().last().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("Invalid direction"),
        }
    });

    // Find the vertexes, make sure we're 'closed' by including the origin at both ends
    let mut vertexes = vec![];
    vertexes.push(Point::ORIGIN);

    let mut current = Point::ORIGIN;
    commands.iter().for_each(|c| {
        current = current + (c.steps as isize) * Point::from(c.direction);
        vertexes.push(current);
    });
    vertexes.push(Point::ORIGIN);

    // https://www.mathopenref.com/coordpolygonarea.html
    let mut result = vertexes
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.x * b.y - a.y * b.x)
        .sum::<isize>()
        / 2;

    // Since we want the border, add half of them (all left and up, it's arbitrary)
    result += commands
        .iter()
        .map(|c| {
            if c.direction == Direction::Left || c.direction == Direction::Up {
                c.steps as isize
            } else {
                0
            }
        })
        .sum::<isize>();

    // Final result is always off by 1 for reasons?
    result += 1;

    Ok(result.to_string())
}
