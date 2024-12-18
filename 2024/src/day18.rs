use aoc_runner_derive::{aoc, aoc_generator};
use image::ImageBuffer;

use crate::{Direction, Grid, Point};

#[derive(Debug, Clone)]
struct Puzzle {
    width: usize,
    height: usize,
    part1_cutoff: usize,
    points: Vec<Point>,
}

impl Puzzle {
    #[allow(dead_code)]
    fn render_ascii(&self, highlight: &[Point]) -> String {
        let mut grid = Grid::new(self.width, self.height);

        for point in self.points.iter() {
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

    #[allow(dead_code)]
    fn render_image(&self, highlight: &[Point]) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut image = ImageBuffer::new(self.width as u32, self.height as u32);

        for point in self.points.iter() {
            image.put_pixel(point.x as u32, point.y as u32, image::Rgb([255, 255, 255]));
        }

        for point in highlight.iter() {
            image.put_pixel(point.x as u32, point.y as u32, image::Rgb([255, 0, 0]));
        }

        image
    }
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Puzzle {
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

#[aoc(day18, part2, v2)]
fn part2_v2(input: &Puzzle) -> String {
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

#[aoc(day18, part2, v3)]
fn part2_v3(input: &Puzzle) -> String {
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
                .filter(|&p| grid.get(*p) == Some(&true))
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
    make_test!([part2_v1, part2_v2] => "day18.txt", "6,1", "36,17");
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_v1(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_v3(&parse(input)).to_string()
}
