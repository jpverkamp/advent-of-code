use anyhow::Result;
use aoc::*;
use std::path::Path;

fn part1(filename: &Path) -> Result<String> {
    Ok(iter_lines(filename)
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

fn part2(filename: &Path) -> Result<String> {
    let digit_words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    Ok(iter_lines(filename)
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
                for (digit, word) in digit_words.iter().enumerate() {
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

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc::aoc_test;

    #[test]
    fn test1() {
        aoc_test("test/01-1", part1, "142");
        aoc_test("test/01-2", part1, "209");
        aoc_test("01", part1, "53651");
    }

    #[test]
    fn test2() {
        aoc_test("test/01-1", part2, "142");
        aoc_test("test/01-2", part2, "281");
        aoc_test("01", part2, "53894");
    }
}
