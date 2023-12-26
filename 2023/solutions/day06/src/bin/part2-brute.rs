use anyhow::Result;
use std::io;

use day06::{parse, types::Race};

// #[aoc_test("data/test/06.txt", "71503")]
// #[aoc_test("data/06.txt", "38220708")]
fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, races) = parse::races(input).unwrap();
    assert_eq!(s.trim(), "");

    let race = Race {
        time: races
            .iter()
            .map(|r| r.time.to_string())
            .collect::<String>()
            .parse::<u64>()?,
        record: races
            .iter()
            .map(|r| r.record.to_string())
            .collect::<String>()
            .parse::<u64>()?,
    };

    Ok(race.record_breakers_bf().to_string())
}
