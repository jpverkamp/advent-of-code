use aoc::*;
use std::path::Path;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn new(c: char) -> Play {
        use Play::*;

        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!("unknown play: {:?}", c),
        }
    }

    fn value(self) -> i32 {
        use Play::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn vs(self, other: Play) -> Outcome {
        use Outcome::*;
        use Play::*;

        match (self, other) {
            (a, b) if a == b => Draw,

            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Win,

            _ => Lose,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn new(c: char) -> Outcome {
        use Outcome::*;

        match c {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("unknown outcome: {:?}", c),
        }
    }

    fn value(self) -> i32 {
        use Outcome::*;

        match self {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

fn part1(filename: &Path) -> String {
    let mut total_score = 0;

    for line in read_lines(filename) {
        let them = Play::new(line.chars().nth(0).expect("must have 1 char per line"));
        let us = Play::new(line.chars().nth(2).expect("must have 3 chars per line"));

        total_score += us.value() + us.vs(them).value();
    }

    total_score.to_string()
}

fn part2(filename: &Path) -> String {
    use Outcome::*;
    use Play::*;

    let mut total_score = 0;

    for line in read_lines(filename) {
        let them = Play::new(line.chars().nth(0).expect("must have 1 char per line"));
        let goal = Outcome::new(line.chars().nth(2).expect("must have 3 chars per line"));

        let us = match goal {
            Lose => match them {
                Rock => Scissors,
                Scissors => Paper,
                Paper => Rock,
            },
            Draw => them,
            Win => match them {
                Rock => Paper,
                Scissors => Rock,
                Paper => Scissors,
            },
        };

        total_score += us.value() + goal.value();
    }

    total_score.to_string()
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
        aoc_test("02", part1, "13446")
    }

    #[test]
    fn test2() {
        aoc_test("02", part2, "13509")
    }
}
