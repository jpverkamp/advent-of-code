use anyhow::Result;
use std::io::{self, BufRead};

// #[aoc_test("data/test/00.txt", "")]
// #[aoc_test("data/00.txt", "")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let result = format!("TODO: got {} lines of input", lines.count());

    println!("{result}");
    Ok(())
}
