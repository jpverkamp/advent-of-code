use anyhow::Result;
use std::io::{self, BufRead};

// #[aoc_test("data/test/__day__.txt", "")]
// #[aoc_test("data/__day__.txt", "")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let result = format!("TODO: got {} lines of input", lines.count());

    println!("{result}");
    Ok(())
}
