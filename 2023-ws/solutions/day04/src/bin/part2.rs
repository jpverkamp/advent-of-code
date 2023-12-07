use anyhow::Result;
use std::io;

use day04::parse;

// #[aoc_test("data/test/04.txt", "30")]
// #[aoc_test("data/04.txt", "9236992")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let (_, cards) = parse::cards(&input).unwrap();

    let mut result = 0;
    let mut counts = vec![1; cards.len()];
    let mut next_counts = vec![0; cards.len()];

    // Earn new cards until stable
    loop {
        // Count all cards earned before updating
        result += counts.iter().sum::<usize>();

        // Each card earns
        // NOTE: We're explicitly guaranteed that next_counts[i + j + 1] doesn't overflow
        for (i, card) in cards.iter().enumerate() {
            for j in 0..card.matches() {
                next_counts[i + j + 1] += counts[i];
            }
        }

        // If no cards were earned, we're done
        if next_counts.iter().all(|&c| c == 0) {
            break;
        }

        // Swap buffers and clear
        // This could be a std::mem::swap, but we'd still need to init the new next_counts
        for i in 0..cards.len() {
            counts[i] = next_counts[i];
            next_counts[i] = 0;
        }
    }

    println!("{result}");
    Ok(())
}
