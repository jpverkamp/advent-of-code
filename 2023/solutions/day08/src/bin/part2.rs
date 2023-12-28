use anyhow::Result;
use std::{collections::BTreeMap, io};

use day08::{parse, types::*};

aoc_test::generate!{day08_part2_test_08 as "test/08.txt" => "3"}
aoc_test::generate!{day08_part2_test_08b as "test/08b.txt" => "6"}
aoc_test::generate!{day08_part2_test_08c as "test/08c.txt" => "10"}
aoc_test::generate!{day08_part2_08 as "08.txt" => "9064949303801"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    // https://math.stackexchange.com/questions/2218763/how-to-find-lcm-of-two-numbers-when-one-starts-with-an-offset
    // I actually originally checked for any Z's along the cycle
    // But in the given test case it doesn't matter and in the final example it doesn't happen
    // It's weird to me that when the cycle starts doesn't end up matter? But it works

    let (s, ref simulation) = parse::simulation(input).unwrap();
    assert_eq!(s.trim(), "");

    // Get all nodes that end in A
    let starts = simulation
        .neighbors
        .keys()
        .filter(|l| l[2] == 'A')
        .cloned()
        .collect::<Vec<_>>();

    // For each node, determine how long of a cycle it has
    // This will be where you see the same node + position in input list twice
    let cycles = starts
        .iter()
        .map(|each| {
            let mut current = *each;
            let mut cycle_length: usize = 0;
            let mut count = 0;

            // Previous states: position in input list + node
            let mut visited = BTreeMap::new();

            for (i, m) in simulation.moves.iter().enumerate().cycle() {
                count += 1;

                // If we're in a final state we've seen before, we have a cycle
                if current[2] == 'Z' && visited.contains_key(&(i, current)) {
                    let cycle_start: usize = visited[&(i, current)];
                    cycle_length = count - cycle_start;
                    break;
                }

                // Otherwise, record this state and update
                visited.insert((i, current), count);
                current = match m {
                    Move::Left => simulation.neighbors[&current].left,
                    Move::Right => simulation.neighbors[&current].right,
                };
            }

            cycle_length
        })
        .collect::<Vec<_>>();

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm(a: usize, b: usize) -> usize {
        a / gcd(a, b) * b
    }

    Ok(cycles.clone().into_iter().reduce(lcm).unwrap().to_string())
}
