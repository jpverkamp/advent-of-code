use anyhow::Result;
use itertools::Itertools;
use petgraph::{algo::connected_components, visit::Dfs};
use std::io;

use day25::parse;

// #[aoc_test("data/test/25.txt", "")]
// #[aoc_test("data/25.txt", "")]
fn main() {
    env_logger::init();
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let graph = parse::read(input);

    let result = (0..3)
        .map(|_i| graph.edge_indices())
        .multi_cartesian_product()
        .filter(|edges| edges.iter().unique().count() == 3)
        .inspect(|edges| log::info!("{:?}", edges))
        .find_map(|edges| {
            let mut graph2 = graph.clone();
            for edge in edges {
                graph2.remove_edge(edge);
            }

            if connected_components(&graph2) != 2 {
                return None;
            }

            let mut dfs = Dfs::new(&graph2, graph.node_indices().next().unwrap());
            let mut count = 0;
            while dfs.next(&graph2).is_some() {
                count += 1;
            }
            Some(count * (graph.node_count() - count))
        })
        .unwrap();

    Ok(result.to_string())
}
