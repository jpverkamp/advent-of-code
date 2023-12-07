use anyhow::Result;
use std::io;

use day03::types::*;

// #[aoc_test("data/test/__day__.txt", "4361")]
// #[aoc_test("data/__day__.txt", "549908")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let schematic = Schematic::from(input);

    let result = schematic
        .numbers
        .iter()
        .filter(|n| schematic.symbols.iter().any(|s| n.is_neighbor(s.x, s.y)))
        .map(|n| n.value)
        .sum::<usize>();

    println!("{result}");
    Ok(())
}
