use anyhow::Result;
use std::io;

// aoc_test::generate!{_template_part2_test___day__ as "test/__day__.txt" => ""}
// aoc_test::generate!{_template_part2___day__ as "__day__.txt" => ""}

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
