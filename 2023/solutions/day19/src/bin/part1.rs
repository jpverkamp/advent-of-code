use anyhow::Result;
use std::io;

use day19::{parse, types::*};

// #[aoc_test("data/test/19.txt", "19114")]
// #[aoc_test("data/19.txt", "476889")]
fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, (rules, parts)) = parse::simulation(input).unwrap();
    assert_eq!(s.trim(), "");

    Ok(parts
        .iter()
        .filter_map(|part| {
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
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<u64>()
        .to_string())
}
