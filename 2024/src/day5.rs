use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Default, Clone)]
pub struct Ordering {
    data: hashbrown::HashMap<u32, hashbrown::HashSet<u32>>,
}

impl Ordering {
    pub fn new() -> Self {
        Self {
            data: hashbrown::HashMap::new(),
        }
    }

    pub fn values(&self) -> Vec<u32> {
        self.data.keys().copied().collect()
    }

    pub fn insert(&mut self, a: u32, b: u32) {
        self.data.entry(a).or_default().insert(b);
    }

    // This was my original (more complicated!) version, but it's not actually correct

    /*
    Imagine this input:

        98|51
        51|22
        22|98

    This would imply both that 98 is before 51 but that 51 is before 22 which is before 98.

    But... that doesn't make any sense... *unless* you can never a valid list that has all three.

    If you have 98 and 51, 98 goes first. But if you have 51,22 or 22,98 those are correct.

    I expect this would do funny things to sort_by if you end up with all three :smile:
    */

    // To proceed, either a is directly before b or recursively before it
    pub fn can_precede_transitive(&self, a: u32, b: u32) -> bool {
        self.data.contains_key(&a)
            && (self.data[&a].contains(&b) || self.data[&a].iter().any(|&c| self.can_precede(c, b)))
    }

    pub fn can_precede_transitive_path(&self, a: u32, b: u32) -> Option<Vec<u32>> {
        if !self.data.contains_key(&a) {
            return None;
        }

        if self.data[&a].contains(&b) {
            return Some(vec![a, b]);
        }

        for &c in &self.data[&a] {
            if let Some(mut path) = self.can_precede_transitive_path(c, b) {
                path.insert(0, a);
                return Some(path);
            }
        }

        None
    }

    pub fn can_precede(&self, a: u32, b: u32) -> bool {
        !self.data.contains_key(&b) || !self.data[&b].contains(&a)
    }

    // A list is valid iff all elements are in order by this ordering
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
