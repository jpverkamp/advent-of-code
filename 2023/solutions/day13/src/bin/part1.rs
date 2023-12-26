use anyhow::Result;
use std::io;

use day13::types::*;

// #[aoc_test("data/test/13.txt", "405")]
// #[aoc_test("data/13.txt", "43614")]
fn main() {
    let stdin = io::stdin();

    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    Ok(input
        .split("\n\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|input| {
            let ashflow = AshFlow::from(*input);
            let mut result = 0;

            // TODO: Is it possible to have more than one mirror?
            // in the input cases, no

            for x_axis in ashflow.bounds.min_x..ashflow.bounds.max_x {
                if ashflow.rocks.iter().all(|p| {
                    let pr = p.reflect_x(x_axis);
                    !ashflow.bounds.contains(&pr) || ashflow.rocks.contains(&pr)
                }) {
                    result += x_axis + 1;
                }
            }

            for y_axis in ashflow.bounds.min_y..ashflow.bounds.max_y {
                if ashflow.rocks.iter().all(|p| {
                    let pr = p.reflect_y(y_axis);
                    !ashflow.bounds.contains(&pr) || ashflow.rocks.contains(&pr)
                }) {
                    result += 100 * (y_axis + 1);
                }
            }

            result
        })
        .sum::<isize>()
        .to_string())
}
