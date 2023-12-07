use anyhow::Result;
use std::io;

use day03::types::*;

// #[aoc_test("data/test/__day__.txt", "467835")]
// #[aoc_test("data/__day__.txt", "81166799")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let schematic = Schematic::from(input);

    let result = schematic
        .symbols
        .iter()
        .filter(|s| s.value == '*')
        .map(|s| {
            schematic
                .numbers
                .iter()
                .filter_map(|n| {
                    if n.is_neighbor(s.x, s.y) {
                        Some(n.value)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .filter(|ratios| ratios.len() == 2)
        .map(|ratios| ratios[0] * ratios[1])
        .sum::<usize>();

    println!("{result}");
    Ok(())
}
