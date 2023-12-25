use anyhow::Result;
use std::io;
use petgraph::{algo::connected_components, visit::Dfs};
use itertools::Itertools;

use day25::parse;

// #[aoc_test("data/test/25.txt", "")]
// #[aoc_test("data/25.txt", "")]
fn main() -> Result<()> {
    env_logger::init();
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let graph = parse::read(&input);

    // fs::write("graph.dot", format!("{:?}", Dot::new(&graph)))?;

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
            while let Some(_) = dfs.next(&graph2) {
                count += 1;
            }
            Some(count * (graph.node_count() - count))
        })
        .unwrap();

    
    println!("{:?}", result);

    // println!("{result}");
    Ok(())
}
