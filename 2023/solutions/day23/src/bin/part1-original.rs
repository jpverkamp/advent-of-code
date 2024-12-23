use anyhow::Result;
use std::io;

use day23::types::*;

use grid::Grid;
use point::Point;

aoc_test::generate!{day23_part1_original_test_23 as "test/23.txt" => "94"}
aoc_test::generate!{day23_part1_original_23 as "23.txt" => "2202"}

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

    #[derive(Debug)]
    struct State {
        position: Point,
        path: Vec<Point>,
    }

    let mut queue = Vec::new();
    queue.push(State {
        position: Point::new(1, 0),
        path: Vec::with_capacity(1024),
    });

    let mut complete = Vec::new();

    while let Some(state) = queue.pop() {
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

            // If we're on a slope, we can only go in the direction of the slope
            if let Some(Object::Slope(s)) = grid.get(&state.position) {
                if direction != &Point::from(*s) {
                    continue;
                }
            }

            // Cannot go through walls
            if let Some(Object::Wall) = grid.get(&next_position) {
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
