use anyhow::Result;
use std::io;

use day07::parse;

// #[aoc_test("data/test/07.txt", "5905")]
// #[aoc_test("data/07.txt", "253907829")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let input = input.replace('J', "*");
    let (s, mut hands) = parse::hands(&input).unwrap();
    assert_eq!(s.trim(), "");

    hands.sort();

    let result = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid as usize)
        .sum::<usize>();

    println!("{result}");
    Ok(())
}
