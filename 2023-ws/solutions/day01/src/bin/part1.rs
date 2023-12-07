use anyhow::Result;
use std::io::{self, BufRead};

// #[aoc_test("data/test/01.txt", 142)]
// #[aoc_test("data/test/01b.txt", 209)]
// #[aoc_test("data/01.txt", 53651)]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let result = lines
        .filter_map(|l| {
            let l = l.ok()?;
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
        .sum::<u32>();

    println!("{result}");
    Ok(())
}
