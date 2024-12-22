use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::astar;

use crate::{Direction, Point};

const KEYPAD: &[char] = &['7', '8', '9', '4', '5', '6', '1', '2', '3', '*', '0', 'A'];
const ARROWS: &[char] = &['*', '^', 'A', '<', 'V', '>'];
const INPUT: &[char] = &['^', '<', 'V', 'A', '>'];

#[aoc_generator(day21)]
fn parse(input: &str) -> String {
    input.to_string()
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    points: [Point; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Day21Error {
    InvalidDirection,
    OutOfBounds,
    InvalidPosition,
}


impl State {
    fn new() -> State {
        State {
            points: [Point::new(2, 0), Point::new(2, 0), Point::new(2, 3)],
        }
    }

    #[tracing::instrument]
    fn hit(&self, c: char) -> Result<(State, Option<char>), Day21Error> {
        let mut new_points = self.points.clone();

        let mut c = c;
        for i in 0..3 {
            tracing::debug!("In level {i}, c={c}");

            let keys = if i == 2 { KEYPAD } else { ARROWS };
            let height = if i == 2 { 4 } else { 2 };
            let index = (new_points[i].y * 3 + new_points[i].x) as usize;

            if c == 'A' {
                // Type that character on the next level
                c = keys[index];
            } else {
                // Try to move this layer
                let d: Direction = c.try_into().map_err(|_| Day21Error::InvalidDirection)?;
                let new_point = new_points[i] + d;

                // Out of bounds
                if new_point.x < 0
                    || new_point.x >= 3
                    || new_point.y < 0
                    || new_point.y >= height
                {
                    return Err(Day21Error::OutOfBounds);
                }

                // Moved to an invalid character
                let new_index = (new_point.y * 3 + new_point.x) as usize;
                if keys[new_index] == '*' {
                    return Err(Day21Error::InvalidPosition);
                }

                // Otherwise, we're done without outputting a character
                new_points[i] = new_point;
                return Ok((State { points: new_points }, None));
            }
        }

        // If we made it out of the list, we typed whatever is left in c
        Ok((State { points: new_points }, Some(c)))
    }
}

#[allow(dead_code)]
fn debug() {
    tracing_subscriber::fmt::init();

    let mut s = State::new();
    tracing::info!("Initial state: {s:?}");

    for c in "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".chars() {
        match s.hit(c) {
            Ok((new_s, Some(c))) => {
                tracing::info!("Typed a {c}");
                s = new_s;
            },
            Ok((new_s, None)) => {
                tracing::debug!("Moved");
                s = new_s;
            }
            Err(e) => {
                tracing::debug!("Invalid move: {:?}", e);
            },
        }
    }

    tracing::info!("Final state: {s:?}");
}

#[aoc(day21, part1, v1)]
fn part1_v1(input: &str) -> usize {
    let mut total_score = 0;

    for line in input.lines() {
        match astar(
            &(State::new(), line),
            |(state, to_type)| {
                let to_type = *to_type;
                let mut result = Vec::new();

                for c in INPUT {
                    match state.hit(*c) {
                        // Hit a character, didn't output anything
                        Ok((new_state, None)) => {
                            result.push(((new_state, to_type), 1));
                        }
                        // Output something
                        Ok((new_state, Some(output))) => {
                            if to_type.starts_with(output) {
                                // This is the next character we're looking for, valid branch
                                result.push(((new_state, &to_type[1..]), 1));
                            } else {
                                // We typed the wrong character, this is an invalid branch
                            }
                        }
                        Err(_) => {
                            // One of the layers hit an invalid character or ran out of bounds
                        }
                    }
                }

                result
            },
            |(_state, to_type)| to_type.len(),
            |(_state, to_type)| to_type.is_empty(),
        ) {
            Some((_path, cost)) => {
                let mut line_value = 0;
                for c in line.chars() {
                    if c.is_ascii_digit() {
                        line_value *= 10;
                        line_value += c.to_digit(10).unwrap() as usize;
                    }
                }

                total_score += cost * line_value;
            }
            None => {
                println!("No path found");
                // No path found
            }
        }
    }

    total_score
}

#[aoc(day21, part2, v1)]
fn part2_v1(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
029A
980A
179A
456A
379A";

    make_test!([part1_v1] => "day21.txt", 126384, "final output");
    make_test!([part2_v1] => "day21.txt", "example output", "final output");
}
