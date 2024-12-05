use aoc_runner_derive::{aoc, aoc_generator};

struct Ordering {
    data: hashbrown::HashMap<u32, hashbrown::HashSet<u32>>,
}

impl Ordering {
    fn new() -> Self {
        Self {
            data: hashbrown::HashMap::new(),
        }
    }

    fn insert(&mut self, a: u32, b: u32) {
        self.data.entry(a).or_default().insert(b);
    }

    // Original version
    // This, for some reason, doesn't actually work
    // We actually only need to check that we *don't* have b|a
    // It works on their example, but not my tests cases :shrug:

    // To proceed, either a is directly before b or recursively before it
    // fn preceeds(&self, a: u32, b: u32) -> bool {
    //     self.data.contains_key(&a)
    //         && (self.data[&a].contains(&b) || self.data[&a].iter().any(|&c| self.preceeds(c, b)))
    // }

    fn preceeds(&self, a: u32, b: u32) -> bool {
        !self.data.contains_key(&b) || !self.data[&b].contains(&a)
    }

    // A list is valid iff all elements are in order by this ordering
    fn validates(&self, list: &[u32]) -> bool {
        list.iter().is_sorted_by(|&a, &b| self.preceeds(*a, *b))
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> (Ordering, Vec<Vec<u32>>) {
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
                if ordering.preceeds(a, b) {
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

    #[test]
    fn part1_example() {
        assert_eq!(part1_v1(&parse(EXAMPLE)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_v1(&parse(EXAMPLE)), 123);
    }
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_v1(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_v1(&parse(input)).to_string()
}
