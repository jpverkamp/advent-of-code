use anyhow::Result;
use std::io;

use day02::parse;

// #[aoc_test("data/test/00.txt", "8")]
// #[aoc_test("data/00.txt", "2061")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let (s, games) = parse::games(&input).unwrap();
    assert_eq!(s.trim(), "");

    let result = games
        .into_iter()
        .filter(|game| {
            game.rounds
                .iter()
                .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
        })
        .map(|game| game.id)
        .sum::<u32>();

    println!("{result}");
    Ok(())
}
