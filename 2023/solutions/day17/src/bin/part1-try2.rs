use anyhow::{anyhow, Result};
use std::io;

use day17::types::*;
use grid::Grid;
use point::Point;

// aoc_test::generate!{day17_part1_try2_test_17 as "test/17.txt" => ""}
// aoc_test::generate!{day17_part1_try2_17 as "17.txt" => "796"}
// 796 is too high

fn main() {
    env_logger::init();
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    use Direction::*;
    let grid = Grid::read(input, |c| c.to_digit(10));

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
    struct State {
        score: u32,
        direction: Direction,
        count: u32,
    }

    let mut best = Grid::default();
    best.insert(
        Point::new(grid.bounds.max_x, grid.bounds.max_y),
        State {
            score: 0,
            direction: East,
            count: 0,
        },
    );

    // TODO: Figure out the 3 in a row thing

    // Until we've stabilized, update any best new paths
    // loop {
    for _i in 0..10 {
        let mut updated = false;

        for x in (grid.bounds.min_x..=grid.bounds.max_x).rev() {
            for y in (grid.bounds.min_y..=grid.bounds.max_y).rev() {
                let p_current = Point::new(x, y);
                let mut current_best = None;

                for d in &[North, South, East, West] {
                    let p_next = p_current + Point::from(*d);
                    // log::info!("{p_current} {d:?} {p_next}");

                    // Moving to square that doesn't have a solution yet
                    // This includes moving out of bounds
                    if best.get(&p_next).is_none() {
                        // log::info!("- no solution yet");
                        continue;
                    }

                    // Get the potential new score
                    let next = *best.get(&p_next).unwrap();
                    let new = State {
                        score: next.score + grid.get(&p_next).unwrap(),
                        direction: *d,
                        count: if *d == next.direction {
                            next.count + 1
                        } else {
                            1
                        },
                    };

                    // Moving 3 in the same direction
                    if new.count > 3 {
                        // log::info!("- 3 in a row");
                        continue;
                    }

                    // Score isn't better
                    if current_best.is_some_and(|b: State| b.score <= new.score) {
                        // log::info!("- score not better {} <= {}", current_best.unwrap().score, new.score);
                        continue;
                    }

                    // log::info!("  - new best: {:?}", new);

                    // Found a new best, update
                    best.insert(p_current, new);
                    current_best = Some(new);
                    updated = true;
                }
            }
        }

        log::info!(
            "{:?}",
            best.get(&Point::ORIGIN).unwrap().score - grid.get(&Point::ORIGIN).unwrap()
        );

        log::info!(
            "{}",
            &best.to_string('.', |state| match state.direction {
                North => '^',
                South => 'v',
                East => '>',
                West => '<',
            })
        );

        if !updated {
            break;
        }
    }

    log::info!(
        "{:?}",
        best.get(&Point::ORIGIN).unwrap().score - grid.get(&Point::ORIGIN).unwrap()
    );

    log::info!(
        "{}",
        &best.to_string('.', |state| match state.direction {
            North => '^',
            South => 'v',
            East => '>',
            West => '<',
        })
    );

    Err(anyhow!("Not finished"))
}
