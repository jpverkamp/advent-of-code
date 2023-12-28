use anyhow::Result;
use std::io;

use day07::parse;

aoc_test::generate!{day07_part1_test_07 as "test/07.txt" => "6440"}
aoc_test::generate!{day07_part1_07 as "07.txt" => "253205868"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
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
