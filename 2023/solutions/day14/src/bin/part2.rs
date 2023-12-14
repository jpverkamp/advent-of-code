use anyhow::Result;
use fxhash::FxHashMap;
use std::io;

use day14::types::*;

// #[aoc_test("data/test/14.txt", "64")]
// #[aoc_test("data/14.txt", "90982")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let mut platform = PlatformV2::from(Platform::from(input.as_str()));

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

                    // Move in that direction until we hit something (or a wall)
                    let mut next = r;
                    loop {
                        next = next + direction;

                        if !platform.bounds.contains(&next) || platform.occupied.contains(&next) {
                            // Have to step back to the last valid point
                            next = next - direction;
                            break;
                        }
                    }

                    // If we didn't actually move, do nothing
                    if next == r {
                        continue;
                    }

                    // If we get here, we can move; do it
                    platform.round_rocks[i].x = next.x;
                    platform.round_rocks[i].y = next.y;

                    platform.occupied.remove(&r);
                    platform.occupied.insert(next);

                    changed = true;
                    break;
                }

                if !changed {
                    break;
                }
            }
        }
    }

    // Calculate final score
    let result = platform
        .round_rocks
        .iter()
        .map(|r| platform.bounds.max_y - r.y + 1)
        .sum::<isize>();

    println!("{result}");
    Ok(())
}
