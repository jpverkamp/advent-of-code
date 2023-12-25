use anyhow::Result;
use fxhash::FxHashMap;
use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use petgraph::graph::DiGraph;
use std::io;

use grid::Grid;
use point::Point;

const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
];

// #[aoc_test("data/test/23.txt", "154")]
// #[aoc_test("data/23.txt", "6226")]
fn main() -> Result<()> {
    env_logger::init();
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;

    let walls = Grid::read(input.as_str(), |c| match c {
        '#' => Some(true),
        _ => None,
    });

    // Find 'points of interest'
    log::info!("Finding splits");
    let mut splits = vec![];

    splits.push(Point::new(1, 0));
    splits.push(Point::new(walls.bounds.max_x - 1, walls.bounds.max_y));

    for y in 0..=walls.bounds.max_y {
        for x in 0..=walls.bounds.max_x {
            let p = Point::new(x, y);

            if walls.get(&p).is_some() {
                continue;
            }

            // Splits are anything with 3 or 4 non-walls
            // Or alternatively, less than 2 walls
            if DIRECTIONS
                .iter()
                .filter(|d| walls.get(&(p + **d)).is_some())
                .count()
                < 2
            {
                splits.push(p);
            }
        }
    }

    // DEBUG
    {
        let mut map = String::from("Map with splits:\n");
        for y in 0..=walls.bounds.max_y {
            for x in 0..=walls.bounds.max_x {
                let p = Point::new(x, y);
                if splits.contains(&p) {
                    map.push('O');
                } else if walls.get(&p).is_some() {
                    map.push('#');
                } else {
                    map.push('.');
                }
            }
            map.push('\n');
        }
        log::info!("{}", map);
    }

    // Calculate distances between splits
    log::info!("Calculating split distances");
    let mut split_distances: FxHashMap<(Point, Point), usize> = FxHashMap::default();

    for split in splits.iter() {
        'found: for direction in DIRECTIONS {
            let mut position = *split + direction;
            let mut distance = 1; // count the first 'direction' step
            let mut path = vec![*split, *split + direction];

            // Make sure the initial move is not out of the map or into a wall
            if !walls.bounds.contains(&position) {
                continue;
            }
            if walls.get(&position).is_some() {
                continue;
            }

            // Keep going until we find the next split in that direction
            'searching: loop {
                // If we found a split, record the distance and move on
                if splits.contains(&position) {
                    split_distances.insert((*split, position), distance);
                    continue 'found;
                }

                distance += 1;

                // Find the one direction (should always be one) we haven't come from
                for direction in DIRECTIONS {
                    let next_position = position + direction;

                    // Don't run into walls
                    if walls.get(&next_position).is_some() {
                        continue;
                    }

                    // And don't backtrack
                    if path.contains(&next_position) {
                        continue;
                    }

                    path.push(next_position);
                    position = next_position;
                    continue 'searching;
                }

                // If we didn't find a direction, this is a dead end
                break 'found;
            }
        }
    }

    for ((src, dst), d) in split_distances.iter() {
        log::info!("- {src:?} -> {dst:?} = {d}", src = src, dst = dst, d = d);
    }

    // Build a petgraph graph from these splits
    let (graph, start, end) = {
        let mut g = DiGraph::new();
        let mut nodes = Vec::new();
        let mut start = None;
        let mut end = None;

        // Add all splits as nodes
        for split in splits.iter() {
            let node = g.add_node(*split);
            nodes.push(node);

            if *split == Point::new(1, 0) {
                start = Some(node);
            }
            if *split == Point::new(walls.bounds.max_x - 1, walls.bounds.max_y) {
                end = Some(node);
            }
        }

        // Add weighted edges between each split that's connected
        for ((src, dst), d) in split_distances.iter() {
            let src = nodes
                .iter()
                .find(|n| *g.node_weight(**n).unwrap() == *src)
                .unwrap();
            let dst = nodes
                .iter()
                .find(|n| *g.node_weight(**n).unwrap() == *dst)
                .unwrap();

            g.add_edge(*src, *dst, *d);
        }

        (g, start.unwrap(), end.unwrap())
    };

    // Get the length of the longest path
    let result = all_simple_paths::<Vec<_>, _>(&graph, start, end, 0, None)
        .map(|path| {
            path.iter()
                .tuple_windows()
                .map(|(a, b)| {
                    split_distances
                        .get(&(
                            *graph.node_weight(*a).unwrap(),
                            *graph.node_weight(*b).unwrap(),
                        ))
                        .unwrap()
                })
                .sum::<usize>()
        })
        .max()
        .unwrap();

    println!("{result}");
    Ok(())
}
