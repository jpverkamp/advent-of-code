use anyhow::Result;
use std::io::{self, BufRead};

const DIGIT_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

// #[aoc_test("data/test/01.txt", 142)]
// #[aoc_test("data/test/01b.txt", 281)]
// #[aoc_test("data/01.txt", 53894)]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let result = lines
        .filter_map(|l| {
            let l = l.ok()?;
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
        .to_string();

    println!("{result}");
    Ok(())
}
