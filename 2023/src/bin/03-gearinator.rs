use anyhow::Result;
use aoc::*;
use std::{fs::read_to_string, path::Path};

#[derive(Debug)]
struct Number {
    value: usize,
    x_min: usize,
    x_max: usize,
    y: usize,
}

impl Number {
    fn is_neighbor(&self, x: usize, y: usize) -> bool {
        x + 1 >= self.x_min && x <= self.x_max && y + 1 >= self.y && y <= self.y + 1
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl From<String> for Schematic {
    fn from(value: String) -> Self {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();

        fn finish_number(
            numbers: &mut Vec<Number>,
            digits: &mut String,
            x_min: usize,
            x_max: usize,
            y: usize,
        ) {
            if digits.is_empty() {
                return;
            }

            let value = digits.parse::<usize>().unwrap();
            digits.clear();
            numbers.push(Number {
                value,
                x_min,
                x_max,
                y,
            });
        }

        for (y, line) in value.lines().enumerate() {
            let mut digits = String::new();
            let mut x_min = 0;

            for (x, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    if digits.is_empty() {
                        x_min = x;
                    }
                    digits.push(c);
                } else if c == '.' {
                    finish_number(&mut numbers, &mut digits, x_min, x, y);
                } else {
                    finish_number(&mut numbers, &mut digits, x_min, x, y);
                    symbols.push(Symbol { value: c, x, y });
                }
            }
            finish_number(&mut numbers, &mut digits, x_min, line.len(), y);
        }

        Schematic { numbers, symbols }
    }
}

fn part1(filename: &Path) -> Result<String> {
    let schematic = Schematic::from(read_to_string(filename)?);

    Ok(schematic
        .numbers
        .iter()
        .filter(|n| schematic.symbols.iter().any(|s| n.is_neighbor(s.x, s.y)))
        .map(|n| n.value)
        .sum::<usize>()
        .to_string())
}

fn part2(filename: &Path) -> Result<String> {
    let schematic = Schematic::from(read_to_string(filename)?);

    Ok(schematic
        .symbols
        .iter()
        .filter(|s| s.value == '*')
        .map(|s| {
            schematic
                .numbers
                .iter()
                .filter_map(|n| {
                    if n.is_neighbor(s.x, s.y) {
                        Some(n.value)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .filter(|ratios| ratios.len() == 2)
        .map(|ratios| ratios[0] * ratios[1])
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
    fn test_parse_numer() {
        let schematic = super::Schematic::from(String::from("1..2\n3..4"));
        assert_eq!(schematic.numbers.len(), 4);
        assert_eq!(schematic.numbers[0].value, 1);
        assert_eq!(schematic.numbers[0].x_min, 0);
        assert_eq!(schematic.numbers[0].x_max, 1);
        assert_eq!(schematic.numbers[0].y, 0);
        assert_eq!(schematic.numbers[1].value, 2);
        assert_eq!(schematic.numbers[1].x_min, 3);
        assert_eq!(schematic.numbers[1].x_max, 4);
        assert_eq!(schematic.numbers[1].y, 0);
    }

    #[test]
    fn test_parse_longer_number() {
        let schematic = super::Schematic::from(String::from("....\n.123"));
        assert_eq!(schematic.numbers.len(), 1);
        assert_eq!(schematic.numbers[0].value, 123);
        assert_eq!(schematic.numbers[0].x_min, 1);
        assert_eq!(schematic.numbers[0].x_max, 4);
        assert_eq!(schematic.numbers[0].y, 1);
    }

    #[test]
    fn test_symbols() {
        let schematic = super::Schematic::from(String::from("1..2\n3..4"));
        assert_eq!(schematic.symbols.len(), 0);

        let schematic = super::Schematic::from(String::from("1..2\n3..4\n..*."));
        assert_eq!(schematic.symbols.len(), 1);
        assert_eq!(schematic.symbols[0].value, '*');
        assert_eq!(schematic.symbols[0].x, 2);
        assert_eq!(schematic.symbols[0].y, 2);
    }

    #[test]
    fn test_neighbor() {
        let number = super::Number {
            value: 1,
            x_min: 0,
            x_max: 1,
            y: 0,
        };

        assert!(number.is_neighbor(0, 0));
        assert!(number.is_neighbor(1, 0));
        assert!(number.is_neighbor(0, 1));
        assert!(number.is_neighbor(1, 1));
        assert!(!number.is_neighbor(2, 0));
        assert!(!number.is_neighbor(0, 2));
    }

    #[test]
    fn test1() {
        aoc_test("test/03", part1, "4361");
        aoc_test("03", part1, "549908");
    }

    #[test]
    fn test2() {
        aoc_test("test/03", part2, "467835");
        aoc_test("03", part2, "81166799");
    }
}
