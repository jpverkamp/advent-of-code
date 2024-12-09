use aoc_runner_derive::{aoc, aoc_generator};

use crate::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tile {
    #[default]
    Empty,
    Tower(char),
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Grid<Tile> {
    Grid::read(input, &|c| match c {
        '.' => Tile::Empty,
        _ => Tile::Tower(c),
    })
}

#[aoc(day8, part1, v1)]
fn part1_v1(input: &Grid<Tile>) -> usize {
    let mut towers = hashbrown::HashMap::new();

    for (point, tile) in input.iter_enumerate() {
        if let Tile::Tower(c) = tile {
            towers.entry(c).or_insert_with(Vec::new).push(point);
        }
    }

    let mut antinodes = Grid::new(input.width, input.height);

    for (_, points) in towers.iter() {
        for p1 in points {
            for p2 in points {
                if p1 != p2 {
                    let d = *p2 - *p1;
                    antinodes.set(*p1 - d, true);
                    antinodes.set(*p2 + d, true);
                }
            }
        }
    }

    antinodes.iter().filter(|&b| *b).count()
}

#[aoc(day8, part2, v1)]
fn part2_v1(input: &Grid<Tile>) -> usize {
    let mut towers = hashbrown::HashMap::new();

    for (point, tile) in input.iter_enumerate() {
        if let Tile::Tower(c) = tile {
            towers.entry(c).or_insert_with(Vec::new).push(point);
        }
    }

    let mut antinodes = Grid::new(input.width, input.height);

    for (_, points) in towers.iter() {
        for p1 in points {
            for p2 in points {
                if p1 != p2 {
                    let delta = *p2 - *p1;

                    let mut p = *p1 + delta;
                    while input.in_bounds(p) {
                        antinodes.set(p, true);
                        p += delta;
                    }

                    let mut p = *p1 - delta;
                    while input.in_bounds(p) {
                        antinodes.set(p, true);
                        p -= delta;
                    }
                }
            }
        }
    }

    antinodes.iter().filter(|&b| *b).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........";

    make_test!([part1_v1] => "day8.txt", 4, 299);
    make_test!([part2_v1] => "day8.txt", 8, 1032);
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_v1(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_v1(&parse(input)).to_string()
}
