use anyhow::Result;
use std::io;

use day05::{parse, types::*};

// #[aoc_test("data/test/05.txt", "35")]
// #[aoc_test("data/05.txt", "825516882")]
fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, simulation) = parse::simulation(input).unwrap();
    assert_eq!(s, "");

    let (cat, values) = simulation.category_maps.iter().fold(
        (Category::Seed, simulation.seeds),
        |(cat, values), range_map| {
            assert_eq!(cat, range_map.src_cat);
            (
                range_map.dst_cat,
                values.iter().map(|x| range_map.apply(*x)).collect(),
            )
        },
    );
    assert_eq!(cat, Category::Location);

    Ok(values.iter().min().unwrap().to_string())
}
