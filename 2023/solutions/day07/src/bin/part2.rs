use anyhow::Result;
use std::io;

use day07::parse;

// #[aoc_test("data/test/07.txt", "5905")]
// #[aoc_test("data/07.txt", "253907829")]
fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let input = input.replace('J', "*");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, mut hands) = parse::hands(input).unwrap();
    assert_eq!(s.trim(), "");

    hands.sort();

    Ok(hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid as usize)
        .sum::<usize>()
        .to_string())
}
