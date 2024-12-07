use crate::{Direction, Grid, Point};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
enum Tile {
    #[default]
    Empty,
    Wall,
}

#[derive(Debug, Clone)]
struct Map {
    guard: Point,
    facing: Direction,
    grid: Grid<Tile>,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Map {
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
    fn walk(&self, check_loops: bool) -> Option<Grid<bool>> {
        let Map {
            mut guard,
            mut facing,
            grid,
        } = self;

        let mut visited = Grid::new(grid.width, grid.height);
        visited.set(guard, true);

        let mut duplicates_up = Grid::new(grid.width, grid.height);
        duplicates_up.set(guard, true);

        let mut duplicates_left = Grid::new(grid.width, grid.height);
        let mut duplicates_right = Grid::new(grid.width, grid.height);
        let mut duplicates_down = Grid::new(grid.width, grid.height);

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

            if check_loops {
                let duplicates = &mut match facing {
                    Direction::Up => &mut duplicates_up,
                    Direction::Left => &mut duplicates_left,
                    Direction::Right => &mut duplicates_right,
                    Direction::Down => &mut duplicates_down,
                };

                if duplicates.get(guard) == Some(&true) {
                    return None;
                }
                duplicates.set(guard, true);
            }
        }

        Some(visited)
    }
}

#[aoc(day6, part1, v1)]
fn part1_v1(input: &Map) -> usize {
    input.walk(true).unwrap().iter().filter(|&v| *v).count()
}

// For each point on the grid, check if adding a wall there would create a loop
#[aoc(day6, part2, v1)]
fn part2_v1(input: &Map) -> usize {
    iproduct!(0..input.grid.width, 0..input.grid.height,)
        .filter(|&(x, y)| {
            // The 'visited' function returns None on loops (no path found)
            let mut input = input.clone();
            input.grid.set((x, y), Tile::Wall);
            input.walk(true).is_none()
        })
        .count()
}

// Only check adding walls to the original path
// We don't have to check adjacent since you have to 'run into' a wall to turn
#[aoc(day6, part2, limited)]
fn part2_limited(input: &Map) -> usize {
    let visited = input.walk(false).unwrap();
    iproduct!(0..input.grid.width, 0..input.grid.height)
        .filter(|&(x, y)| {
            let p = Point::from((x, y));
            if visited.get(p) == Some(&true) {
                let mut input = input.clone();
                input.grid.set(p, Tile::Wall);
                input.walk(true).is_none()
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

    let visited = input.walk(false).unwrap();
    iproduct!(0..input.grid.width, 0..input.grid.height)
        // Any points not on or adjacent to original path cannot introduce a loop
        .filter(|&(x, y)| {
            let p = Point::from((x, y));
            if visited.get(p) == Some(&true) {
                input.grid.set((x, y), Tile::Wall);
                let result = input.walk(true).is_none();
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
    let visited = input.walk(false).unwrap();
    iproduct!(0..input.grid.width, 0..input.grid.height)
        .into_iter()
        .par_bridge()
        .into_par_iter()
        .map(|(x, y)| {
            let p = Point::from((x, y));

            if visited.get(p) == Some(&true) {
                let mut input = input.clone();
                input.grid.set(p, Tile::Wall);
                if input.walk(true).is_none() {
                    1
                } else {
                    0
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

    #[test]
    fn part1_example() {
        assert_eq!(part1_v1(&parse(EXAMPLE)), 41);
    }

    #[test]
    fn part1_final() {
        assert_eq!(
            part1_v1(&parse(include_str!("../input/2024/day6.txt"))),
            5551
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_v1(&parse(EXAMPLE)), 6);
    }

    #[test]
    fn part2_final() {
        assert_eq!(
            part2_v1(&parse(include_str!("../input/2024/day6.txt"))),
            1939
        );
    }

    #[test]
    fn part2_limited_example() {
        assert_eq!(part2_limited(&parse(EXAMPLE)), 6);
    }

    #[test]
    fn part2_limited_final() {
        assert_eq!(
            part2_limited(&parse(include_str!("../input/2024/day6.txt"))),
            1939
        );
    }


    #[test]
    fn part2_limited_rayon_example() {
        assert_eq!(part2_limited_rayon(&parse(EXAMPLE)), 6);
    }

    #[test]
    fn part2_limited_rayon_final() {
        assert_eq!(
            part2_limited_rayon(&parse(include_str!("../input/2024/day6.txt"))),
            1939
        );
    }
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_v1(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_limited_rayon(&parse(input)).to_string()
}
