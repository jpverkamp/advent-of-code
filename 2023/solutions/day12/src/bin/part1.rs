use anyhow::Result;
use std::{collections::VecDeque, io};

use day12::{parse, types::*};

aoc_test::generate!{day12_part1_test_12 as "test/12.txt" => "21"}
aoc_test::generate!{day12_part1_12 as "12.txt" => "7025"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, springs) = parse::springs(input).unwrap();
    assert_eq!(s.trim(), "");

    Ok(springs
        .iter()
        .map(|s| {
            use Condition::*;

            let mut possibles = 0;
            let mut queue = VecDeque::new();
            queue.push_back(s.clone());

            while let Some(current) = queue.pop_front() {
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

            possibles
        })
        .sum::<u64>()
        .to_string())
}
