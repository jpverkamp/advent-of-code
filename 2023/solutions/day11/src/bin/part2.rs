use anyhow::Result;
use itertools::Itertools;
use std::io;

use day11::types::*;

// aoc_test::generate!{day11_part2_test_11 as "test/11.txt" => "1030"} // with n = 10
// aoc_test::generate!{day11_part2_test_11 as "test/11.txt" => "8410"} // with n = 100
aoc_test::generate!{day11_part2_test_11 as "test/11.txt" => "82000210"} // with n = 1_000_000
aoc_test::generate!{day11_part2_11 as "11.txt" => "685038186836"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let mut galaxy = Galaxy::from(input);

    // galaxy.expand(10);
    // galaxy.expand(100);
    galaxy.expand(1_000_000);

    Ok((
        galaxy
            .stars
            .iter()
            .cartesian_product(galaxy.stars.iter())
            .map(|(a, b)| a.manhattan_distance(b))
            .sum::<i128>()
            / 2
        // we're double counting
    )
    .to_string())
}
