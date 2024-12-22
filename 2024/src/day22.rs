use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use itertools::{repeat_n, Itertools};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SuperSecretPseudoRandomNumberGenerator {
    pub value: u64,
}

impl SuperSecretPseudoRandomNumberGenerator {
    pub fn new(seed: u64) -> Self {
        Self { value: seed }
    }
}

impl Iterator for SuperSecretPseudoRandomNumberGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.value ^= self.value << 6; // times 64, mix
        self.value &= 0x00FF_FFFF;
        self.value ^= self.value >> 5; // divide by 32, mix
        self.value &= 0x00FF_FFFF;
        self.value ^= self.value << 11; // times 2048, mix
        self.value &= 0x00FF_FFFF;

        Some(self.value)
    }
}

#[aoc_generator(day22)]
pub fn parse(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day22, part1, v1)]
fn part1_v1(input: &[u64]) -> u64 {
    input
        .iter()
        .map(|&seed| {
            SuperSecretPseudoRandomNumberGenerator::new(seed)
                .into_iter()
                .nth(1999) // It's zero based :)
                .unwrap()
        })
        .sum()
}

// #[aoc(day22, part2, bruteforce)]
#[allow(dead_code)]
fn part2_bruteforce(input: &[u64]) -> usize {
    fn banana_score(input: &[u64], seq: &[i8]) -> usize {
        input
            .iter()
            .map(|&seed| {
                let mut rng = SuperSecretPseudoRandomNumberGenerator::new(seed);
                let mut previous_ones = (seed % 10) as i8;
                let mut delta_buffer = VecDeque::new();

                let mut index = 0;
                loop {
                    index += 1;
                    let value = rng.next().unwrap();
                    let ones = (value % 10) as i8;

                    delta_buffer.push_back(ones - previous_ones);
                    if delta_buffer.len() > 4 {
                        delta_buffer.pop_front();
                    }

                    previous_ones = ones;

                    if delta_buffer.len() == 4
                        && delta_buffer[0] == seq[0]
                        && delta_buffer[1] == seq[1]
                        && delta_buffer[2] == seq[2]
                        && delta_buffer[3] == seq[3]
                    {
                        break;
                    }

                    if index >= 2_000 {
                        return 0;
                    }
                }

                previous_ones as usize
            })
            .sum()
    }

    repeat_n(-9..=9, 4)
        .multi_cartesian_product()
        .map(|seq| banana_score(input, seq.as_slice()))
        .max()
        .unwrap()
}

#[aoc(day22, part2, seqscore)]
fn part2_seqscore(input: &[u64]) -> usize {
    let mut sequence_scores = HashMap::new();

    input.iter().for_each(|&seed| {
        // Find the first time each sequence appears and store the score for that sequence
        let mut local_sequence_scores = HashMap::new();
        let mut delta_buffer = VecDeque::new();

        SuperSecretPseudoRandomNumberGenerator::new(seed)
            .into_iter()
            .take(2_000)
            .fold((seed % 10) as i8, |previous_ones, value| {
                let ones = (value % 10) as i8;

                delta_buffer.push_back(ones - previous_ones);
                if delta_buffer.len() > 4 {
                    delta_buffer.pop_front();
                }

                if delta_buffer.len() == 4 {
                    let key = (
                        delta_buffer[0],
                        delta_buffer[1],
                        delta_buffer[2],
                        delta_buffer[3],
                    );
                    if !local_sequence_scores.contains_key(&key) {
                        local_sequence_scores.insert(key, ones as usize);
                    }
                }

                ones
            });

        // Add the new local sequence scores to the overall map
        local_sequence_scores.into_iter().for_each(|(key, value)| {
            sequence_scores
                .entry(key)
                .and_modify(|v| *v += value)
                .or_insert(value);
        });
    });

    // Find whichever sequence has the highest overall score
    sequence_scores
        .into_iter()
        .map(|(_key, value)| value)
        .max()
        .unwrap()
}

#[aoc(day22, part2, bitbuffer)]
fn part2_bitbuffer(input: &[u64]) -> usize {
    let mut sequence_scores = HashMap::new();

    input.iter().for_each(|&seed| {
        // Find the first time each sequence appears and store the score for that sequence
        let mut local_sequence_scores = HashMap::new();

        // The delta buffer is 4x 5-bit numbers
        let mut delta_buffer = 0u32;

        SuperSecretPseudoRandomNumberGenerator::new(seed)
            .into_iter()
            .take(2_000)
            .enumerate()
            .fold((seed % 10) as i8, |previous_ones, (index, value)| {
                let ones = (value % 10) as i8;

                delta_buffer <<= 5;
                delta_buffer |= (ones - previous_ones + 9) as u32;
                delta_buffer &= 0b11111_11111_11111_11111;

                if index > 4 && !local_sequence_scores.contains_key(&delta_buffer) {
                    local_sequence_scores.insert(delta_buffer, ones as usize);
                }

                ones
            });

        // Add the new local sequence scores to the overall map
        local_sequence_scores.into_iter().for_each(|(key, value)| {
            sequence_scores
                .entry(key)
                .and_modify(|v| *v += value)
                .or_insert(value);
        });
    });

    // Find whichever sequence has the highest overall score
    sequence_scores
        .into_iter()
        .map(|(_key, value)| value)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
1
10
100
2024";

    const EXAMPLE2: &str = "\
1
2
3
2024";

    make_test!([part1_v1] => "day22.txt", 37327623, "13764677935");
    make_test!([part2_bruteforce, part2_seqscore] => "day22.txt", 24, 1619);

    #[test]
    fn test_part2_bruteforce_example2() {
        assert_eq!(part2_bruteforce(&parse(EXAMPLE2)), 23);
    }

    #[test]
    fn test_part2_v2_example2() {
        assert_eq!(part2_seqscore(&parse(EXAMPLE2)), 23);
    }

    #[test]
    fn test_generator() {
        let mut rng = SuperSecretPseudoRandomNumberGenerator::new(123);

        assert_eq!(rng.next().unwrap(), 15887950);
        assert_eq!(rng.next().unwrap(), 16495136);
        assert_eq!(rng.next().unwrap(), 527345);
        assert_eq!(rng.next().unwrap(), 704524);
        assert_eq!(rng.next().unwrap(), 1553684);
        assert_eq!(rng.next().unwrap(), 12683156);
        assert_eq!(rng.next().unwrap(), 11100544);
        assert_eq!(rng.next().unwrap(), 12249484);
        assert_eq!(rng.next().unwrap(), 7753432);
        assert_eq!(rng.next().unwrap(), 5908254);
    }
}
