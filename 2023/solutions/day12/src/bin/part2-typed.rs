use anyhow::Result;
use fxhash::FxHashMap;
use std::io;

use day12::{parse, types::*};

type Key<'a> = (&'a [Condition], Condition, Condition, &'a [u64], u64);
struct Solver<'a> {
    cache: FxHashMap<Key<'a>, u128>,
}

impl<'a> Solver<'a> {
    fn new() -> Self {
        Self {
            cache: FxHashMap::default(),
        }
    }

    fn check(
        &mut self,
        s: &'a [Condition], // The remaining input string after current
        curr: Condition,    // The current character to check
        prev: Condition,    // The previous character to check
        groups: &'a [u64],  // The remaining groups to match
        count: u64,         // The size of the current group
    ) -> u128 {
        use Condition::*;
        let key = (s, curr, prev, groups, count);

        if let Some(value) = self.cache.get(&key) {
            return *value;
        }

        let result = {
            if groups.is_empty() {
                // Base case, we have no more groups to go
                // Everything else must not be #
                if curr == Operational || s.iter().any(|c| *c == Operational) {
                    0
                } else {
                    1
                }
                // From here on out, we know groups is not empty
            } else if curr == Unknown {
                // Current is unknown, try both cases (without advancing s!)
                let if_d = self.check(s, Damaged, prev, groups, count);
                let if_o = self.check(s, Operational, prev, groups, count);
                if_d + if_o
            } else if s.is_empty() {
                // This block seems wrong, but I need it to have curr and prev work with ?
                // We have no more input, check the last current
                // We have at least one group at this point
                if curr == Operational {
                    // If the last current is operational, we need to match the last group
                    if groups.len() == 1 && count + 1 == groups[0] {
                        1
                    } else {
                        0
                    }
                } else if curr == Damaged {
                    // If we came from operational check the last group
                    if groups.len() == 1 && count == groups[0] {
                        1
                    } else {
                        0
                    }
                } else {
                    panic!("got something weird on empty input: {curr:?}")
                }
            } else if curr == Operational {
                // Current is operational
                if prev == Damaged {
                    // After damaged, start a new group
                    self.check(&s[1..], s[0], curr, groups, 1)
                } else if prev == Operational {
                    // After another operational, continue group
                    self.check(&s[1..], s[0], curr, groups, count + 1)
                } else {
                    panic!("got # after something weird: {prev:?}")
                }
            } else if curr == Damaged {
                // Current is damaged
                if prev == Damaged {
                    // After another damaged, nothing happens
                    self.check(&s[1..], s[0], curr, groups, 0)
                } else if prev == Operational {
                    // After operational, finish the current group
                    // If the size doesn't match, this branch is immediately invalid
                    if count == groups[0] {
                        self.check(&s[1..], s[0], curr, &groups[1..], 0)
                    } else {
                        0
                    }
                } else {
                    panic!("got . after something weird: {prev:?}")
                }
            } else {
                panic!("got something weird: {curr:?}")
            }
        };

        // dbg!(result);

        self.cache.insert(key, result);
        result
    }
}

aoc_test::generate!{day12_part2_typed_test_12 as "test/12.txt" => "525152"}
aoc_test::generate!{day12_part2_typed_12 as "12.txt" => "11461095383315"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    use Condition::*;

    let (s, springs) = parse::springs(&input).unwrap();
    assert_eq!(s.trim(), "");

    Ok(springs
        .iter()
        .map(|spring| Spring {
            conditions: (spring
                .conditions
                .clone()
                .into_iter()
                .chain(std::iter::once(Unknown))
                .collect::<Vec<_>>())
            .into_iter()
            .cycle()
            .take(spring.conditions.len() * 5 + 4)
            .collect::<Vec<_>>(),
            groups: spring
                .groups
                .clone()
                .into_iter()
                .cycle()
                .take(spring.groups.len() * 5)
                .collect::<Vec<_>>(),
        })
        .map(|spring| Solver::new().check(&spring.conditions, Damaged, Damaged, &spring.groups, 0))
        .sum::<u128>()
        .to_string())
}
