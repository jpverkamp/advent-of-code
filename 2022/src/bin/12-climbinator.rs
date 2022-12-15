use aoc::*;
use std::{collections::VecDeque, path::Path};

const MOVES: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 1, y: 0 },
];

#[derive(Debug)]
struct HeightMap {
    data: Matrix<u8>,
    src: Point,
    dst: Point,
}

impl From<Vec<String>> for HeightMap {
    fn from(lines: Vec<String>) -> Self {
        let width = lines.get(0).expect("must have at least one line").len();
        let height = lines.len();

        let mut data = Matrix::new(width, height);
        let mut src = Point::ORIGIN;
        let mut dst = Point::ORIGIN;

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        data[[x, y]] = 1;
                        src = Point {
                            x: x as isize,
                            y: y as isize,
                        };
                    }
                    'E' => {
                        data[[x, y]] = 26;
                        dst = Point {
                            x: x as isize,
                            y: y as isize,
                        };
                    }
                    _ => {
                        data[[x, y]] = (c as u8) - ('a' as u8) + 1;
                    }
                }
            }
        }

        HeightMap { data, src, dst }
    }
}

#[derive(Debug)]
struct DistanceMap {
    distances: Matrix<Option<usize>>,
    directions: Matrix<Point>,
}

impl From<&HeightMap> for DistanceMap {
    fn from(heights: &HeightMap) -> Self {
        let width = heights.data.width();
        let height = heights.data.height();

        let mut distances = Matrix::<Option<usize>>::new(width, height);
        let mut directions = Matrix::<Point>::new(width, height);

        distances[[heights.dst.x as usize, heights.dst.y as usize]] = Some(0);

        let mut q = VecDeque::new();
        q.push_back(heights.dst);

        while !q.is_empty() {
            let p_dst = q.pop_front().unwrap();
            let h_dst = heights.data.at(&p_dst);

            // d_dst will always be set if there are no bugs
            let d_dst_p = distances.at(&p_dst);
            let d_dst = d_dst_p.unwrap();

            for m in MOVES.into_iter() {
                let p_src = p_dst + m;

                if !distances.in_bounds(p_src.x as usize, p_src.y as usize) {
                    continue;
                }

                let h_src = heights.data.at(&p_src);
                let d_src = distances.at(&p_src);

                // If the jump is too high, can't go this way
                if *h_dst > h_src + 1 {
                    continue;
                }

                // If we already have a better path, don't go this way
                if d_src.is_some() && d_src.unwrap() < d_dst + 1 {
                    continue;
                }

                // If we make it this far, we found a new (valid) better path

                // Store the new distance and direction
                distances[[p_src.x as usize, p_src.y as usize]] = Some(d_dst + 1);
                directions[[p_src.x as usize, p_src.y as usize]] = m;

                // Add this point to the queue of points to expand further
                if !q.contains(&p_src) {
                    q.push_back(p_src);
                }
            }
        }

        DistanceMap {
            distances,
            directions,
        }
    }
}

fn part1(filename: &Path) -> String {
    let lines = read_lines(filename);
    let height_map = HeightMap::from(lines);
    let distance_map = DistanceMap::from(&height_map);

    if cfg!(debug_assertions) {
        for y in 0..height_map.data.height() {
            for x in 0..height_map.data.width() {
                match distance_map.distances[[x, y]] {
                    Some(d) => {
                        print!("{:4}", d);
                    }
                    None => {
                        print!("  . ");
                    }
                }
            }
            println!();
        }
        println!();

        for y in 0..height_map.data.height() {
            for x in 0..height_map.data.width() {
                print!(
                    "{}",
                    match distance_map.directions[[x, y]] {
                        Point { x: 0, y: 1 } => '^',
                        Point { x: 0, y: -1 } => 'v',
                        Point { x: 1, y: 0 } => '<',
                        Point { x: -1, y: 0 } => '>',
                        _ => '.',
                    }
                );
            }
            println!();
        }
    }

    distance_map.distances[[height_map.src.x as usize, height_map.src.y as usize]]
        .expect("must have a solution")
        .to_string()
}

fn part2(filename: &Path) -> String {
    let lines = read_lines(filename);
    let height_map = HeightMap::from(lines);
    let distance_map = DistanceMap::from(&height_map);

    let mut best_distance = usize::MAX;

    for y in 0..height_map.data.height() {
        for x in 0..height_map.data.width() {
            if height_map.data[[x, y]] > 1 {
                continue;
            }

            match distance_map.distances[[x, y]] {
                Some(d) if d < best_distance => {
                    best_distance = d;
                }
                _ => {}
            }
        }
    }

    best_distance.to_string()
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc::aoc_test;

    #[test]
    fn test1() {
        aoc_test("12", part1, "383")
    }

    #[test]
    fn test2() {
        aoc_test("12", part2, "377")
    }
}
