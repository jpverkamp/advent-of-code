use aoc::*;
use std::path::Path;

fn part1(filename: &Path) -> String {
    iter_lines(filename)
        .map(|l| {
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

            (10 * (first.unwrap() as usize - '0' as usize))
                + (last.unwrap() as usize - '0' as usize)
        })
        .sum::<usize>()
        .to_string()
}

fn part2(filename: &Path) -> String {
    let digit_words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    iter_lines(filename)
        .map(|l| {
            let mut first = None;
            let mut last = None;

            for (i, c) in l.chars().enumerate() {
                // Match literal digits
                if c.is_numeric() {
                    let c = c.to_digit(10).unwrap() as usize;
                    if first.is_none() {
                        first = Some(c);
                    }
                    last = Some(c);
                    continue;
                }

                // Match digit words
                for digit in 0..digit_words.len() {
                    if l[i..].starts_with(digit_words[digit]) {
                        if first.is_none() {
                            first = Some(digit);
                        }
                        last = Some(digit);
                        break;
                    }
                }
            }

            10 * first.unwrap() + last.unwrap()
        })
        .sum::<usize>()
        .to_string()
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
        aoc_test("01", part1, "53651")
    }

    #[test]
    fn test2() {
        aoc_test("01", part2, "53894")
    }
}
