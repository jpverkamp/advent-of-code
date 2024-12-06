use crate::{Direction, Grid, Point};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

#[derive(Debug, Copy, Clone, Default)]
enum Tile {
    #[default]
    Empty,
    Wall,
}

#[derive(Debug, Clone)]
struct Map {
    player: Point,
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

    let player_index = input.find('^').unwrap();

    let per_row = grid.width + 1;
    let player = Point::new(
        (player_index % per_row) as i32,
        (player_index / per_row) as i32,
    );
    let facing = Direction::Up;

    Map {
        player,
        facing,
        grid,
    }
}

impl Map {
    fn visited(&self, check_loops: bool) -> Option<Grid<bool>> {
        let Map {
            mut player,
            mut facing,
            grid,
        } = self;

        let mut visited = Grid::new(grid.width, grid.height);
        visited.set(player, true);

        let mut duplicates = hashbrown::HashSet::new();
        duplicates.insert((player, facing));

        while grid.in_bounds(player) {
            match grid.get(player + facing) {
                Some(Tile::Empty) => {
                    player += facing.into();
                    visited.set(player, true);
                }
                Some(Tile::Wall) => {
                    facing = facing.rotate_cw();
                }
                None => break,
            }

            if check_loops {
                if duplicates.contains(&(player, facing)) {
                    return None;
                }
                duplicates.insert((player, facing));
            }
        }

        Some(visited)
    }
}

#[aoc(day6, part1, v1)]
fn part1_v1(input: &Map) -> usize {
    input.visited(false).unwrap().iter().filter(|&v| *v).count()
}

// For each point on the grid, check if adding a wall there would create a loop
#[aoc(day6, part2, v1)]
fn part2_v1(input: &Map) -> usize {
    iproduct!(0..input.grid.width, 0..input.grid.height,)
        .filter(|&(x, y)| {
            let mut input = input.clone();
            input.grid.set((x, y), Tile::Wall);

            input.visited(true).is_none()
        })
        .count()
}

// Only check walls on or adjacent to an originally visited path
#[aoc(day6, part2, limited)]
fn part2_limited(input: &Map) -> usize {
    let visited = input.visited(false).unwrap();
    iproduct!(0..input.grid.width, 0..input.grid.height)
        .filter(|&(x, y)| {
            let p = Point::from((x, y));
            visited.get(p) == Some(&true)
                || p.neighbors().iter().any(|&p| visited.get(p) == Some(&true))
        })
        .filter(|&(x, y)| {
            let mut input = input.clone();
            input.grid.set((x, y), Tile::Wall);
            input.visited(true).is_none()
        })
        .count()
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
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_v1(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_v1(&parse(input)).to_string()
}
