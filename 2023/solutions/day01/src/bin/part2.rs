use anyhow::Result;
use std::io;

const DIGIT_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

aoc_test::generate!{day01_part2_test_01 as "test/01.txt" => "142"}
aoc_test::generate!{day01_part2_test_01b as "test/01b.txt" => "281"}
aoc_test::generate!{day01_part2_01 as "01.txt" => "53894"}

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

            for (i, c) in l.chars().enumerate() {
                // Match literal digits
                if c.is_numeric() {
                    let c = c.to_digit(10)? as usize;
                    if first.is_none() {
                        first = Some(c);
                    }
                    last = Some(c);
                    continue;
                }

                // Match digit words
                for (digit, word) in DIGIT_WORDS.iter().enumerate() {
                    if l[i..].starts_with(word) {
                        if first.is_none() {
                            first = Some(digit);
                        }
                        last = Some(digit);
                        break;
                    }
                }
            }

            Some(10 * first? + last?)
        })
        .sum::<usize>()
        .to_string())
}
