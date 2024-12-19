use aoc_runner_derive::{aoc, aoc_generator};
use image::ImageBuffer;

use crate::{Direction, Grid, Point};

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub width: usize,
    pub height: usize,
    pub part1_cutoff: usize,
    pub points: Vec<Point>,
}

impl Puzzle {
    pub fn render_ascii(&self, cutoff: usize, highlight: &[Point]) -> String {
        let mut grid = Grid::new(self.width, self.height);

        for point in self.points.iter().take(cutoff) {
            grid.set(*point, 1);
        }

        for point in highlight.iter() {
            grid.set(*point, 2);
        }

        grid.to_string(&|cell| {
            match cell {
                0 => '.',
                1 => '#',
                2 => 'O',
                _ => unreachable!(),
            }
            .to_string()
        })
    }

    pub fn render_image(
        &self,
        cutoff: usize,
        highlight: &[Point],
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut image = ImageBuffer::new(self.width as u32, self.height as u32);

        for point in self.points.iter().take(cutoff) {
            image.put_pixel(point.x as u32, point.y as u32, image::Rgb([255, 255, 255]));
        }

        for point in highlight.iter() {
            image.put_pixel(point.x as u32, point.y as u32, image::Rgb([255, 0, 0]));
        }

        image
    }
}

#[aoc_generator(day18)]
pub fn parse(input: &str) -> Puzzle {
    let mut width = 71;
    let mut height = 71;
    let mut cutoff = 1024;
    let mut points = vec![];

    for line in input.lines() {
        if line.starts_with('#') {
            let (size_part, cutoff_part) = line[2..].split_once('@').unwrap();
            cutoff = cutoff_part.parse().unwrap();

            let (width_part, height_part) = size_part.split_once('x').unwrap();
            width = width_part.parse().unwrap();
            height = height_part.parse().unwrap();
        }

        if line.contains(',') {
            let (x, y) = line.split_once(',').unwrap();
            points.push((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()).into());
        }
    }

    Puzzle {
        width,
        height,
        part1_cutoff: cutoff,
        points,
    }
}

#[aoc(day18, part1, v1)]
fn part1_v1(input: &Puzzle) -> i32 {
    let end = (input.width - 1, input.height - 1).into();

    match pathfinding::prelude::astar(
        &Point::ZERO,
        |&point| {
            Direction::all()
                .iter()
                .filter_map(|d| {
                    let new_p = point + *d;
                    if new_p.x < 0
                        || new_p.y < 0
                        || new_p.x >= input.width as i32
                        || new_p.y >= input.height as i32
                        || input.points[..input.part1_cutoff].contains(&new_p)
                    {
                        None
                    } else {
                        Some((new_p, 1))
                    }
                })
                .collect::<Vec<_>>()
        },
        |point| point.manhattan_distance(&end),
        |point| *point == end,
    ) {
        Some((_, cost)) => cost,
        _ => panic!("unsolvable maze"),
    }
}

#[aoc(day18, part1, v2_grid)]
fn part1_v2_grid(input: &Puzzle) -> i32 {
    let end = (input.width - 1, input.height - 1).into();

    let mut grid = Grid::new(input.width, input.height);
    for point in input.points.iter().take(input.part1_cutoff) {
        grid.set(*point, true);
    }

    match pathfinding::prelude::astar(
        &Point::ZERO,
        |&point| {
            Direction::all()
                .iter()
                .filter_map(|d| {
                    let new_p = point + *d;
                    if grid.get(new_p) != Some(&false) {
                        None
                    } else {
                        Some((new_p, 1))
                    }
                })
                .collect::<Vec<_>>()
        },
        |point| point.manhattan_distance(&end),
        |point| *point == end,
    ) {
        Some((_, cost)) => cost,
        _ => panic!("unsolvable maze"),
    }
}

#[aoc(day18, part2, v1)]
fn part2_v1(input: &Puzzle) -> String {
    let end = (input.width - 1, input.height - 1).into();

    let p = (input.part1_cutoff..)
        .find_map(|cutoff| {
            match pathfinding::prelude::astar(
                &Point::ZERO,
                |&point| {
                    Direction::all()
                        .iter()
                        .filter_map(|d| {
                            let new_p = point + *d;
                            if new_p.x < 0
                                || new_p.y < 0
                                || new_p.x >= input.width as i32
                                || new_p.y >= input.height as i32
                                || input.points[..cutoff].contains(&new_p)
                            {
                                None
                            } else {
                                Some((new_p, 1))
                            }
                        })
                        .collect::<Vec<_>>()
                },
                |point| point.manhattan_distance(&end),
                |point| *point == end,
            ) {
                Some(_) => None,
                _ => Some(input.points[cutoff - 1]),
            }
        })
        .unwrap();

    format!("{x},{y}", x = p.x, y = p.y)
}

