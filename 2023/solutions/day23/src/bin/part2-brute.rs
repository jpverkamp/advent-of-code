use anyhow::Result;
use std::io;

use grid::Grid;
use point::Point;

// #[aoc_test("data/test/23.txt", "154")]
// #[aoc_test("data/23.txt", "")]
fn main() {
    env_logger::init();
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let grid = Grid::read(input, |c| match c {
        '#' => Some(true),
        _ => None,
    });

    #[derive(Debug)]
    struct State {
        position: Point,
        path: Vec<Point>,
    }

    let mut queue = Vec::new();
    queue.push(State {
        position: Point::new(1, 0),
        path: Vec::new(),
    });

    let mut complete = Vec::new();

    let start = std::time::Instant::now();
    let mut count = 0;
    while let Some(state) = queue.pop() {
        count += 1;
        if count % 100_000 == 0 {
            log::info!("{:?} {:?}", count, start.elapsed());
        }

        for direction in &[
            Point::new(0, 1),
            Point::new(0, -1),
            Point::new(1, 0),
            Point::new(-1, 0),
        ] {
            let next_position = state.position + *direction;

            // If we're at the exit, we've found a complete path
            if next_position == Point::new(grid.bounds.max_x - 1, grid.bounds.max_y) {
                complete.push(state.path.clone());
                continue;
            }

            // If we're out of bounds, we've found an invalid path
            if !grid.bounds.contains(&next_position) {
                continue;
            }

            // Cannot go through walls
            if grid.get(&next_position).is_some() {
                continue;
            }

            // Cannot visit the same point more than once
            if state.path.contains(&next_position) {
                continue;
            }

            // Otherwise, queue it up
            let new_state = State {
                position: next_position,
                path: {
                    let mut path = state.path.clone();
                    path.push(next_position);
                    path
                },
            };
            queue.push(new_state);
        }
    }

    // Find the longest path
    // Add 1 to account for leaving the grid
    Ok((1 + complete
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len())
    .to_string())
}
