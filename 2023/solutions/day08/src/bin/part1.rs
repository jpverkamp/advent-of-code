use anyhow::Result;
use std::io;

use day08::{parse, types::*};

// #[aoc_test("data/test/08.txt", "6")]
// #[aoc_test("data/08.txt", "12737")]
fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, ref simulation) = parse::simulation(input).unwrap();
    assert_eq!(s.trim(), "");

    let mut current: Label = ['A', 'A', 'A'];
    let target: Label = ['Z', 'Z', 'Z'];
    let mut result = 0;

    for m in simulation.moves.iter().cycle() {
        result += 1;

        current = match m {
            Move::Left => simulation.neighbors[&current].left,
            Move::Right => simulation.neighbors[&current].right,
        };

        if current == target {
            break;
        }
    }

    Ok(result.to_string())
}
