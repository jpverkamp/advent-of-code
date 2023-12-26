use anyhow::Result;
use std::io;

use day09::parse;

// #[aoc_test("data/test/09.txt", "2")]
// #[aoc_test("data/09.txt", "990")]
fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, equations) = parse::equations(input).unwrap();
    assert_eq!(s.trim(), "");

    Ok(equations
        .iter()
        .map(|equation| {
            // Build the stacks as in part 1, but reverse them all (so we can generate a new 'first' element)
            // Alternatively, use a VecDeque
            let mut stack = equation.stack();
            stack.iter_mut().for_each(|v| v.reverse());

            // Same (from the bottom up), but this time we're subtracting
            for i in (0..stack.len() - 1).rev() {
                let next = stack[i].last().unwrap() - stack[i + 1].last().unwrap();
                stack[i].push(next);
            }

            // The new last value is the value 'before' the original list
            *stack[0].last().unwrap()
        })
        .sum::<i64>()
        .to_string())
}
