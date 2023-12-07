use anyhow::Result;
use std::io;

use day06::parse;

// #[aoc_test("data/test/06.txt", "288")]
// #[aoc_test("data/06.txt", "741000")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let (s, races) = parse::races(&input).unwrap();
    assert_eq!(s.trim(), "");

    let result = races.iter().map(|r| r.record_breakers()).product::<u64>();

    println!("{result}");
    Ok(())
}
