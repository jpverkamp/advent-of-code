use anyhow::Result;
use std::io;

use day13::types::*;

// Find the first reflection
// on_x if reflecting about the x axis, false otherwise
// if ignore is set, don't return this axis
fn reflect(ashflow: &AshFlow, on_x: bool, ignore: Option<isize>) -> Option<isize> {
    let axis_range = if on_x {
        ashflow.bounds.min_x..ashflow.bounds.max_x
    } else {
        ashflow.bounds.min_y..ashflow.bounds.max_y
    };

    for axis in axis_range {
        if ignore == Some(axis) {
            continue;
        }

        if ashflow.rocks.iter().all(|p| {
            let pr = if on_x {
                p.reflect_x(axis)
            } else {
                p.reflect_y(axis)
            };
            !ashflow.bounds.contains(&pr) || ashflow.rocks.contains(&pr)
        }) {
            return Some(axis);
        }
    }

    None
}

// Used to smudge either way
fn toggle(ashflow: &mut AshFlow, p: &Point) {
    if ashflow.rocks.contains(p) {
        ashflow.rocks.remove(p);
    } else {
        ashflow.rocks.insert(*p);
    }
}

// #[aoc_test("data/test/13.txt", "400")]
// #[aoc_test("data/13.txt", "36771")]
fn main() {
    let stdin = io::stdin();

    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let result = input
        .split("\n\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|input| {
            let mut ashflow = AshFlow::from(*input);
            let mut result = 0;

            // TODO: Is it possible to have more than one mirror?
            // in the input cases, no

            // Calculate the old axis of reflection (to ignore)
            let old_x = reflect(&ashflow, true, None);
            let old_y = reflect(&ashflow, false, None);

            'found: for x_smudge in ashflow.bounds.min_x..=ashflow.bounds.max_x {
                for y_smudge in ashflow.bounds.min_y..=ashflow.bounds.max_y {
                    let p_smudge = Point {
                        x: x_smudge,
                        y: y_smudge,
                    };
                    toggle(&mut ashflow, &p_smudge);

                    // If we got a new x (or later a y) ignoring the one we already saw
                    // This is our solution, score it and stop looking

                    if let Some(new_x) = reflect(&ashflow, true, old_x) {
                        result += new_x + 1;
                        break 'found;
                    }

                    if let Some(new_y) = reflect(&ashflow, false, old_y) {
                        result += 100 * (new_y + 1);
                        break 'found;
                    }

                    toggle(&mut ashflow, &p_smudge);
                }
            }

            result
        })
        .sum::<isize>();

    Ok(result.to_string())
}
