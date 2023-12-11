use anyhow::Result;
use std::io;

use day__day__::{parse, types::*};

// #[aoc_test("data/test/__day__.txt", "")]
// #[aoc_test("data/__day__.txt", "")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;

    let result = input.lines().count();

    println!("{result}");
    Ok(())
}
