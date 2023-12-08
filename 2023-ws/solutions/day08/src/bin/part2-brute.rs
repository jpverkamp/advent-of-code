use anyhow::Result;
use std::io;

use day08::{parse, types::*};

// #[aoc_test("data/test/08.txt", "6")]
// #[aoc_test("data/test/08b.txt", "6")]
// #[aoc_test("data/08.txt", "")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let (s, ref simulation) = parse::simulation(&input).unwrap();
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

    println!("{result}");
    Ok(())
}
