use anyhow::Result;
use std::io;

use day05::{parse, types::*};

aoc_test::generate!{day05_part2_brute_test_05 as "test/05.txt" => "46"}
// aoc_test::generate!{day05_part2_brute_05 as "05.txt" => "136096660"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, mut simulation) = parse::simulation(input).unwrap();
    assert_eq!(s, "");

    // Replace seeds with ranges
    simulation.seeds = simulation
        .seeds
        .chunks(2)
        .flat_map(|lo_hi| (lo_hi[0]..=(lo_hi[0] + lo_hi[1])).collect::<Vec<_>>())
        .collect::<Vec<_>>();

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
