use anyhow::Result;
use petgraph::algo::all_simple_paths;
use petgraph::graph::DiGraph;
use std::io;

use day23::types::*;

use grid::Grid;
use point::Point;

// #[aoc_test("data/test/23.txt", "94")]
// #[aoc_test("data/23.txt", "2202")]
fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let grid = Grid::read(input, |c| match c {
        '#' => Some(Object::Wall),
        '^' => Some(Object::Slope(Slope::North)),
        'v' => Some(Object::Slope(Slope::South)),
        '>' => Some(Object::Slope(Slope::East)),
        '<' => Some(Object::Slope(Slope::West)),
        _ => None,
    });

    let (graph, start, end) = {
        let mut g = DiGraph::new();
        let mut nodes = Vec::new();
        let mut start = None;
        let mut end = None;

        // Add each walkable point as a node to the graph
        for y in 0..=grid.bounds.max_y {
            for x in 0..=grid.bounds.max_x {
                let p = Point::new(x, y);

                if let Some(Object::Wall) = grid.get(&p) {
                    continue;
                }

                let node = g.add_node(p);
                nodes.push(node);

                if x == 1 && y == 0 {
                    start = Some(node);
                }
                if x == grid.bounds.max_x - 1 && y == grid.bounds.max_y {
                    end = Some(node);
                }
            }
        }

        // For each created node, add edges between that node and its neighbors
        for node in &nodes {
            let p = *g.node_weight(*node).unwrap();

            for direction in &[
                Point::new(0, 1),
                Point::new(0, -1),
                Point::new(1, 0),
                Point::new(-1, 0),
            ] {
                let next_position = p + *direction;

                // If we're out of bounds, we've found an invalid path
                if !grid.bounds.contains(&next_position) {
                    continue;
                }

                // If we're on a slope, we can only go in the direction of the slope
                if let Some(Object::Slope(s)) = grid.get(&p) {
                    if direction != &Point::from(*s) {
                        continue;
                    }
                }

                // Cannot go through walls
                if let Some(Object::Wall) = grid.get(&next_position) {
                    continue;
                }

                // Otherwise, queue it up
                let next_node = nodes
                    .iter()
                    .find(|n| *g.node_weight(**n).unwrap() == next_position)
                    .unwrap();

                g.add_edge(*node, *next_node, ());
            }
        }

        (g, start.unwrap(), end.unwrap())
    };

    Ok(all_simple_paths::<Vec<_>, _>(&graph, start, end, 0, None)
        .map(|path| path.len() - 1)
        .max()
        .unwrap()
        .to_string())
}
