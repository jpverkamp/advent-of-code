use anyhow::Result;
use std::io;

aoc_test::generate!{day15_part1_test_15 as "test/15.txt" => "1320"}
aoc_test::generate!{day15_part1_15 as "15.txt" => "508552"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    fn hash(s: &str) -> u8 {
        s.chars()
            .fold(0, |v, c| ((v.wrapping_add(c as u8)).wrapping_mul(17)))
    }

    Ok(input
        .split(',')
        .map(hash)
        .map(|v| v as usize)
        .sum::<usize>()
        .to_string())
}
