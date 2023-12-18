use anyhow::{Ok, Result};
use std::io;

use day17::types::*;
use grid::Grid;
use point::Point;
use Direction::*;

use pathfinding::prelude::astar;

// #[aoc_test("data/test/17.txt", "94")]
// #[aoc_test("data/test/17b.txt", "71")]
// #[aoc_test("data/17.txt", "930")]
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
                // Must go 4 in a direction before turning
                // Cannot go more than 10 in a direction
                // count == 0 is a special case for the start
                // This count is before the current move
                .filter(|d| {
                    s.count == 0
                        || (s.count < 4 && s.direction == *d)
                        || (s.count >= 4 && s.count <= 10)
                })
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
        // New condition: must have moved at least 4 in a straight line to stop
        |&s| s.position.x == grid.bounds.max_x && s.position.y == grid.bounds.max_y && s.count >= 4,
    );

    // Calculate total score
    if let Some((_path, score)) = result {
        // for y in grid.bounds.min_y..=grid.bounds.max_y {
        //     for x in grid.bounds.min_x..=grid.bounds.max_x {
        //         if let Some(s) = _path.iter().find(|s| s.position.x == x && s.position.y == y) {
        //             match s.direction {
        //                 North => print!("^"),
        //                 South => print!("v"),
        //                 East => print!(">"),
        //                 West => print!("<"),
        //             }
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }

        println!("{score}");
    } else {
        eprintln!("no path found");
    }

    Ok(())
}
