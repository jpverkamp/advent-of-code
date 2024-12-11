use hashbrown::HashMap;
use std::collections::BTreeMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn blink(input: &[u64], count: usize) -> usize {
    let mut input = input.to_vec();

    for _ in 0..count {
        input = input
            .iter()
            .flat_map(|&v| {
                if v == 0 {
                    vec![1]
                } else {
                    let digit_count = v.ilog10() + 1;
                    if digit_count % 2 == 0 {
                        let divisor = 10u64.pow(digit_count / 2);
                        vec![v / divisor, v % divisor]
                    } else {
                        vec![v * 2024]
                    }
                }
            })
            .collect();
    }

    input.len()
}

#[aoc(day11, part1, v1)]
fn part1_v1(input: &[u64]) -> usize {
    blink(input, 25)
}

// #[aoc(day11, part2, v1)]
// fn part2_v1(input: &[u64]) -> usize {
//     blink(input, 75)
// }

// Solve it recursively instead

fn blink_recur(input: &[u64], count: usize) -> usize {
    fn recur(value: u64, depth: usize) -> usize {
        if depth == 0 {
            1
        } else if value == 0 {
            recur(1, depth - 1)
        } else {
            let digit_count = value.ilog10() + 1;
            if digit_count % 2 == 0 {
                let divisor = 10u64.pow(digit_count / 2);
                recur(value / divisor, depth - 1) + recur(value % divisor, depth - 1)
            } else {
                recur(value * 2024, depth - 1)
            }
        }
    }

    input.iter().map(|&v| recur(v, count)).sum::<usize>()
}

#[aoc(day11, part1, recursive)]
fn part1_recursive(input: &[u64]) -> usize {
    blink_recur(input, 25)
}

// #[aoc(day11, part2, recursive)]
// fn part2_recursive(input: &[u64]) -> usize {
//     blink_recur(input, 75)
// }

// Add memoization

fn blink_recur_memo(input: &[u64], count: usize) -> usize {
    fn recur(cache: &mut HashMap<(u64, usize), usize>, value: u64, depth: usize) -> usize {
        if let Some(&v) = cache.get(&(value, depth)) {
            return v;
        }

        let result = if depth == 0 {
            1
        } else if value == 0 {
            recur(cache, 1, depth - 1)
        } else {
            let digit_count = value.ilog10() + 1;
            if digit_count % 2 == 0 {
                let divisor = 10u64.pow(digit_count / 2);
                recur(cache, value / divisor, depth - 1) + recur(cache, value % divisor, depth - 1)
            } else {
                recur(cache, value * 2024, depth - 1)
            }
        };

        cache.insert((value, depth), result);
        result
    }

    let mut cache = HashMap::new();
    input
        .iter()
        .map(|&v| recur(&mut cache, v, count))
        .sum::<usize>()
}

#[aoc(day11, part1, recursive_memo)]
fn part1_recursive_memo(input: &[u64]) -> usize {
    blink_recur_memo(input, 25)
}

#[aoc(day11, part2, recursive_memo)]
fn part2_recursive_memo(input: &[u64]) -> usize {
    blink_recur_memo(input, 75)
}

// Try an association list

fn blink_recur_memo_assoc(input: &[u64], count: usize) -> usize {
    fn recur(cache: &mut Vec<(u64, usize, usize)>, value: u64, depth: usize) -> usize {
        if let Some(r) = cache.iter().find_map(|(v, d, r)| {
            if *v == value && *d == depth {
                Some(*r)
            } else {
                None
            }
        }) {
            return r;
        }

        let result = if depth == 0 {
            1
        } else if value == 0 {
            recur(cache, 1, depth - 1)
        } else {
            let digit_count = value.ilog10() + 1;
            if digit_count % 2 == 0 {
                let divisor = 10u64.pow(digit_count / 2);
                recur(cache, value / divisor, depth - 1) + recur(cache, value % divisor, depth - 1)
            } else {
                recur(cache, value * 2024, depth - 1)
            }
        };

        cache.push((value, depth, result));
        result
    }

    let mut cache = vec![];
    input
        .iter()
        .map(|&v| recur(&mut cache, v, count))
        .sum::<usize>()
}

#[aoc(day11, part1, recursive_memo_assoc)]
fn part1_recursive_memo_assoc(input: &[u64]) -> usize {
    blink_recur_memo_assoc(input, 25)
}

#[aoc(day11, part2, recursive_memo_assoc)]
fn part2_recursive_memo_assoc(input: &[u64]) -> usize {
    blink_recur_memo_assoc(input, 75)
}

// Try a btree

fn blink_recur_memo_btree(input: &[u64], count: usize) -> usize {
    fn recur(cache: &mut BTreeMap<(u64, usize), usize>, value: u64, depth: usize) -> usize {
        if let Some(&v) = cache.get(&(value, depth)) {
            return v;
        }

        let result = if depth == 0 {
            1
        } else if value == 0 {
            recur(cache, 1, depth - 1)
        } else {
            let digit_count = value.ilog10() + 1;
            if digit_count % 2 == 0 {
                let divisor = 10u64.pow(digit_count / 2);
                recur(cache, value / divisor, depth - 1) + recur(cache, value % divisor, depth - 1)
            } else {
                recur(cache, value * 2024, depth - 1)
            }
        };

        cache.insert((value, depth), result);
        result
    }

    let mut cache = BTreeMap::new();
    input
        .iter()
        .map(|&v| recur(&mut cache, v, count))
        .sum::<usize>()
}

#[aoc(day11, part1, recursive_memo_btree)]
fn part1_recursive_memo_btree(input: &[u64]) -> usize {
    blink_recur_memo_btree(input, 25)
}

#[aoc(day11, part2, recursive_memo_btree)]
fn part2_recursive_memo_btree(input: &[u64]) -> usize {
    blink_recur_memo_btree(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "125 17";

    make_test!([part1_v1, part1_recursive, part1_recursive_memo, part1_recursive_memo_assoc, part1_recursive_memo_btree] => "day11.txt", 55312, 194482);
    make_test!([part2_recursive_memo] => "day11.txt", "65601038650482", "232454623677743");
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_recursive_memo(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_recursive_memo(&parse(input)).to_string()
}
