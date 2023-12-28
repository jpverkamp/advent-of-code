use anyhow::Result;
use std::io;

use day07::parse;

aoc_test::generate!{day07_part2_test_07 as "test/07.txt" => "5905"}
aoc_test::generate!{day07_part2_07 as "07.txt" => "253907829"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let input = input.replace('J', "*");
    let (s, mut hands) = parse::hands(input.as_str()).unwrap();
    assert_eq!(s.trim(), "");

    hands.sort();

    Ok(hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid as usize)
        .sum::<usize>()
        .to_string())
}
