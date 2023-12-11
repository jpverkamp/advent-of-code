use anyhow::Result;
use std::{collections::HashSet, io};

use day10::types::*;

// #[aoc_test("data/test/10.txt", "1")]
// #[aoc_test("data/test/10b.txt", "1")]
// #[aoc_test("data/test/10c.txt", "1")]
// #[aoc_test("data/test/10d.txt", "4")]
// #[aoc_test("data/test/10e.txt", "8")]
// #[aoc_test("data/test/10f.txt", "10")]
// #[aoc_test("data/10.txt", "455")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let map = Map::from(input.as_str());

    let (min_x, min_y, max_x, max_y) = map.bounds();

    // Each region is a hash set of points
    let mut region_cw = HashSet::new();
    let mut region_ccw = HashSet::new();

    let mut outside_cw = false;
    let mut outside_ccw = false;

    // Determine the main loop
    let loop_points = map
        .iter()
        .map(|node| (node.x(), node.y()))
        .collect::<HashSet<_>>();

    // Given a specific start point, flood fill all points into the specific region
    // Do not add points that are:
    // 1 - Part of the loop (points that are set but not part of the loop are fine)
    // 2 - Out of bounds
    // 3 - Already added
    let flood_fill = |region: &mut HashSet<(isize, isize)>, start: (isize, isize)| -> bool {
        let mut stack = vec![start];
        let mut is_outside = false;

        while let Some((x, y)) = stack.pop() {
            // Never add points on the loop
            if loop_points.contains(&(x, y)) {
                continue;
            }

            // Never add points out of bounds
            if x < min_x - 1 || x > max_x + 1 || y < min_y - 1 || y > max_y + 1 {
                is_outside = true;
                continue;
            }

            // Stop if we've already added this point to the region
            if region.contains(&(x, y)) {
                continue;
            }

            // Otherwise, add it and expand
            region.insert((x, y));
            stack.push((x + 1, y));
            stack.push((x - 1, y));
            stack.push((x, y + 1));
            stack.push((x, y - 1));
        }

        is_outside
    };

    // Over each pair of points, determine which side is 'clockwise' and which 'counter-clockwise' from that point
    // Flood fill the approproiate region
    map.iter()
        .zip(map.iter().cycle().skip(1))
        .for_each(|(n1, n2)| {
            let (x1, y1) = (n1.x(), n1.y());
            let (x2, y2) = (n2.x(), n2.y());
            let xd = x2 - x1;
            let yd = y2 - y1;

            match (xd, yd) {
                (0, -1) => {
                    // Up
                    // ...
                    // .2x
                    // .1x
                    // ...
                    (0..=1).for_each(|yd| {
                        outside_cw |= flood_fill(&mut region_cw, (x2 + 1, y2 + yd));
                        outside_ccw |= flood_fill(&mut region_ccw, (x2 - 1, y2 + yd));
                    });
                }
                (1, 0) => {
                    // Right
                    // ....
                    // .12.
                    // .xx.
                    (-1..=0).for_each(|xd| {
                        outside_cw |= flood_fill(&mut region_cw, (x2 + xd, y2 + 1));
                        outside_ccw |= flood_fill(&mut region_ccw, (x2 + xd, y2 - 1));
                    });
                }
                (0, 1) => {
                    // Down
                    // ...
                    // x1.
                    // x2.
                    // ...
                    (-1..=0).for_each(|yd| {
                        outside_cw |= flood_fill(&mut region_cw, (x2 - 1, y2 + yd));
                        outside_ccw |= flood_fill(&mut region_ccw, (x2 + 1, y2 + yd));
                    });
                }
                (-1, 0) => {
                    // Left
                    // .xx.
                    // .21.
                    // ....
                    (0..=1).for_each(|xd| {
                        outside_cw |= flood_fill(&mut region_cw, (x2 + xd, y2 - 1));
                        outside_ccw |= flood_fill(&mut region_ccw, (x2 + xd, y2 + 1));
                    });
                }
                _ => panic!("Invalid direction: ({}, {})", xd, yd),
            }

            // match (xd, yd) {
            //     (0, -1) => {
            //         // Up
            //         outside_cw |= flood_fill(&mut region_cw, (x2 + 1, y2));
            //         outside_ccw |= flood_fill(&mut region_ccw, (x2 - 1, y2));
            //     }
            //     (1, 0) => {
            //         // Right
            //         outside_cw |= flood_fill(&mut region_cw, (x2, y2 + 1));
            //         outside_ccw |= flood_fill(&mut region_ccw, (x2, y2 - 1));
            //     }
            //     (0, 1) => {
            //         // Down
            //         outside_cw |= flood_fill(&mut region_cw, (x2 - 1, y2));
            //         outside_ccw |= flood_fill(&mut region_ccw, (x2 + 1, y2));
            //     }
            //     (-1, 0) => {
            //         // Left
            //         outside_cw |= flood_fill(&mut region_cw, (x2, y2 - 1));
            //         outside_ccw |= flood_fill(&mut region_ccw, (x2, y2 + 1));
            //     }
            //     _ => panic!("Invalid direction: ({}, {})", xd, yd),
            // }
        });
    assert!(outside_cw ^ outside_ccw);

    let result = if outside_ccw { region_cw } else { region_ccw }.len();

    println!("{result}");
    Ok(())
}
