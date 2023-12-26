use petgraph::dot::Dot;
use std::{fs, io};

use day25::parse;

fn main() {
    env_logger::init();
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let graph = parse::read(&input);

    fs::write("graph.dot", format!("{:?}", Dot::new(&graph))).expect("write graph.dot");
}
