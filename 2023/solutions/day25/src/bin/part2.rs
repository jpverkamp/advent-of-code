use anyhow::Result;
use std::io;

// #[aoc_test("data/test/25.txt", "")]
// #[aoc_test("data/25.txt", "")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let _input = io::read_to_string(stdin.lock())?;

    let result = "Merry Christmas!";

    println!("{result}");
    Ok(())
}
