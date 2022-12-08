use aoc::*;
use std::{iter::Sum, path::Path};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Elf {
    calories: i32,
}

impl Elf {
    fn new() -> Self {
        Elf { calories: 0 }
    }
}

impl Sum for Elf {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut calories = 0;
        for elf in iter {
            calories += elf.calories;
        }
        Elf { calories }
    }
}

fn read(filename: &Path) -> Vec<Elf> {
    let mut elves = Vec::new();
    let mut current = Elf::new();

    for line in read_lines(filename) {
        if line.len() == 0 {
            elves.push(current);
            current = Elf::new();
        } else {
            current.calories += line.parse::<i32>().unwrap();
        }
    }
    elves.push(current);

    return elves;
}

fn part1(filename: &Path) -> String {
    let elves = read(filename);
    elves
        .iter()
        .max()
        .expect("no Elves found, can't take max")
        .calories
        .to_string()
}

fn part2(filename: &Path) -> String {
    let mut elves = read(filename);

    elves.sort();
    elves.reverse();

    elves.into_iter().take(3).sum::<Elf>().calories.to_string()
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
        aoc_test("01", part1, "70369")
    }

    #[test]
    fn test2() {
        aoc_test("01", part2, "203002")
    }
}
