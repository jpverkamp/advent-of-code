use anyhow::Result;
use itertools::Itertools;
use std::io;

use day24::{parse, types::*};

// const MIN_X: f64 = 7_f64;
// const MAX_X: f64 = 27_f64;
// const MIN_Y: f64 = 7_f64;
// const MAX_Y: f64 = 27_f64;

const MIN_X: f64 = 200000000000000_f64;
const MAX_X: f64 = 400000000000000_f64;
const MIN_Y: f64 = 200000000000000_f64;
const MAX_Y: f64 = 400000000000000_f64;

// #[aoc_test("data/test/24.txt", "2")] // with first bounds
// #[aoc_test("data/24.txt", "")]
fn main() -> Result<()> {
    env_logger::init();

    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let (s, lines) = parse::lines(input.as_str()).unwrap();
    assert!(s.trim().is_empty());

    let result = lines
        .iter()
        .cartesian_product(lines.iter())
        .filter_map(|(l1, l2)| intersect_xy(*l1, *l2))
        .inspect(|p| log::info!("potential: {:?}", p))
        .filter(|p| p.x >= MIN_X && p.x <= MAX_X && p.y >= MIN_Y && p.y <= MAX_Y)
        .inspect(|p| log::info!("in bounds: {:?}", p))
        .count()
        / 2; // counts l1,l2 and l2,l1

    println!("{result:?}");
    Ok(())
}

fn intersect_xy(l1: Line, l2: Line) -> Option<Point> {
    // origin is x1/y1/x2/y2
    // direction is dx1/dy1/dx2/dy2
    let Point {
        x: x1,
        y: y1,
        z: _z1,
    } = l1.origin;
    let Point {
        x: x2,
        y: y2,
        z: _z2,
    } = l2.origin;
    let Point {
        x: dx1,
        y: dy1,
        z: _dz1,
    } = l1.direction;
    let Point {
        x: dx2,
        y: dy2,
        z: _dz2,
    } = l2.direction;

    // times are t and u

    // x1 + t * dx1 = x2 + u * dx2
    // t * dx1 = x2 + u * dx2 - x1
    // t = (x2 + u * dx2 - x1) / dx1

    // y1 + t * dy1 = y2 + u * dy2
    // t * dy1 = y2 + u * dy2 - y1
    // t = (y2 + u * dy2 - y1) / dy1

    // (x2 + u * dx2 - x1) / dx1 = (y2 + u * dy2 - y1) / dy1
    // (x2 + u * dx2 - x1) * dy1 = (y2 + u * dy2 - y1) * dx1
    // x2 * dy1 + u * dx2 * dy1 - x1 * dy1 = y2 * dx1 + u * dy2 * dx1 - y1 * dx1
    // u * dx2 * dy1 - u * dy2 * dx1 = y2 * dx1 - x2 * dy1 - y1 * dx1 + x1 * dy1
    // u * (dx2 * dy1 - dy2 * dx1) = y2 * dx1 - x2 * dy1 - y1 * dx1 + x1 * dy1
    // u = (y2 * dx1 - x2 * dy1 - y1 * dx1 + x1 * dy1) / (dx2 * dy1 - dy2 * dx1)
    if (dx2 * dy1 - dy2 * dx1).abs() < 0.0000001_f64 {
        return None;
    }

    let u = (y2 * dx1 - x2 * dy1 - y1 * dx1 + x1 * dy1) / (dx2 * dy1 - dy2 * dx1);
    let t = (x2 + u * dx2 - x1) / dx1;

    if u < 0_f64 || t < 0_f64 {
        return None;
    }

    let x = x1 + t * dx1;
    let y = y1 + t * dy1;

    Some(Point { x, y, z: 0_f64 })
}
