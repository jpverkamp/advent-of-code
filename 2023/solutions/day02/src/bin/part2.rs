use anyhow::Result;
use std::io;

use day02::parse;

// #[aoc_test("data/test/00.txt", "8")]
// #[aoc_test("data/00.txt", "2061")]
fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, games) = parse::games(input).unwrap();
    assert_eq!(s.trim(), "");

    Ok(games
        .into_iter()
        .map(|game| game.power())
        .sum::<u32>()
        .to_string())
}
