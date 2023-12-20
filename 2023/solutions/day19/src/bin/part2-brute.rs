use anyhow::Result;
use std::io;

use day19::{parse, types::*};

use itertools::Itertools;

// #[aoc_test("data/test/19.txt", "19114")]
// #[aoc_test("data/19.txt", "476889")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let (s, (rules, _)) = parse::simulation(&input).unwrap();
    assert_eq!(s.trim(), "");

    let values = 1..=4000;
    let start = std::time::Instant::now();

    let result = values
        .clone()
        .cartesian_product(values.clone())
        .inspect(|v| println!("{v:?} in {sec:?}", sec = start.elapsed()))
        .cartesian_product(values.clone())
        .cartesian_product(values.clone())
        .flat_map(|(((x, m), a), s)| {
            let part = Part { x, m, a, s };
            let mut label = Label::Input;

            loop {
                let rule = rules.get(&label).unwrap();
                label = rule.default;

                for comparison in rule.comparisons.iter() {
                    let value = match comparison.category {
                        RatingCategory::X => part.x,
                        RatingCategory::M => part.m,
                        RatingCategory::A => part.a,
                        RatingCategory::S => part.s,
                    };
                    match comparison.comparator {
                        Comparator::LessThan => {
                            if value < comparison.value {
                                label = comparison.label;
                                break;
                            }
                        }
                        Comparator::GreaterThan => {
                            if value > comparison.value {
                                label = comparison.label;
                                break;
                            }
                        }
                    }
                }

                if label == Label::Accept {
                    return Some(part);
                } else if label == Label::Reject {
                    return None;
                }
            }
        })
        .count();

    println!("{result}");
    Ok(())
}
