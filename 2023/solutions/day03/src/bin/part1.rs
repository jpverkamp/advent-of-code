use anyhow::Result;
use std::io;

use day03::types::*;

aoc_test::generate!{day03_part1_test_03 as "test/03.txt" => "4361"}
aoc_test::generate!{day03_part1_03 as "03.txt" => "549908"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let schematic = Schematic::from(input);

    Ok(schematic
        .numbers
        .iter()
        .filter(|n| schematic.symbols.iter().any(|s| n.is_neighbor(s.x, s.y)))
        .map(|n| n.value)
        .sum::<usize>()
        .to_string())
}
