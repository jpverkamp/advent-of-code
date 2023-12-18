use anyhow::{Ok, Result};
use std::io;

use day17::types::*;
use grid::Grid;
use point::Point;
use Direction::*;

use pathfinding::prelude::astar;

// #[aoc_test("data/test/17.txt", "102")]
// #[aoc_test("data/17.txt", "771")]
// 796 is too high
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;

    let grid = Grid::read(input.as_str(), |c| c.to_digit(10));

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
    struct State {
        position: Point,
        direction: Direction,
        count: u32,
    }

    let result = astar(
        &State {
            position: Point::new(0, 0),
            direction: South,
            count: 0,
        },
        // successor function
        |&s| {
            [s.direction.left(), s.direction, s.direction.right()]
                .into_iter()
                // Next point must be in bounds
                .filter(|d| grid.bounds.contains(&(s.position + Point::from(*d))))
                // Can't go more than 3 in the same direction
                .filter(|d| s.count < 3 || s.direction != *d)
                // Generate the next state for each neighbor
                .map(|d| State {
                    position: s.position + Point::from(d),
                    direction: d,
                    count: if s.direction == d { s.count + 1 } else { 1 },
                })
                // Add score for each node moved
                .map(|s| (s, *grid.get(&s.position).unwrap()))
                .collect::<Vec<_>>()
        },
        // heuristic function
        |&s| {
            s.position
                .manhattan_distance(&Point::new(grid.bounds.max_x, grid.bounds.max_y))
                as u32
        },
        // goal function
        |&s| s.position.x == grid.bounds.max_x && s.position.y == grid.bounds.max_y,
    );

    // Calculate total score
    if let Some((_path, score)) = result {
        println!("{score}");
    } else {
        eprintln!("no path found");
    }

    Ok(())
}
