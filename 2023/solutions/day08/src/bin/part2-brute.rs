use anyhow::Result;
use std::io;

use day08::{parse, types::*};

// Using:

// $ time just run 8 2-brute | ts
//
// cat data/$(printf "%02d" 8).txt | cargo run --release -p day$(printf "%02d" 8) --bin part2-brute
// [2023-12-08 01:40:42] --- <ts> ---
//    Compiling day08 v0.1.0 (/Users/jp/Projects/advent-of-code/2023-ws/solutions/day08)
//     Finished release [optimized] target(s) in 0.16s
//      Running `target/release/part2-brute`
// [2023-12-08 01:41:08] 100000000
// [2023-12-08 01:41:34] 200000000
// [2023-12-08 01:42:00] 300000000

// 9064949303801 / (100000000/(26 seconds)) ~= 2.5e6 seconds ~= 27 days 6 hours

// #[aoc_test("data/test/08.txt", "3")]
// #[aoc_test("data/test/08b.txt", "6")]
// #[aoc_test("data/test/08c.txt", "10")]
// #[aoc_test("data/08.txt", "9064949303801")]
fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, ref simulation) = parse::simulation(input).unwrap();
    assert_eq!(s.trim(), "");

    // Get all nodes that end in A
    let mut current = simulation
        .neighbors
        .keys()
        .filter(|l| l[2] == 'A')
        .cloned()
        .collect::<Vec<_>>();

    // Count cycles
    let mut result = 0;
    for m in simulation.moves.iter().cycle() {
        result += 1;
        if result % 100_000_000 == 0 {
            println!("{result}");
        }

        // Update all nodes
        current = current
            .into_iter()
            .map(|l| match m {
                Move::Left => simulation.neighbors[&l].left,
                Move::Right => simulation.neighbors[&l].right,
            })
            .collect::<Vec<_>>();

        // If all nodes end in Z, we can exit
        if current.iter().all(|l| l[2] == 'Z') {
            break;
        }
    }

    Ok(result.to_string())
}
