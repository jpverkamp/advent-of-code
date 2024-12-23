use crate::{Direction, Grid, Point};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum Tile {
    #[default]
    Empty,
    Wall,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub guard: Point,
    pub facing: Direction,
    pub grid: Grid<Tile>,
}

#[aoc_generator(day6)]
pub fn parse(input: &str) -> Map {
    let grid = Grid::read(input, &|c| match c {
        '.' => Tile::Empty,
        '#' => Tile::Wall,
        '^' => Tile::Empty,
        _ => panic!("Invalid character: {}", c),
    });

    let guard_index = input.find('^').unwrap();

    let per_row = grid.width + 1;
    let guard = Point::new(
        (guard_index % per_row) as i32,
        (guard_index / per_row) as i32,
    );
    let facing = Direction::Up;

    Map {
        guard,
        facing,
        grid,
    }
}

impl Map {
    fn walk(&self) -> Option<Grid<bool>> {
        let Map {
            mut guard,
            mut facing,
            grid,
        } = self;

        let mut visited = Grid::new(grid.width, grid.height);
        visited.set(guard, true);

        while grid.in_bounds(guard) {
            match grid.get(guard + facing) {
                Some(Tile::Empty) => {
                    guard += facing.into();
                    visited.set(guard, true);
                }
                Some(Tile::Wall) => {
                    facing = facing.rotate_cw();
                }
                None => break,
            }
        }

        Some(visited)
    }

    fn loops(&self) -> bool {
        let Map {
            mut guard,
            mut facing,
            grid,
        } = self;

        let mut duplicates_up = Grid::new(grid.width, grid.height);
        duplicates_up.set(guard, true);

        let mut duplicates_left = Grid::new(grid.width, grid.height);
        let mut duplicates_right = Grid::new(grid.width, grid.height);
        let mut duplicates_down = Grid::new(grid.width, grid.height);

        while grid.in_bounds(guard) {
            match grid.get(guard + facing) {
                Some(Tile::Empty) => {
                    guard += facing.into();
                }
                Some(Tile::Wall) => {
                    facing = facing.rotate_cw();
                }
                None => break,
            }

            let duplicates = &mut match facing {
                Direction::Up => &mut duplicates_up,
                Direction::Left => &mut duplicates_left,
                Direction::Right => &mut duplicates_right,
                Direction::Down => &mut duplicates_down,
            };

            if duplicates.get(guard) == Some(&true) {
                return false;
            }
            duplicates.set(guard, true);
        }

        true
    }
}

#[aoc(day6, part1, v1)]
fn part1_v1(input: &Map) -> usize {
    input.walk().unwrap().iter().filter(|&v| *v).count()
}

// For each point on the grid, check if adding a wall there would create a loop
#[aoc(day6, part2, v1)]
fn part2_v1(input: &Map) -> usize {
    iproduct!(0..input.grid.width, 0..input.grid.height,)
        .filter(|&(x, y)| {
            // The 'visited' function returns None on loops (no path found)
            let mut input = input.clone();
            input.grid.set((x, y), Tile::Wall);
            !input.loops()
        })
        .count()
}

// Only check adding walls to the original path
// We don't have to check adjacent since you have to 'run into' a wall to turn
#[aoc(day6, part2, limited)]
fn part2_limited(input: &Map) -> usize {
    let visited = input.walk().unwrap();
    iproduct!(0..input.grid.width, 0..input.grid.height)
        .filter(|&(x, y)| {
            let p = Point::from((x, y));
            if visited.get(p) == Some(&true) {
                let mut input = input.clone();
                input.grid.set(p, Tile::Wall);
                !input.loops()
            } else {
                false
            }
        })
        .count()
}

// Try without cloning the input (more than once)
#[aoc(day6, part2, no_clone)]
fn part2_limited_no_clone(input: &Map) -> usize {
    let mut input = input.clone();

    let visited = input.walk().unwrap();
    iproduct!(0..input.grid.width, 0..input.grid.height)
        // Any points not on or adjacent to original path cannot introduce a loop
        .filter(|&(x, y)| {
            let p = Point::from((x, y));
            if visited.get(p) == Some(&true) {
                input.grid.set((x, y), Tile::Wall);
                let result = !input.loops();
                input.grid.set((x, y), Tile::Empty);
                result
            } else {
                false
            }
        })
        .count()
}

// Add rayon parallelization
#[aoc(day6, part2, limited_rayon)]
fn part2_limited_rayon(input: &Map) -> usize {
    let visited = input.walk().unwrap();
    iproduct!(0..input.grid.width, 0..input.grid.height)
        .par_bridge()
        .into_par_iter()
        .map(|(x, y)| {
            let p = Point::from((x, y));

            if visited.get(p) == Some(&true) {
                let mut input = input.clone();
                input.grid.set(p, Tile::Wall);
                if input.loops() {
                    0
                } else {
                    1
                }
            } else {
                0
            }
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    make_test!([part1_v1] => "day6.txt", 41, 5551);
    make_test!([part2_v1, part2_limited, part2_limited_rayon, part2_limited_no_clone] => "day6.txt", 6, 1939);
}
