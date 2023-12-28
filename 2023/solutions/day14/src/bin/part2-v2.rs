use anyhow::Result;
use fxhash::FxHashMap;
use std::io;

use day14::types::*;

aoc_test::generate!{day14_part2_v2_test_14 as "test/14.txt" => "64"}
aoc_test::generate!{day14_part2_v2_14 as "14.txt" => "90982"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let mut platform = PlatformV2::from(Platform::from(input));

    let mut seen = FxHashMap::default();

    const TARGET: i32 = 1_000_000_000;

    for cycle in 0..=TARGET {
        // Check if we've seen this platform state before (it's deterministic, thus cycling)
        // Keep going until the cycle is in the same phase as the TARGET
        let key = platform.to_string();
        if let Some(cycle_start) = seen.get(&key) {
            let cycle_length = cycle - cycle_start;

            if (TARGET - cycle_start) % cycle_length == 0 {
                break;
            }
        }
        seen.insert(key, cycle);

        // The rocks will slide N, W, S, E
        for direction in [Point::NORTH, Point::WEST, Point::SOUTH, Point::EAST] {
            // Let the rocks slide until they stop moving
            loop {
                let mut changed = false;

                for i in 0..platform.round_rocks.len() {
                    let r = platform.round_rocks[i];
                    let next = r + direction;

                    // Check that the next point is available
                    if !platform.bounds.contains(&next) || platform.occupied.contains(&next) {
                        continue;
                    }

                    // If we get here, we can move; do it
                    platform.round_rocks[i].x = next.x;
                    platform.round_rocks[i].y = next.y;

                    platform.occupied.remove(&r);
                    platform.occupied.insert(next);

                    changed = true;
                }

                if !changed {
                    break;
                }
            }
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
