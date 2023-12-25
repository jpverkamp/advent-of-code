use anyhow::Result;
use std::{io, fs};
use petgraph::dot::Dot;

use day25::parse;

// #[aoc_test("data/test/25.txt", "")]
// #[aoc_test("data/25.txt", "")]
fn main() -> Result<()> {
    env_logger::init();
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let graph = parse::read(&input);

    fs::write("graph.dot", format!("{:?}", Dot::new(&graph)))?;

    Ok(())
}
