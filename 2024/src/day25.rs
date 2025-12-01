use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocksAndKeys {
    Lock([u8; 5]),
    Key([u8; 5]),
}

#[aoc_generator(day25)]
fn parse(input: &str) -> Vec<LocksAndKeys> {
    let mut lines = input.lines();
    let mut result = vec![];

    loop {
        let mut buffer = vec![];
        for _ in 0..7 {
            let line = lines.next().unwrap();
            buffer.push(line);
        }

        let is_lock = buffer[0].starts_with('#');
        let mut values = [0u8; 5];

        // First and last line are always ### and ...
        // For locks, it's #s going down, otherwise . going up
        #[allow(clippy::needless_range_loop)]
        for x in 0..5 {
            values[x] = if is_lock {
                (0..5)
                    .find(|y| buffer[y + 1].chars().nth(x).unwrap() == '.')
                    .unwrap_or(5)
            } else {
                (0..5)
                    .find(|y| buffer[5 - y].chars().nth(x).unwrap() == '.')
                    .unwrap_or(5)
            } as u8;
        }

        result.push(if is_lock {
            LocksAndKeys::Lock(values)
        } else {
            LocksAndKeys::Key(values)
        });

        if lines.next().is_none() {
            break;
        }
    }

    result
}

#[aoc(day25, part1, v1)]
fn part1_v1(input: &[LocksAndKeys]) -> usize {
    use LocksAndKeys::*;

    input
        .iter()
        .permutations(2)
        .filter(|p| {
            let a = p[0];
            let b = p[1];

            // We'll generate both orders, so only count the one with the lock first
            // A lock and key match if there's no overlap *not* if they're exact
            match (a, b) {
                (Lock(a), Key(b)) => a.iter().zip(b.iter()).all(|(a, b)| a + b <= 5),
                _ => false,
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    make_test!([part1_v1] => "day25.txt", 3, 2835);
}
