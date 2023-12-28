use anyhow::Result;
use std::io;

aoc_test::generate!{day01_part1_test_01 as "test/01.txt" => "142"}
aoc_test::generate!{day01_part1_test_01b as "test/01b.txt" => "209"}
aoc_test::generate!{day01_part1_01 as "01.txt" => "53651"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .filter_map(|l| {
            let mut first = None;
            let mut last = None;

            for c in l.chars() {
                if c.is_numeric() {
                    if first.is_none() {
                        first = Some(c);
                    }
                    last = Some(c);
                }
            }

            Some(10 * first?.to_digit(10)? + last?.to_digit(10)?)
        })
        .sum::<u32>()
        .to_string())
}
