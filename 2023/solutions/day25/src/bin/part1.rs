use anyhow::Result;
use fxhash::FxHashMap;
use std::io;
use petgraph::{algo::{connected_components, astar}, visit::Dfs};
use itertools::Itertools;

use day25::parse;

// #[aoc_test("data/test/25.txt", "")]
// #[aoc_test("data/25.txt", "")]
fn main() -> Result<()> {
    env_logger::init();
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let graph = parse::read(&input);

    // Count how many times we cross each edge
    let mut counter: FxHashMap<_, usize> = FxHashMap::default();

    // For each pair of nodes, find the shortest path between them
    // Add each edge in that path to the counter
    graph
        .node_indices()
        .take(10)
        .inspect(|node| log::info!("{:?}", node))
        .cartesian_product(graph.node_indices())
        .for_each(|(a, b)|
            astar(
                &graph, 
                a, |node| node == b,
                |_| 1,
                |_| 0
            )
            .unwrap()
            .1
            .iter()
            .tuple_windows()
            .map(|(a, b)| graph.find_edge(*a, *b).unwrap())
            .for_each(|edge| 
                *counter.entry(edge).or_default() += 1
            )
        );
        
    // Sort the list of edges with the heaviest first
    let heavy_edges = counter
        .iter()
        .sorted_by(|(_, a), (_, b)| b.cmp(a))
        .map(|(edge, _)| *edge)
        .collect_vec();

    // Try each combination of 3 edges, starting with the 'heaviest'
    // As soon as we find a trio that splits the graph in 2 we have an answer
    let result = (0..3)
        .map(|_i| heavy_edges.iter())
        .multi_cartesian_product()
        .filter(|edges| edges.iter().unique().count() == 3)
        .inspect(|edges| log::info!("{:?}", edges))
        .find_map(|edges| {
            let mut graph2 = graph.clone();
            for edge in edges {
                graph2.remove_edge(*edge);
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
   
    println!("{result}");
    Ok(())
}
