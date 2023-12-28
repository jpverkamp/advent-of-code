use anyhow::Result;
use std::io;

use day02::parse;

aoc_test::generate!{day02_part1_test_02 as "test/02.txt" => "8"}
aoc_test::generate!{day02_part1_02 as "02.txt" => "2061"}

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
        .filter(|game| {
            game.rounds
                .iter()
                .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
        })
        .map(|game| game.id)
        .sum::<u32>()
        .to_string())
}
