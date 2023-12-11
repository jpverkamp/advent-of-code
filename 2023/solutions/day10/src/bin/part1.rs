use anyhow::Result;
use std::io;

use day10::types::*;

// #[aoc_test("data/test/10.txt", "4")]
// #[aoc_test("data/test/10b.txt", "4")]
// #[aoc_test("data/test/10c.txt", "8")]
// #[aoc_test("data/10.txt", "6956")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let map = Map::from(input.as_str());

    // Set off two iters, one at double speed
    // Skip the first nyde for each to avoid the start node
    // When they are equal, they have reached the farthest point
    let mut result = map
        .iter()
        .cycle()
        .skip(1)
        .zip(map.iter().cycle().skip(2).step_by(2))
        .position(|(n1, n2)| n1 == n2)
        .unwrap();

    result = (result + 1) / 2;

    println!("{result}");
    Ok(())
}
