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

    // Original:
    // Set off two iters, one at double speed
    // Skip the first node for each to avoid the start node
    // When they are equal, they have reached the farthest point

    // New version:
    // Half the loop is the farthest point...
    let result = map.iter().count() / 2;

    println!("{result}");
    Ok(())
}
