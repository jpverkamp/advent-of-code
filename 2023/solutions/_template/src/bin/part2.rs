use anyhow::Result;
use std::io;

// #[aoc_test("data/test/__day__.txt", "")]
// #[aoc_test("data/__day__.txt", "")]
fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let result = input.lines().count();

    Ok(result.to_string())
}
