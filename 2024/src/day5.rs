use aoc_runner_derive::{aoc, aoc_generator};
use bitvec::{prelude::bitvec, vec::BitVec};

#[derive(Debug, Clone, Default)]
pub struct Ordering {
    data: BitVec,
}

impl Ordering {
    pub fn new() -> Self {
        Self {
            data: bitvec![0; 100*100],
        }
    }

    pub fn insert(&mut self, a: u32, b: u32) {
        self.data.set((a as usize) * 100 + (b as usize), true);
    }

    pub fn can_precede(&self, a: u32, b: u32) -> bool {
        self.data[(a as usize) * 100 + (b as usize)]
    }

    pub fn validates(&self, list: &[u32]) -> bool {
        list.iter().is_sorted_by(|&a, &b| self.can_precede(*a, *b))
    }
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> (Ordering, Vec<Vec<u32>>) {
    use nom::{
        character::complete::{self, newline},
        multi::{many1, separated_list1},
        sequence::separated_pair,
    };

    fn parse_ordering(input: &str) -> nom::IResult<&str, Ordering> {
        let (rest, ls) = separated_list1(
            newline,
            separated_pair(complete::u32, complete::char('|'), complete::u32),
        )(input)?;

        let mut ordering = Ordering::new();
        for (a, b) in ls {
            ordering.insert(a, b);
        }
        Ok((rest, ordering))
    }

    fn parse_list(input: &str) -> nom::IResult<&str, Vec<u32>> {
        separated_list1(complete::char(','), complete::u32)(input)
    }

    fn parse_input(input: &str) -> nom::IResult<&str, (Ordering, Vec<Vec<u32>>)> {
        let (input, ordering) = parse_ordering(input)?;
        let (input, _) = many1(newline)(input)?;
        let (input, data) = separated_list1(newline, parse_list)(input)?;
        Ok((input, (ordering, data)))
    }

    parse_input(input).unwrap().1
}

#[aoc(day5, part1, v1)]
fn part1_v1((ordering, data): &(Ordering, Vec<Vec<u32>>)) -> u32 {
    data.iter()
        .filter(|list| ordering.validates(list))
        .map(|list| list[list.len() / 2])
        .sum()
}

#[aoc(day5, part2, v1)]
fn part2_v1((ordering, data): &(Ordering, Vec<Vec<u32>>)) -> u32 {
    data.iter()
        .filter(|list| !ordering.validates(list))
        .map(|list| {
            // TODO: I don't want to have to clone this here, but AOC requires it
            let mut list = list.clone();
            list.sort_by(|&a, &b| {
                if ordering.can_precede(a, b) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            list
        })
        .map(|list| list[list.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::make_test;

    use super::*;

    const EXAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_validate() {
        let (ordering, data) = parse(EXAMPLE);

        assert!(ordering.validates(&data[0]));
        assert!(ordering.validates(&data[1]));
        assert!(ordering.validates(&data[2]));
        assert!(!ordering.validates(&data[3]));
        assert!(!ordering.validates(&data[4]));
        assert!(!ordering.validates(&data[5]));
    }

    make_test!([part1_v1] => "day5.txt", 143, 4924);
    make_test!([part2_v1] => "day5.txt", 123, 6085);
}
