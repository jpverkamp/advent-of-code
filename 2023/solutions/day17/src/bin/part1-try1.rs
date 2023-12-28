use anyhow::{anyhow, Ok, Result};
use std::io;

use day17::types::*;
use grid::Grid;
use point::Point;

// aoc_test::generate!{day17_part1_try1_test_17 as "test/17.txt" => ""}
// aoc_test::generate!{day17_part1_try1_17 as "17.txt" => ""}

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

    type Key = (Point, Direction, usize);
    let mut cache = fxhash::FxHashMap::default();

    fn best_path(
        grid: &Grid<u32>,
        cache: &mut fxhash::FxHashMap<Key, (u32, Direction)>,
        p: Point,
        d: Direction,
        c: usize,
        visited: Vec<(Point, Direction, usize)>,
    ) -> Option<(u32, Direction)> {
        log::info!("{p} {d:?} {c}");

        // Already cached
        let key = (p, d, c);
        if let Some(result) = cache.get(&key) {
            return Some(*result);
        }

        // Base case, we're in the south-east corner
        // Direction here doesn't actually matter
        if p.x == grid.bounds.max_x && p.y == grid.bounds.max_y {
            cache.insert(key, (0, South));
            return Some((0, South));
        }

        // We're visiting same case more than once, ignore it
        if visited.contains(&(p, d, c)) {
            return None;
        }
        let mut next_visited = visited.clone();
        next_visited.push((p, d, c));

        // Generate a list of moves based on edge conditions and moving up to 3
        let mut cases = vec![];

        // Try turning left
        {
            let d_next = d.left();
            let p_next = p + Point::from(d_next);
            if grid.bounds.contains(&p_next) {
                cases.push((p_next, d_next, c));
            }
        }

        // Try turning right
        {
            let d_next = d.right();
            let p_next = p + Point::from(d_next);
            if grid.bounds.contains(&p_next) {
                cases.push((p_next, d_next, c));
            }
        }

        // Try going straight (if we haven't gone 3)
        if c < 3 {
            let p_next = p + Point::from(d);
            if grid.bounds.contains(&p_next) {
                cases.push((p_next, d, c + 1));
            }
        }

        // If none of these work, something went wrong
        if cases.is_empty() {
            return None;
        }

        // Find the best of these cases
        let best = cases
            .iter()
            .filter_map(|(p_next, d_next, c_next)| {
                let (score, _) =
                    best_path(grid, cache, *p_next, *d_next, *c_next, next_visited.clone())?;
                Some((score + grid.get(p_next).unwrap(), *d_next))
            })
            .max_by_key(|(score, _)| *score);

        if let Some(best) = best {
            cache.insert(key, best);
            Some(best)
        } else {
            None
        }
    }

    // Fill in the cache
    if let Some((score, _)) = best_path(&grid, &mut cache, Point::new(0, 0), South, 0, vec![]) {
        Ok(score.to_string())
    } else {
        Err(anyhow!("No path found"))
    }
}
