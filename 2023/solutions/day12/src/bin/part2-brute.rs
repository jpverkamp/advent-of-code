use anyhow::Result;
use std::{collections::VecDeque, io};

use day12::{parse, types::*};

// #[aoc_test("data/test/12.txt", "21")]
// #[aoc_test("data/12.txt", "7025")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;

    fn drep(s: &str, d: &str, n: usize) -> String {
        std::iter::repeat(s).take(n).collect::<Vec<_>>().join(d)
    }

    let input = input
        .lines()
        .map(|line| {
            let parts = line.split_once(' ').unwrap();
            drep(parts.0, "?", 5) + " " + &drep(parts.1, ",", 5)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let (s, springs) = parse::springs(&input).unwrap();
    assert_eq!(s.trim(), "");

    let result = springs
        .iter()
        .map(|s| {
            use Condition::*;

            let mut possibles = 0;
            let mut queue = VecDeque::new();
            queue.push_back(s.clone());

            let mut i = 0;
            while let Some(current) = queue.pop_back() {
                if i % 1_000_000 == 0 {
                    println!("{i}, q={}, p={possibles}: {current}", queue.len());
                }
                i += 1;

                // If the current state is impossible, skip it
                if !current.is_valid() {
                    // println!("{current} is invalid");
                    continue;
                }

                // If it is possible and completely known, score it
                if current.is_correct() {
                    // println!("{current} is scoring");
                    possibles += 1;
                    continue;
                }

                // Otherwise, queue one in with each possibility
                for (i, condition) in current.conditions.iter().enumerate() {
                    if !condition.is_known() {
                        let mut next = current.clone();
                        next.conditions[i] = Operational;
                        queue.push_back(next);

                        let mut next = current.clone();
                        next.conditions[i] = Damaged;
                        queue.push_back(next);
                        break;
                    }
                }
            }

            dbg!(possibles)
        })
        .sum::<u64>();

    println!("{result}");
    Ok(())
}
