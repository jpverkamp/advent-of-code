use anyhow::Result;
use itertools::Itertools;
use std::io;

use day11::types::*;

// #[aoc_test("data/test/11.txt", "1030")] // with n = 10
// #[aoc_test("data/test/11.txt", "8410")] // with n = 100
// #[aoc_test("data/test/11.txt", "82000210")] // with n = 1_000_000
// #[aoc_test("data/11.txt", "685038871866")] // too high
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let mut galaxy = Galaxy::from(input);

    // galaxy.expand(10);
    // galaxy.expand(100);
    galaxy.expand(1_000_000);

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
