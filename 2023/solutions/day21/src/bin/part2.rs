use anyhow::Result;
use fxhash::FxHashSet;
use std::io;

use day21::parse;

use bounds::Bounds;
use point::Point;

const STEPS: i32 = 26501365;

// #[aoc_test("data/test/21.txt", "522388151441217")] // I dunno
// #[aoc_test("data/21.txt", "612941134797232")]
fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (walls, start) = parse::read(input);
    let wall_bounds = Bounds::from(walls.iter());

    // Note: Assuming min bounds are 0
    // The original cell takes cell_width/2 steps to reach the edge
    // And then each cell takes cell_width steps to fill across
    let cell_width = wall_bounds.max_x + 2;
    let cell_height = wall_bounds.max_y + 2;
    let half_width = cell_width / 2;

    // Find the number of cycles it would take to get to the target
    // If this doesn't evenly divide, messy things happen
    let target = ((STEPS as isize) - half_width) / cell_width;

    // A modular wall function
    let wall_mod_contains = |&p: &Point| {
        let mut p = Point::new(p.x % cell_width, p.y % cell_height);

        if p.x < 0 {
            p.x += cell_width;
        }
        if p.y < 0 {
            p.y += cell_height;
        }

        walls.contains(&p)
    };

    // The set of active points
    let mut active = FxHashSet::default();
    active.insert(start);

    // We're not going to have to actually need to iterate this far
    let mut points = Vec::new();
    for step in 1..=STEPS {
        let mut next_active = FxHashSet::default();

        for pos in active {
            for neighbor in pos.neighbors() {
                if !wall_mod_contains(&neighbor) {
                    next_active.insert(neighbor);
                }
            }
        }

        active = next_active;

        if ((step as isize) - half_width) % cell_width == 0 {
            let i = ((step as isize) - half_width) / cell_width;
            let p = Point::new(i, active.len() as isize);
            points.push(p);

            if points.len() == 3 {
                break;
            }
        }
    }

    // Solve the quadratic equation
    // https://stackoverflow.com/questions/19175037/determine-a-b-c-of-quadratic-equation-using-data-points
    let a = points[0].y / ((points[0].x - points[1].x) * (points[0].x - points[2].x))
        + points[1].y / ((points[1].x - points[0].x) * (points[1].x - points[2].x))
        + points[2].y / ((points[2].x - points[0].x) * (points[2].x - points[1].x));

    let b = -points[0].y * (points[1].x + points[2].x)
        / ((points[0].x - points[1].x) * (points[0].x - points[2].x))
        - points[1].y * (points[0].x + points[2].x)
            / ((points[1].x - points[0].x) * (points[1].x - points[2].x))
        - points[2].y * (points[0].x + points[1].x)
            / ((points[2].x - points[0].x) * (points[2].x - points[1].x));

    let c = points[0].y * points[1].x * points[2].x
        / ((points[0].x - points[1].x) * (points[0].x - points[2].x))
        + points[1].y * points[0].x * points[2].x
            / ((points[1].x - points[0].x) * (points[1].x - points[2].x))
        + points[2].y * points[0].x * points[1].x
            / ((points[2].x - points[0].x) * (points[2].x - points[1].x));

    let target = target as i128;
    let result = (a as i128) * target * target + (b as i128) * target + (c as i128);

    Ok(result.to_string())
}
