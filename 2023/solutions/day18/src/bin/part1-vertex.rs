use anyhow::Result;
use itertools::Itertools;
use std::io;

use day18::{parse, types::*};

use point::Point;

// #[aoc_test("data/test/18.txt", "62")]
// #[aoc_test("data/18.txt", "52055")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let (s, commands) = parse::commands(&input).unwrap();
    assert_eq!(s.trim(), "");

    let mut vertexes = vec![];
    vertexes.push(Point::ORIGIN);

    let mut current = Point::ORIGIN;
    commands.iter().for_each(|c| {
        current = current + Point::from(c.direction) * c.steps as isize;
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

    println!("{result}");
    Ok(())
}
