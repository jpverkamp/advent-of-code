use anyhow::Result;
use std::io;

use day19::{parse, types::*};
use log::info;

// #[aoc_test("data/test/19.txt", "167409079868000")]
// #[aoc_test("data/19.txt", "132380153677887")]
fn main() {
    env_logger::init();

    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, (rules, _)) = parse::simulation(input).unwrap();
    assert_eq!(s.trim(), "");

    #[derive(Debug, Clone)]
    struct State<'a> {
        label: Label<'a>,
        part: RangedPart,
    }

    let mut queue = vec![State {
        label: Label::Input,
        part: RangedPart {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        },
    }];
    let mut accepted = Vec::new();

    while let Some(state) = queue.pop() {
        info!("===== =====");
        info!("{:?}", state);

        // If we're in accept/reject, process specifically
        if state.label == Label::Accept {
            info!("- accepting");
            accepted.push(state.part);
            continue;
        } else if state.label == Label::Reject {
            info!("- rejecting");
            continue;
        }

        let rule = rules.get(&state.label).unwrap();

        // 'Remaining' is ranges that haven't had any comparison applied to them
        // Anything that makes it all the way through will be defaulted
        let mut remaining = vec![state];

        // Apply comparisons to all remaining ranges
        for comparison in rule.comparisons.iter() {
            info!("  - comparison: {:?}", comparison);
            info!("   - remaining: {:?}", remaining);

            // Update remaining
            // Any parts that are moved to queue will filter_map to None (and be removed)
            // Anything else will Some and be kept for the next comparison
            remaining = remaining
                .into_iter()
                .filter_map(|state| {
                    // Get the relevant range
                    let value_range = match comparison.category {
                        RatingCategory::X => state.part.x.clone(),
                        RatingCategory::M => state.part.m.clone(),
                        RatingCategory::A => state.part.a.clone(),
                        RatingCategory::S => state.part.s.clone(),
                    };

                    // Apply the comparison to the range, possibly splitting
                    // There are three cases: less than, during, and greater than the range
                    if comparison.value < *value_range.start() {
                        info!("    - all values are greater than value");
                        match comparison.comparator {
                            Comparator::GreaterThan => {
                                info!("    - pushing to queue with label={:?}", comparison.label);
                                queue.push(State {
                                    label: comparison.label,
                                    part: state.part,
                                });
                                None
                            }
                            Comparator::LessThan => {
                                info!("    - remaining");
                                Some(state)
                            }
                        }
                    } else if comparison.value > *value_range.end() {
                        info!("    - all values are less than value");
                        match comparison.comparator {
                            Comparator::GreaterThan => {
                                info!("    - remaining");
                                Some(state)
                            }
                            Comparator::LessThan => {
                                info!("    - pushing to queue with label={:?}", comparison.label);
                                queue.push(State {
                                    label: comparison.label,
                                    part: state.part,
                                });
                                None
                            }
                        }
                    } else {
                        info!("    - value is in range, splitting");
                        match comparison.comparator {
                            Comparator::GreaterThan => {
                                // Comparison is less than, so the value goes with the upper half
                                // Lower half goes to queue, upper half stays in remaining
                                let lo_range = (*value_range.start()..=comparison.value).clone();
                                let hi_range =
                                    ((comparison.value + 1)..=*value_range.end()).clone();

                                let mut lo = state.clone().part;
                                match comparison.category {
                                    RatingCategory::X => lo.x = lo_range,
                                    RatingCategory::M => lo.m = lo_range,
                                    RatingCategory::A => lo.a = lo_range,
                                    RatingCategory::S => lo.s = lo_range,
                                }

                                let mut hi = state.clone().part.clone();
                                match comparison.category {
                                    RatingCategory::X => hi.x = hi_range,
                                    RatingCategory::M => hi.m = hi_range,
                                    RatingCategory::A => hi.a = hi_range,
                                    RatingCategory::S => hi.s = hi_range,
                                }

                                info!("     - lo ({:?}) is is remaining", lo);
                                info!(
                                    "     - hi ({:?}) is pushing to queue with label={:?}",
                                    hi, comparison.label
                                );

                                queue.push(State {
                                    label: comparison.label,
                                    part: hi,
                                });

                                Some(State {
                                    label: state.label,
                                    part: lo,
                                })
                            }
                            Comparator::LessThan => {
                                // Comparison is greater than, so value goes with the lower half
                                // Lower half stays in remaining, upper half goes to queue
                                let lo_range =
                                    (*value_range.start()..=(comparison.value - 1)).clone();
                                let hi_range = (comparison.value..=*value_range.end()).clone();

                                let mut lo = state.part.clone();
                                match comparison.category {
                                    RatingCategory::X => lo.x = lo_range,
                                    RatingCategory::M => lo.m = lo_range,
                                    RatingCategory::A => lo.a = lo_range,
                                    RatingCategory::S => lo.s = lo_range,
                                }

                                let mut hi = state.part.clone();
                                match comparison.category {
                                    RatingCategory::X => hi.x = hi_range,
                                    RatingCategory::M => hi.m = hi_range,
                                    RatingCategory::A => hi.a = hi_range,
                                    RatingCategory::S => hi.s = hi_range,
                                }

                                info!(
                                    "     - lo ({:?}) is pushing to queue with label={:?}",
                                    lo, comparison.label
                                );
                                info!("     - hi ({:?}) is is remaining", hi);

                                queue.push(State {
                                    label: comparison.label,
                                    part: lo,
                                });

                                Some(State {
                                    label: state.label,
                                    part: hi,
                                })
                            }
                        }
                    }
                })
                .collect();
        }

        // Anything still remaining gets the default label
        remaining.iter_mut().for_each(|state| {
            state.label = rule.default;
            info!("  - defaulting: {:?}", state);
            queue.push(state.clone());
        });
    }

    Ok(accepted
        .iter()
        .map(|part| {
            (part.x.end() - part.x.start() + 1) as u128
                * (part.m.end() - part.m.start() + 1) as u128
                * (part.a.end() - part.a.start() + 1) as u128
                * (part.s.end() - part.s.start() + 1) as u128
        })
        .sum::<u128>()
        .to_string())
}
