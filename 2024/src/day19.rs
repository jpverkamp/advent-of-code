use aoc_runner_derive::aoc;
use hashbrown::{HashMap, HashSet};
use regex::Regex;

pub struct Puzzle<'input> {
    pub towels: Vec<&'input str>,
    pub targets: Vec<&'input str>,
}

impl<'input> From<&'input str> for Puzzle<'input> {
    fn from(input: &'input str) -> Self {
        let mut lines = input.lines();

        let towels = lines.next().unwrap().split(", ").collect::<Vec<_>>();
        lines.next();
        let targets = lines.collect::<Vec<_>>();

        Self { towels, targets }
    }
}

// Needed for tests, but not actually used directly (so we use lifetimes)
// But make_tests gets unhappy if we don't have a parse function, so...
#[allow(dead_code)]
fn parse(input: &str) -> &str {
    input
}

// #[aoc(day19, part1, backtracking)]
#[allow(dead_code)]
fn part1_backtracking(input: &str) -> usize {
    let puzzle: Puzzle = input.into();

    fn recur(towels: &[&str], target: &str) -> bool {
        if target.is_empty() {
            return true;
        }

        for towel in towels {
            if let Some(rest) = target.strip_prefix(towel) {
                if recur(towels, rest) {
                    return true;
                }
            }
        }

        false
    }

    puzzle
        .targets
        .iter()
        .filter(|target| recur(&puzzle.towels, target))
        .count()
}

#[aoc(day19, part1, bt_simplified)]
fn part1_bt_simplified(input: &str) -> usize {
    let puzzle: Puzzle = input.into();

    // Remove any towels that can be created by a combination of other towels
    let mut towels = puzzle.towels.clone();
    let mut i = 0;
    while i < towels.len() {
        let mut subtowels = towels.clone();
        subtowels.remove(i);

        if recur(&subtowels, towels[i]) {
            towels.remove(i);
        } else {
            i += 1;
        }
    }

    fn recur(towels: &[&str], target: &str) -> bool {
        if target.is_empty() {
            return true;
        }

        for towel in towels {
            if let Some(rest) = target.strip_prefix(towel) {
                if recur(towels, rest) {
                    return true;
                }
            }
        }

        false
    }

    puzzle
        .targets
        .iter()
        .filter(|target| recur(&towels, target))
        .count()
}

#[aoc(day19, part1, bt_memo)]
fn part1_bt_memo(input: &str) -> usize {
    let puzzle: Puzzle = input.into();
    let mut cache = HashSet::new();

    fn recur<'input>(cache: &mut HashSet<&'input str>, towels: &[&str], target: &'input str) -> bool {
        if target.is_empty() {
            return true;
        }

        if cache.contains(target) {
            return false;
        }

        for towel in towels {
            if let Some(rest) = target.strip_prefix(towel) {
                if recur(cache, towels, rest) {
                    return true;
                }
            }
        }

        cache.insert(target);
        false
    }

    puzzle
        .targets
        .iter()
        .filter(|target| recur(&mut cache, &puzzle.towels, target))
        .count()
}

#[aoc(day19, part1, regex)]
fn part1_regex(input: &str) -> usize {
    let puzzle: Puzzle = input.into();

    let regex = format!("^({})+$", &puzzle.towels.join("|"));
    let regex = Regex::new(regex.as_str()).unwrap();

    puzzle
        .targets
        .iter()
        .filter(|target| regex.is_match(target))
        .count()
}

// #[aoc(day19, part2, backtracking)]
#[allow(dead_code)]
fn part2_backtracking(input: &str) -> usize {
    let puzzle: Puzzle = input.into();

    fn recur(towels: &[&str], target: &str) -> usize {
        if target.is_empty() {
            return 1;
        }

        let mut count = 0;

        for towel in towels {
            if let Some(rest) = target.strip_prefix(towel) {
                count += recur(towels, rest);
            }
        }

        count
    }

    puzzle
        .targets
        .iter()
        .map(|target| recur(&puzzle.towels, target))
        .sum()
}

#[aoc(day19, part2, bt_memo)]
pub fn part2_backtracking_memo(input: &str) -> usize {
    let puzzle: Puzzle = input.into();

    fn recur<'input>(
        cache: &mut HashMap<&'input str, usize>,
        towels: &[&str],
        target: &'input str,
    ) -> usize {
        // Base case: empty tests are makeable exactly 1 way
        if target.is_empty() {
            return 1;
        }

        // If we've already calculated this target, return the cached value
        // Memoization yo
        if let Some(&count) = cache.get(target) {
            return count;
        }

        // Try each towel and recur on the first occurrence of that towel in the target
        let mut count = 0;

        for towel in towels {
            if let Some(rest) = target.strip_prefix(towel) {
                count += recur(cache, towels, rest);
            }
        }

        cache.insert(target, count);
        count
    }

    let mut cache = HashMap::new();

    puzzle
        .targets
        .iter()
        .map(|target| recur(&mut cache, &puzzle.towels, target))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    make_test!([part1_regex, part1_backtracking, part1_bt_simplified] => "day19.txt", 6, 336);
    make_test!([part2_backtracking, part2_backtracking_memo] => "day19.txt", 16, "758890600222015");

    macro_rules! make_part2_sub_tests {
        ([$($func:ident),+ $(,)?], $inputs:tt) => {
            $(
                make_part2_sub_tests_inner!([$func] $inputs);
            )*
        };
    }

    macro_rules! make_part2_sub_tests_inner {
        ([$func:ident] [$($input:expr => $expected:expr),+ $(,)?]) => {
            $(
                paste::paste! {
                #[test]
                fn [<sub_test_ $func _ $input>]() {
                    assert_eq!($func(format!("r, wr, b, g, bwu, rb, gb, br\n\n{}", $input).as_str()), $expected);
                }
                }
            )*
        }
    }

    make_part2_sub_tests!(
        [
            part2_backtracking_memo
        ],
        [
            "brwrr" => 2,
            "bggr" => 1,
            "gbbr" => 4,
            "rrbgbr" => 6,
            "bwurrg" => 1,
            "brgr" => 2,
            "ubwu" => 0,
            "bbrgwb" => 0,
        ]
    );
}
