use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use itertools::Itertools;
use pathfinding::prelude::astar;

use crate::{Direction, Point};

const KEYPAD: &[char] = &['7', '8', '9', '4', '5', '6', '1', '2', '3', '*', '0', 'A'];
const ARROWS: &[char] = &['*', '^', 'A', '<', 'v', '>'];
const INPUT: &[char] = &['^', '<', 'v', 'A', '>'];

fn line_multiplier(line: &str) -> usize {
    line.chars()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap() as usize)
}

#[aoc_generator(day21)]
fn parse(input: &str) -> String {
    input.to_string()
}

// region: Original solution with direct simulation

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    points: Vec<Point>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Day21Error {
    InvalidDirection,
    OutOfBounds,
    InvalidPosition,
}

impl State {
    fn new(arrow_robots: usize) -> State {
        let mut points = Vec::new();

        for _ in 0..arrow_robots {
            points.push(Point::new(2, 0));
        }
        points.push(Point::new(2, 3));

        State { points }
    }

    #[tracing::instrument]
    fn hit(&self, c: char) -> Result<(State, Option<char>), Day21Error> {
        let mut new_points = self.points.clone();

        let mut c = c;
        for i in 0..new_points.len() {
            tracing::debug!("In level {i}, c={c}");

            let keys = if i == new_points.len() - 1 {
                KEYPAD
            } else {
                ARROWS
            };
            let height = if i == new_points.len() - 1 { 4 } else { 2 };
            let index = (new_points[i].y * 3 + new_points[i].x) as usize;

            if c == 'A' {
                // Type that character on the next level
                c = keys[index];
            } else {
                // Try to move this layer
                let d: Direction = c.try_into().map_err(|_| Day21Error::InvalidDirection)?;
                let new_point = new_points[i] + d;

                // Out of bounds
                if new_point.x < 0 || new_point.x >= 3 || new_point.y < 0 || new_point.y >= height {
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

    let mut s = State::new(2);
    tracing::info!("Initial state: {s:?}");

    for c in "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".chars() {
        match s.hit(c) {
            Ok((new_s, Some(c))) => {
                tracing::info!("Typed a {c}");
                s = new_s;
            }
            Ok((new_s, None)) => {
                tracing::debug!("Moved");
                s = new_s;
            }
            Err(e) => {
                tracing::debug!("Invalid move: {:?}", e);
            }
        }
    }

    tracing::info!("Final state: {s:?}");
}

#[aoc(day21, part1, sim)]
fn part1_sim(input: &str) -> usize {
    let mut total_score = 0;

    for line in input.lines() {
        match astar(
            &(State::new(2), line),
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
                total_score += cost * line_multiplier(line);
            }
            None => {
                println!("No path found");
                // No path found
            }
        }
    }

    total_score
}

// endregion

// region: Recursive solution

// For each numpad key, where is it?
#[tracing::instrument(ret)]
fn keypad_position(key: char) -> (usize, usize) {
    match key {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        // missing bottom left key
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!("Invalid key"),
    }
}

// Generate sequences of ^v<>A that will move from src to dst on the keypad
#[tracing::instrument(ret)]
fn keypad_paths(src: char, dst: char) -> Vec<String> {
    // Convert to points
    let (x1, y1) = keypad_position(src);
    let (x2, y2) = keypad_position(dst);

    // If we're move left/up, use <^; otherwise >V (deal with zero later)
    let h_char = if x1 < x2 { '>' } else { '<' };
    let v_char = if y1 < y2 { 'v' } else { '^' };

    let h_delta = (x2 as isize - x1 as isize).abs();
    let h_string = std::iter::repeat_n(h_char, h_delta as usize).collect::<String>();

    let v_delta = (y2 as isize - y1 as isize).abs();
    let v_string = std::iter::repeat_n(v_char, v_delta as usize).collect::<String>();

    // If we only have one of the two, then our path is simple :smile:
    // (This avoids duplicate paths in the _ case below)
    if h_delta == 0 || v_delta == 0 {
        return vec![format!("{h_string}{v_string}A")];
    }

    match (x1, y1, x2, y2) {
        // Moving from the bottom to the left
        // Avoid the missing square by going up first
        (_, 3, 0, _) => vec![format!("{v_string}{h_string}A")],
        // Moving from the left to the bottom
        // Avoid the missing square by going right first
        (0, _, _, 3) => vec![format!("{h_string}{v_string}A")],
        // Otherwise, try both
        _ => {
            let vh = format!("{v_string}{h_string}A");
            let hv = format!("{h_string}{v_string}A");
            vec![vh, hv]
        }
    }
}

// Generate sequences of movements on the arrow keys
#[tracing::instrument(ret)]
fn arrows_paths(src: char, dst: char) -> Vec<String> {
    // For any square one away, go directly
    // For any two away, return both options
    match (src, dst) {
        ('^', 'A') => vec![">A"],
        ('^', '<') => vec!["v<A"],
        ('^', 'v') => vec!["vA"],
        ('^', '>') => vec![">vA", "v>A"],

        ('A', '^') => vec!["<A"],
        ('A', '<') => vec!["v<<A"],
        ('A', 'v') => vec!["<vA", "v<A"],
        ('A', '>') => vec!["vA"],

        ('<', '^') => vec![">^A"],
        ('<', 'A') => vec![">>^A"],
        ('<', 'v') => vec![">A"],
        ('<', '>') => vec![">>A"],

        ('v', '^') => vec!["^A"],
        ('v', 'A') => vec!["^>A", ">^A"],
        ('v', '<') => vec!["<A"],
        ('v', '>') => vec![">A"],

        ('>', '^') => vec!["<^A", "^<A"],
        ('>', 'A') => vec!["^A"],
        ('>', '<') => vec!["<<A"],
        ('>', 'v') => vec!["<A"],

        // I had a heck of a time debugging in here... v =/= V
        (a, b) if a == b => vec!["A"],
        (a, b) => panic!("Bad encoding for {a} -> {b}"),
    }
    .iter()
    .map(|&s| s.to_owned())
    .collect()
}

// To move level 0 from (x, y) to (x + xd, y + yd), what do we need to do at this level?
#[tracing::instrument(ret)]
fn arrows_cost(input: &str, arrow_bots: usize) -> usize {
    // If we don't have any more arrow bots, it's easy :smile:
    if arrow_bots == 0 {
        return input.len();
    }

    // Otherwise, assume we're starting at A
    // For each pair of characters, find the minimum path between them with one less bot
    format!("A{}", input)
        .chars()
        .tuple_windows()
        .map(|(src, dst)| {
            tracing::info!("keypad cost mapping src={src}, dst={dst}");
            arrows_paths(src, dst)
                .iter()
                .map(|path| arrows_cost(path, arrow_bots - 1))
                .min()
                .unwrap()
        })
        .sum()
}

// To move a given sequence of characters with that many arrow bots
#[tracing::instrument(ret)]
fn keypad_cost(input: &str, arrow_bots: usize) -> usize {
    // Assume we're starting at A
    // For each pair of characters, find the minimum path between them recursively
    format!("A{}", input)
        .chars()
        .tuple_windows()
        .map(|(src, dst)| {
            tracing::info!(
                "arrows cost outer map src={src}, dst={dst} with arrow_bots={arrow_bots}"
            );
            keypad_paths(src, dst)
                .iter()
                .map(|path| {
                    tracing::info!(
                        "arrow costs inner map path={path} with arrow_bots={arrow_bots}"
                    );
                    if arrow_bots == 0 {
                        path.len()
                    } else {
                        arrows_cost(path, arrow_bots)
                    }
                })
                .min()
                .unwrap()
        })
        .sum()
}

#[aoc(day21, part1, recur)]
fn part1_recur(input: &str) -> usize {
    input
        .lines()
        .map(|line| line_multiplier(line) * keypad_cost(line, 2))
        .sum()
}

// endregion

// region: Recursive solution with memoization

type CacheType = HashMap<(String, usize), usize>;

// To move level 0 from (x, y) to (x + xd, y + yd), what do we need to do at this level?
// This function is the one we actually memoize
#[tracing::instrument(ret)]
fn arrows_cost_memo(cache: &mut CacheType, input: &str, arrow_bots: usize) -> usize {
    // If we don't have any more arrow bots, it's easy :smile:
    if arrow_bots == 0 {
        return input.len();
    }

    // Already cached
    // NOTE: This is expensive, since I'm cloning a ton of strings and hashing
    //       But when the alternative is branching trillions of times...
    let cache_key = (input.to_owned(), arrow_bots);
    if let Some(&value) = cache.get(&cache_key) {
        return value;
    }

    // Otherwise, assume we're starting at A
    // For each pair of characters, find the minimum path between them with one less bot
    let result = format!("A{}", input)
        .chars()
        .tuple_windows()
        .map(|(src, dst)| {
            tracing::info!("keypad cost mapping src={src}, dst={dst}");
            arrows_paths(src, dst)
                .iter()
                .map(|path| arrows_cost_memo(cache, path, arrow_bots - 1))
                .min()
                .unwrap()
        })
        .sum();

    cache.insert(cache_key, result);
    result
}

// To move a given sequence of characters with that many arrow bots
// This function exists entirely to call arrows_cost_memo with the cache
#[tracing::instrument(ret)]
fn keypad_cost_memo(cache: &mut CacheType, input: &str, arrow_bots: usize) -> usize {
    // Assume we're starting at A
    // For each pair of characters, find the minimum path between them recursively
    format!("A{}", input)
        .chars()
        .tuple_windows()
        .map(|(src, dst)| {
            tracing::info!(
                "arrows cost outer map src={src}, dst={dst} with arrow_bots={arrow_bots}"
            );
            keypad_paths(src, dst)
                .iter()
                .map(|path| {
                    tracing::info!(
                        "arrow costs inner map path={path} with arrow_bots={arrow_bots}"
                    );
                    if arrow_bots == 0 {
                        path.len()
                    } else {
                        arrows_cost_memo(cache, path, arrow_bots)
                    }
                })
                .min()
                .unwrap()
        })
        .sum()
}

#[aoc(day21, part1, memo)]
fn part1_recur_memo(input: &str) -> usize {
    let mut cache = CacheType::new();

    input
        .lines()
        .map(|line| line_multiplier(line) * keypad_cost_memo(&mut cache, line, 2))
        .sum()
}

#[aoc(day21, part2, memo)]
fn part2_recur_memo(input: &str) -> usize {
    let mut cache = CacheType::new();

    input
        .lines()
        .map(|line| line_multiplier(line) * keypad_cost_memo(&mut cache, line, 25))
        .sum()
}

// endregion

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

    make_test!([part1_sim, part1_recur, part1_recur_memo] => "day21.txt", 126384, 205160);
    make_test!([part2_recur_memo] => "day21.txt", "154115708116294", "252473394928452");

    #[test]
    fn test_keypad_cost_0() {
        assert_eq!(keypad_cost("029A", 0), 12);
    }

    #[test]
    fn test_keypad_cost_1() {
        assert_eq!(keypad_cost("029A", 1), 28);
    }

    #[test]
    fn test_keypad_cost_2() {
        assert_eq!(keypad_cost("029A", 2), 68);
    }
}
