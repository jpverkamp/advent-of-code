use anyhow::Result;
use std::io;

use day12::{parse, types::*};

aoc_test::generate!{day12_part1_orig_test_12 as "test/12.txt" => "21"}
aoc_test::generate!{day12_part1_orig_12 as "12.txt" => "7025"}

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
            let mut queue = Vec::new();
            queue.push(s.conditions.clone());

            while let Some(current) = queue.pop() {
                // If there are no unknown components, score it
                if current.iter().all(|c| c.is_known()) {
                    let groups = current
                        .iter()
                        .chain(std::iter::once(&Damaged))
                        .fold(
                            (Damaged, 0, vec![]),
                            |(previous, current_length, mut lengths), current| match (
                                previous, current,
                            ) {
                                (Operational, Operational) => {
                                    (Operational, current_length + 1, lengths)
                                }
                                (Operational, Damaged) => {
                                    lengths.push(current_length);
                                    (Damaged, 0, lengths)
                                }
                                (Damaged, Operational) => (Operational, 1, lengths),
                                (Damaged, Damaged) => (Damaged, 0, lengths),
                                _ => panic!(
                                    "Invalid state, previous: {:?}, current: {:?}",
                                    previous, current
                                ),
                            },
                        )
                        .2;

                    if groups == s.groups {
                        possibles += 1;
                    }
                } else {
                    // Otherwise, queue one in with each possibility
                    for (i, condition) in current.iter().enumerate() {
                        if !condition.is_known() {
                            let mut next = current.clone();
                            next[i] = Operational;
                            queue.push(next);

                            let mut next = current.clone();
                            next[i] = Damaged;
                            queue.push(next);
                            break;
                        }
                    }
                }
            }

            possibles
        })
        .sum::<u64>()
        .to_string())
}
