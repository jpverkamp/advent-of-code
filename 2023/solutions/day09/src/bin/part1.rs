use anyhow::Result;
use std::io;

use day09::parse;

// #[aoc_test("data/test/09.txt", "114")]
// #[aoc_test("data/09.txt", "1887980197")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let (s, equations) = parse::equations(&input).unwrap();
    assert_eq!(s.trim(), "");

    let result = equations
        .iter()
        .map(|equation| {
            // Build a stack of differences until we get to 0
            let mut stack = equation.stack();

            // From the bottom up, add the last value to the differences beneath it
            for i in (0..stack.len() - 1).rev() {
                let next = stack[i].last().unwrap() + stack[i + 1].last().unwrap();
                stack[i].push(next);
            }

            // The new last value of the top line (the original list)
            *stack[0].last().unwrap()
        })
        .sum::<i64>();

    println!("{result}");
    Ok(())
}
