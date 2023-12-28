use anyhow::Result;
use fxhash::FxHashMap;
use std::io;

use grid::Grid;
use point::Point;

const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
];

aoc_test::generate!{day23_part2_splits_test_23 as "test/23.txt" => "154"}
// aoc_test::generate!{day23_part2_splits_23 as "23.txt" => "6226"}

fn main() {
    env_logger::init();
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let walls = Grid::read(input, |c| match c {
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

    // Now search for the longest path using splits
    #[derive(Debug)]
    struct State {
        position: Point,
        path: Vec<Point>,
        distance: usize,
    }

    let mut queue = Vec::new();
    queue.push(State {
        position: Point::new(1, 0),
        path: Vec::new(),
        distance: 0,
    });

    let mut complete = Vec::new();

    log::info!("Searching for longest path");
    let start = std::time::Instant::now();
    let mut count = 0;
    while let Some(state) = queue.pop() {
        count += 1;
        if count % 1_000_000 == 0 {
            log::info!("- {:?} paths examined in {:?}", count, start.elapsed());
        }

        // Which nodes can we go to next?
        let nexts = splits.iter().filter_map(|dst| {
            split_distances
                .get(&(state.position, *dst))
                .map(|distance| (*dst, *distance))
        });

        for (next, distance) in nexts {
            // If we're at the exit, we've found a complete path
            if next == Point::new(walls.bounds.max_x - 1, walls.bounds.max_y) {
                complete.push((state.path.clone(), state.distance + distance));
                continue;
            }

            // If we've already hit this split, we've found a loop
            if state.path.contains(&next) {
                continue;
            }

            // Otherwise, queue it up
            let new_state = State {
                position: next,
                path: {
                    let mut path = state.path.clone();
                    path.push(next);
                    path
                },
                distance: state.distance + distance,
            };
            queue.push(new_state);
        }
    }

    Ok(complete.iter().map(|(_, d)| d).max().unwrap().to_string())
}
