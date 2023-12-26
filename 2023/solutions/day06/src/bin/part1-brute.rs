use anyhow::Result;
use std::io;

use day06::parse;

// #[aoc_test("data/test/06.txt", "288")]
// #[aoc_test("data/06.txt", "741000")]
fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, races) = parse::races(input).unwrap();
    assert_eq!(s.trim(), "");

    Ok(races
        .iter()
        .map(|r| r.record_breakers_bf())
        .product::<u64>()
        .to_string())
}
