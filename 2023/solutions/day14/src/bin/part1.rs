use anyhow::Result;
use std::io;

use day14::types::*;

aoc_test::generate!{day14_part1_test_14 as "test/14.txt" => "136"}
aoc_test::generate!{day14_part1_14 as "14.txt" => "110274"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let mut platform = Platform::from(input);

    // Let the rocks slide until they stop moving
    loop {
        let mut changed = false;

        for i in 0..platform.round_rocks.len() {
            // Get current point; if we're at the top already, skip
            let r = platform.round_rocks[i];
            let next = Point { x: r.x, y: r.y - 1 };

            // Check that the next point is available
            if !platform.bounds.contains(&next)
                || platform.round_rocks.contains(&next)
                || platform.cube_rocks.contains(&next)
            {
                continue;
            }

            // If we get here, we can move; do it
            platform.round_rocks[i].y = next.y;
            changed = true;
        }

        if !changed {
            break;
        }
    }

    // Calculate final score
    Ok(platform
        .round_rocks
        .iter()
        .map(|r| platform.bounds.max_y - r.y + 1)
        .sum::<isize>()
        .to_string())
}
