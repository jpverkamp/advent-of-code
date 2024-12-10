use aoc_runner_derive::{aoc, aoc_generator};
use bitvec::{prelude::bitvec, vec::BitVec};

use crate::Grid;

#[aoc_generator(day10)]
fn parse(input: &str) -> Grid<u8> {
    Grid::read(input, &|c| c.to_digit(10).unwrap() as u8)
}

#[aoc(day10, part1, search)]
fn part1_search(input: &Grid<u8>) -> u32 {
    input
        .iter_enumerate()
        .filter(|(_, &v)| v == 0)
        .map(|(p, _)| {
            // For each 0, search for how man 9s are reachable
            let mut checked = Grid::new(input.width, input.height);
            let mut queue = vec![p];
            let mut nines_reached = 0;

            while let Some(p) = queue.pop() {
                if input.get(p) == Some(&9) {
                    nines_reached += 1;
                    continue; // no points higher than 9
                }

                p.neighbors()
                    .iter()
                    .filter(|&p2| {
                        input.in_bounds(*p2)
                            && input.get(p).unwrap() + 1 == *input.get(*p2).unwrap()
                    })
                    .for_each(|p2| {
                        if !checked.get(*p2).unwrap_or(&false) {
                            checked.set(*p2, true);
                            queue.push(*p2);
                        }
                    });
            }

            nines_reached
        })
        .sum()
}

#[aoc(day10, part1, dynamic)]
fn part1_dynamic(input: &Grid<u8>) -> usize {
    let mut trail_counts: Grid<BitVec> = Grid::new(input.width, input.height);

    // How many 9s are there? 
    let nines = input.iter().filter(|&&v| v == 9).count();

    // Flag each 9 with a unique bit
    let mut index = 0;
    input.iter_enumerate().for_each(|(p, &v)| {
        if v == 9 {
            trail_counts.set(p, {
                let mut b = bitvec![0; nines];
                b.set(index, true);
                b
            });
            index += 1;
        }
    });

    // For each height, we're going to OR the bits of reachable 9s together
    for height in (0..=8).rev() {
        input.iter_enumerate().for_each(|(p, &v)| {
            if v == height {
                trail_counts.set(
                    p,
                    p.neighbors()
                        .iter()
                        .filter(|&p2| input.get(*p2).is_some_and(|&v| v == height + 1))
                        .map(|&p2| trail_counts.get(p2).unwrap().clone())
                        .reduce(|a, b| a | b)
                        .unwrap_or_else(|| bitvec![0; nines])
                );
            }
        });
    }

    // Sum the ratings of the 9s
    input
        .iter_enumerate()
        .filter(|(_, &v)| v == 0)
        .map(|(p, _)| (*trail_counts.get(p).unwrap()).count_ones())
        .sum::<usize>()
}

#[aoc(day10, part1, dynamic_tupled)]
fn part1_dynamic_tupled(input: &Grid<u8>) -> usize {
    let mut trail_counts: Grid<(u128, u128)> = Grid::new(input.width, input.height);

    // Flag each 9 with a unique bit
    let mut index = 0;
    input.iter_enumerate().for_each(|(p, &v)| {
        if v == 9 {
            trail_counts.set(p, if index < 128 {
                (1 << index, 0)
            } else {
                (0, 1 << (index - 128))
            });
            index += 1;
        }
    });

    // Failsafe just in case we have more than 256 nines
    if index > 256 {
        return part1_search(input) as usize;
    }

    // For each height, we're going to OR the bits of reachable 9s together
    for height in (0..=8).rev() {
        input.iter_enumerate().for_each(|(p, &v)| {
            if v == height {
                trail_counts.set(
                    p,
                    p.neighbors()
                        .iter()
                        .filter(|&p2| input.get(*p2).is_some_and(|&v| v == height + 1))
                        .map(|&p2| trail_counts.get(p2).unwrap().clone())
                        .reduce(|(a1, a2), (b1, b2)| (a1 | b1, a2 | b2))
                        .unwrap_or_else(|| (0, 0))
                );
            }
        });
    }

    // Sum the ratings of the 9s
    input
        .iter_enumerate()
        .filter(|(_, &v)| v == 0)
        .map(|(p, _)| {
            let &(a, b) = trail_counts.get(p).unwrap();
            a.count_ones() as usize + b.count_ones() as usize
        })
        .sum::<usize>()
}

#[aoc(day10, part2, dynamic)]
fn part2_dynamic(input: &Grid<u8>) -> u32 {
    let mut ratings = Grid::new(input.width, input.height);

    // All 9s can be reached one way
    input.iter_enumerate().for_each(|(p, &v)| {
        if v == 9 {
            ratings.set(p, 1);
        }
    });

    // For each height, we're going to sum the ratings of all points one higher
    for height in (0..=8).rev() {
        input.iter_enumerate().for_each(|(p, &v)| {
            if v == height {
                ratings.set(
                    p,
                    p.neighbors()
                        .iter()
                        .filter(|&p2| input.get(*p2).is_some_and(|&v| v == height + 1))
                        .map(|p2| ratings.get(*p2).unwrap_or(&0))
                        .sum(),
                );
            }
        });
    }

    // Sum the ratings of the 0s
    input
        .iter_enumerate()
        .filter(|(_, &v)| v == 0)
        .map(|(p, _)| ratings.get(p).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    make_test!([part1_search, part1_dynamic, part1_dynamic_tupled] => "day10.txt", 36, 659);
    make_test!([part2_dynamic] => "day10.txt", 81, 1463);
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_search(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_dynamic(&parse(input)).to_string()
}
