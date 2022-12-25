use aoc::*;
use std::{fmt::Display, path::Path};

#[derive(Clone, Debug)]
struct Snafu {
    value: String,
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<String> for Snafu {
    fn from(value: String) -> Self {
        Snafu { value }
    }
}

impl From<isize> for Snafu {
    fn from(mut v: isize) -> Self {
        // Convert to base 5
        let mut digits = Vec::new();

        while v > 0 {
            let m = v % 5;
            v = v / 5;

            if m < 3 {
                digits.push(m.to_string());
            } else if m == 3 {
                digits.push(String::from("="));
                v += 1;
            } else if m == 4 {
                digits.push(String::from("-"));
                v += 1;
            }
        }

        Snafu {
            value: digits.into_iter().rev().collect::<Vec<_>>().join(""),
        }
    }
}

impl Into<isize> for Snafu {
    fn into(self) -> isize {
        self.value.chars().fold(0, |a, c| match c {
            '2' | '1' | '0' => a * 5 + c.to_digit(10).unwrap() as isize,
            '-' => a * 5 - 1,
            '=' => a * 5 - 2,
            _ => panic!("Snafu SNAFUed, what the Snafu is a {c}"),
        })
    }
}

fn part1(filename: &Path) -> String {
    Snafu::from(
        iter_lines(filename)
            .map(Snafu::from)
            .map::<isize, _>(Snafu::into)
            .sum::<isize>(),
    )
    .to_string()
}

fn part2(_filename: &Path) -> String {
    String::from("Start The Blender")
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
        aoc_test("25", part1, "2-10==12-122-=1-1-22")
    }

    // too low: 35023647158862

    #[test]
    fn test2() {
        aoc_test("25", part2, "Start The Blender")
    }
}
