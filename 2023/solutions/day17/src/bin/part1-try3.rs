use anyhow::{Ok, Result};
use std::io;

use day17::types::*;
use grid::Grid;
use point::Point;
use Direction::*;

type Key = (Point, Direction, usize);

fn best_path(
    grid: &Grid<u32>,
    cache: &mut fxhash::FxHashMap<Key, (u32, Direction)>,
    p: Point,
    d: Direction,
    c: usize,
    path: Vec<(Point, Direction, usize)>,
    fuel: usize,
) -> Option<(u32, Direction)> {
    // println!("{p} {d:?} {c}", p = p, d = d, c = c);
    // println!("cache size: {}", cache.len());

    // Out of fuel, stop recurring
    if fuel == 0 {
        return None;
    }

    // Base case, we're in the south-east corner
    // Direction here doesn't actually matter
    if p.x == grid.bounds.max_x && p.y == grid.bounds.max_y {
        cache.insert((p, d, c), (0, South));
        return Some((0, South));
    }

    // Cache hit
    if let Some(result) = cache.get(&(p, d, c)) {
        return Some(*result);
    }

    // // We're visiting same case more than once, ignore it
    // if path.contains(&(p, d, c)) {
    //     return None;
    // }

    // Three cases: left, straight, right
    let cases = [
        (p + Point::from(d), d, c + 1),
        (p + Point::from(d.left()), d.left(), 1),
        (p + Point::from(d.right()), d.right(), 1),
    ];

    let result = cases
        .iter()
        // Remove cases that go out of bounds
        .filter(|(p, _, _)| grid.bounds.contains(p))
        // Remove cases that go straight too far
        .filter(|(_, _, c)| *c <= 3)
        // Remove cases we've already seen
        .filter(|(p, d, c)| !path.contains(&(*p, *d, *c)))
        // Score and return the best of each
        .filter_map(|(p, d, c)| {
            // Update path
            let mut path = path.clone();
            path.push((*p, *d, *c));

            // Make the recursive call, add the new heat to it
            // Ignore any calls that fail
            let mut result = best_path(grid, cache, *p, *d, *c, path, fuel - 1)?;
            result.0 += grid.get(p)?;
            Some(result)
        })
        .min();

    if let Some(result) = result {
        cache.insert((p, d, c), result);
    }

    result
}

// #[aoc_test("data/test/17.txt", "")]
// #[aoc_test("data/17.txt", "796")]
// 796 is too high
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;

    let grid = Grid::read(input.as_str(), |c| c.to_digit(10));
    let mut cache = fxhash::FxHashMap::default();

    // Pre-populate the cache
    for x in (grid.bounds.min_x..=grid.bounds.max_x).rev() {
        for y in (grid.bounds.min_y..=grid.bounds.max_y).rev() {
            let p = Point::new(x, y);
            let _ = best_path(&grid, &mut cache, p, South, 0, vec![], 100);
        }
    }
    println!("pre-cache done, cache size: {}", cache.len());

    if let Some((score, direction)) = best_path(
        &grid,
        &mut cache,
        Point::new(0, 0),
        South,
        0,
        vec![],
        usize::MAX,
    ) {
        println!("{} {:?}", score, direction);
    } else {
        println!("No solution found");
    }

    Ok(())
}
