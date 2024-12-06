use crate::{Direction, Grid, Point};
use aoc_runner_derive::{aoc, aoc_generator};

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

#[aoc(day6, part1, v1)]
fn part1_v1(input: &Map) -> usize {
    let Map {
        mut player,
        mut facing,
        grid,
    } = input;
    let mut path = Grid::new(grid.width, grid.height);

    path.set(player, true);

    while grid.in_bounds(player) {
        match grid.get(player + facing) {
            Some(Tile::Empty) => {
                player += facing.into();
                path.set(player, true);
            }
            Some(Tile::Wall) => {
                facing = facing.rotate_cw();
            }
            None => break,
        }
    }

    path.iter().filter(|&&b| b).count()
}

#[aoc(day6, part2, v1)]
fn part2_v1(input: &Map) -> usize {
    let mut looping = 0;

    for x in 0..input.grid.width {
        for y in 0..input.grid.height {
            let Map {
                mut player,
                mut facing,
                grid,
            } = input;
            let mut grid = grid.clone();
            grid.set((x, y), Tile::Wall);

            let mut seen = hashbrown::HashSet::new();
            seen.insert((player, facing));

            while grid.in_bounds(player) {
                match grid.get(player + facing) {
                    Some(Tile::Empty) => {
                        player += facing.into();
                    }
                    Some(Tile::Wall) => {
                        facing = facing.rotate_cw();
                    }
                    None => break,
                }

                if seen.contains(&(player, facing)) {
                    looping += 1;
                    break;
                }

                seen.insert((player, facing));
            }
        }
    }

    looping
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
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_v1(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_v1(&parse(input)).to_string()
}
