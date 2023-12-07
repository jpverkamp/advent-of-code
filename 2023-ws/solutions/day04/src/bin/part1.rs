use anyhow::Result;
use std::io;

use day04::parse;

// #[aoc_test("data/test/04.txt", "13")]
// #[aoc_test("data/04.txt", "23028")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let (_, cards) = parse::cards(&input).unwrap();

    // Wrapper to avoid calculating 2^(-1) or 2^(usize::MAX)
    fn score(matches: usize) -> usize {
        if matches == 0 {
            0
        } else {
            2_usize.pow((matches - 1) as u32)
        }
    }

    let result = cards
        .iter()
        .map(|card| score(card.matches()))
        .sum::<usize>();

    println!("{result}");
    Ok(())
}
