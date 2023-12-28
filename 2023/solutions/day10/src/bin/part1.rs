use anyhow::Result;
use std::io;

use day10::types::*;

aoc_test::generate!{day10_part1_test_10 as "test/10.txt" => "4"}
aoc_test::generate!{day10_part1_test_10b as "test/10b.txt" => "4"}
aoc_test::generate!{day10_part1_test_10c as "test/10c.txt" => "8"}
aoc_test::generate!{day10_part1_10 as "10.txt" => "6956"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let map = Map::from(input);

    // Original:
    // Set off two iters, one at double speed
    // Skip the first node for each to avoid the start node
    // When they are equal, they have reached the farthest point

    // New version:
    // Half the loop is the farthest point...
    let result = map.iter().count() / 2;
    Ok(result.to_string())
}
