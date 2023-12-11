use anyhow::Result;
use itertools::Itertools;
use std::io;

use day11::types::*;

// #[aoc_test("data/test/11.txt", "374")]
// #[aoc_test("data/11.txt", "9556896")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let mut galaxy = Galaxy::from(input);

    galaxy.expand(1);

    let result = galaxy
        .stars
        .iter()
        .cartesian_product(galaxy.stars.iter())
        .map(|(a, b)| a.manhattan_distance(b))
        .sum::<i128>()
        / 2; // we're double counting

    println!("{result}");
    Ok(())
}