#[aoc(day18, part2, v2_two_neighbors)]
fn part2_v2_two_neighbors(input: &Puzzle) -> String {
    let end = (input.width - 1, input.height - 1).into();

    let p = (input.part1_cutoff..)
        .find_map(|cutoff| {
            let new_point = input.points[cutoff - 1];

            // To be a cutoff, the new point must have exactly two open neighbors
            if Direction::all()
                .iter()
                .filter(|d| {
                    let new_p = new_point + **d;
                    new_p.x >= 0
                        && new_p.y >= 0
                        && new_p.x < input.width as i32
                        && new_p.y < input.height as i32
                        && !input.points[..cutoff].contains(&new_p)
                })
                .count()
                != 2
            {
                return None;
            }

            match pathfinding::prelude::astar(
                &Point::ZERO,
                |&point| {
                    Direction::all()
                        .iter()
                        .filter_map(|d| {
                            let new_p = point + *d;
                            if new_p.x < 0
                                || new_p.y < 0
                                || new_p.x >= input.width as i32
                                || new_p.y >= input.height as i32
                                || input.points[..cutoff].contains(&new_p)
                            {
                                None
                            } else {
                                Some((new_p, 1))
                            }
                        })
                        .collect::<Vec<_>>()
                },
                |point| point.manhattan_distance(&end),
                |point| *point == end,
            ) {
                Some(_) => None,
                _ => Some(input.points[cutoff - 1]),
            }
        })
        .unwrap();

    format!("{x},{y}", x = p.x, y = p.y)
}

#[aoc(day18, part2, v3_grid)]
fn part2_v3_grid(input: &Puzzle) -> String {
    let end = (input.width - 1, input.height - 1).into();

    let mut grid = Grid::new(input.width, input.height);
    for p in input.points.iter().take(input.part1_cutoff) {
        grid.set(*p, true);
    }

    let p = (input.part1_cutoff..)
        .find_map(|cutoff| {
            let new_point = input.points[cutoff - 1];
            grid.set(new_point, true);

            // To be a cutoff, the new point must have exactly two open neighbors
            if new_point
                .neighbors()
                .iter()
                .filter(|&p| grid.get(*p) != Some(&false))
                .count()
                != 2
            {
                return None;
            }

            // Verify if the new point *actually* cut us off
            match pathfinding::prelude::astar(
                &Point::ZERO,
                |&point| {
                    Direction::all()
                        .iter()
                        .filter_map(|d| {
                            if grid.get(point + *d) == Some(&false) {
                                Some((point + *d, 1))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                },
                |point| point.manhattan_distance(&end),
                |point| *point == end,
            ) {
                Some(_) => None,
                _ => Some(input.points[cutoff - 1]),
            }
        })
        .unwrap();

    format!("{x},{y}", x = p.x, y = p.y)
}

#[aoc(day18, part2, v4_on_best_path)]
fn part2_v4_on_best_path(input: &Puzzle) -> String {
    let end = (input.width - 1, input.height - 1).into();

    let mut grid = Grid::new(input.width, input.height);
    for p in input.points.iter().take(input.part1_cutoff) {
        grid.set(*p, true);
    }

    let mut previous_best_path = None;

    let p = (input.part1_cutoff..input.points.len())
        .find_map(|cutoff| {
            let new_point = input.points[cutoff - 1];
            grid.set(new_point, true);

            // To be a cutoff, the new point must have exactly two open neighbors
            if new_point
                .neighbors()
                .iter()
                .filter(|&p| grid.get(*p) != Some(&false))
                .count()
                != 2
            {
                return None;
            }

            // And it must have been on the previous best path (if there was one)
            if previous_best_path
                .as_ref()
                .map_or(false, |path: &Vec<Point>| !path.contains(&new_point))
            {
                return None;
            }

            // Verify if the new point *actually* cut us off
            match pathfinding::prelude::astar(
                &Point::ZERO,
                |&point| {
                    Direction::all()
                        .iter()
                        .filter_map(|d| {
                            if grid.get(point + *d) == Some(&false) {
                                Some((point + *d, 1))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                },
                |point| point.manhattan_distance(&end),
                |point| *point == end,
            ) {
                Some((path, _)) => {
                    previous_best_path = Some(path);
                    None
                }
                _ => Some(input.points[cutoff - 1]),
            }
        })
        .unwrap();

    format!("{x},{y}", x = p.x, y = p.y)
}

#[aoc(day18, part2, v5_binary)]
fn part2_v5_binary(input: &Puzzle) -> String {
    let end = (input.width - 1, input.height - 1).into();

    let mut lower_bound = input.part1_cutoff;
    let mut upper_bound = input.points.len();

    let mut guess = (lower_bound + upper_bound) / 2;

    loop {
        let mut grid = Grid::new(input.width, input.height);
        for p in input.points.iter().take(guess) {
            grid.set(*p, true);
        }

        match pathfinding::prelude::astar(
            &Point::ZERO,
            |&point| {
                Direction::all()
                    .iter()
                    .filter_map(|d| {
                        if grid.get(point + *d) == Some(&false) {
                            Some((point + *d, 1))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            },
            |point| point.manhattan_distance(&end),
            |point| *point == end,
        ) {
            Some((_, _)) => {
                if upper_bound - lower_bound <= 1 {
                    break;
                }
                lower_bound = guess;
                guess = (upper_bound + guess) / 2;
            }
            None => {
                if upper_bound - lower_bound <= 1 {
                    break;
                }
                upper_bound = guess;
                guess = (lower_bound + guess) / 2;
            }
        }
    }

    let point = input.points[guess - 1];
    format!("{x},{y}", x = point.x, y = point.y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
# 7x7@12
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    make_test!([part1_v1] => "day18.txt", 22, 354);
    make_test!([part2_v1, part2_v2_two_neighbors, part2_v3_grid, part2_v4_on_best_path] => "day18.txt", "6,1", "36,17");
}

