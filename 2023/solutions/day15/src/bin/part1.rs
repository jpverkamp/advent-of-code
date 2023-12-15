use anyhow::Result;
use std::io;

// #[aoc_test("data/test/15.txt", "1320")]
// #[aoc_test("data/15.txt", "508552")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;

    fn hash(s: &str) -> u8 {
        s.chars()
            .fold(0, |v, c| ((v.wrapping_add(c as u8)).wrapping_mul(17)))
    }

    let result = input
        .split(',')
        .map(hash)
        .map(|v| v as usize)
        .sum::<usize>();

    println!("{result}");
    Ok(())
}
