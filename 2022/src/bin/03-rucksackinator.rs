use std::{path::Path, collections::HashSet};
use aoc::*;

#[derive(Debug)]
struct Rucksack {
    left: HashSet<char>,
    right: HashSet<char>,
}

impl Rucksack {
    fn new(items: String) -> Rucksack {
        let half = items.len() / 2;
        let left = items.chars().take(half).collect();
        let right = items.chars().skip(half).collect();

        Rucksack { left, right }
    }

    fn all(self) -> HashSet<char> {
        self.left.union(&self.right).copied().collect()
    }
}

fn rucksack_priority(c: &char) -> u32 {
    match c {
        'a'..='z' => (*c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (*c as u32) - ('A' as u32) + 27,
        _ => panic!("unknown rucksack character: {:?}", c)
    }
}

fn part1(filename: &Path) -> String {
    let lines: Vec<String> = read_lines(filename);

    let rucksacks: Vec<Rucksack> = lines.into_iter().map(Rucksack::new).collect();

    let uniques: Vec<Vec<&char>> = rucksacks.iter().map(
        |r| r.left.intersection(&r.right).collect()
    ).collect();

    let priorities: Vec<Vec<u32>> = uniques.into_iter().map(
        |ls| ls.into_iter().map(rucksack_priority).collect()
    ).collect();

    priorities.into_iter().map(|ls| ls.iter().sum::<u32>()).sum::<u32>().to_string()
}

fn part2(filename: &Path) -> String {
    let lines: Vec<String> = read_lines(filename);

    let rucksacks: Vec<Rucksack> = lines.into_iter().map(Rucksack::new).collect();

    let groups: Vec<&[Rucksack]> = rucksacks.chunks(3).collect();

    let uniques: Vec<HashSet<char>> = groups.into_iter().map(
        |g| {
            let s1: HashSet<char> = g[0].left.union(&g[0].right).copied().collect();
            let s2: HashSet<char> = g[1].left.union(&g[1].right).copied().collect();
            let s3: HashSet<char> = g[2].left.union(&g[2].right).copied().collect();

            let i12: HashSet<char> = s1.intersection(&s2).copied().collect();
            let i123: HashSet<char> = i12.intersection(&s3).copied().collect();

            i123
        }
    ).collect();

    let priorities: Vec<Vec<u32>> = uniques.into_iter().map(
        |ls| ls.iter().map(rucksack_priority).collect()
    ).collect();

    priorities.into_iter().map(|ls| ls.iter().sum::<u32>()).sum::<u32>().to_string()
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use aoc::aoc_test;
    use crate::{part1, part2};

    #[test]   
    fn test1() { aoc_test("03", part1, "7845") }

    #[test]
    fn test2() { aoc_test("03", part2, "2790") }
}
