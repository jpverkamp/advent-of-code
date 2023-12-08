use anyhow::Result;
use std::io;

use day05::{parse, types::*};

// #[aoc_test("data/test/05.txt", "46")]
// #[aoc_test("data/05.txt", "136096660")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let (s, simulation) = parse::simulation(&input).unwrap();
    assert_eq!(s, "");

    // Replace seeds with ranges
    let ranges = simulation
        .seeds
        .chunks(2)
        .map(|lo_hi| lo_hi[0]..=(lo_hi[0] + lo_hi[1]))
        .collect::<Vec<_>>();

    let (cat, values) = simulation.category_maps.iter().fold(
        (Category::Seed, ranges),
        |(cat, values), range_map| {
            assert_eq!(cat, range_map.src_cat);
            (
                range_map.dst_cat,
                values
                    .iter()
                    .flat_map(|r| range_map.apply_range(r.clone()))
                    .collect(),
            )
        },
    );
    assert_eq!(cat, Category::Location);

    assert_eq!(cat, Category::Location);
    let result = values
        .iter()
        .map(|r| r.clone().min().unwrap())
        .min()
        .unwrap();

    println!("{result}");
    Ok(())
}
