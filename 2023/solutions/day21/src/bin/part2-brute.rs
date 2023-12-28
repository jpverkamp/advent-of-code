use anyhow::Result;
use fxhash::FxHashSet;
use std::io;

use day21::parse;

use bounds::Bounds;
use point::Point;

const STEPS: i32 = 100;

// aoc_test::generate!{day21_part2_brute_test_21 as "test/21.txt" => "16733044"}
// aoc_test::generate!{day21_part2_brute_21 as "21.txt" => ""}

fn main() {
    env_logger::init();
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (walls, start) = parse::read(input);
    let wall_bounds = Bounds::from(walls.iter());

    // Note: Assuming min bounds are 0
    let width = wall_bounds.max_x + 2;
    let height = wall_bounds.max_y + 2;

    // A modular wall function
    let wall_mod_contains = |&p: &Point| {
        let mut p = Point::new(p.x % width, p.y % height);

        if p.x < 0 {
            p.x += width;
        }
        if p.y < 0 {
            p.y += height;
        }

        walls.contains(&p)
    };

    let mut active = FxHashSet::default();
    active.insert(start);

    for _i in 1..=STEPS {
        let mut next_active = FxHashSet::default();

        for pos in active {
            for neighbor in pos.neighbors() {
                if !wall_mod_contains(&neighbor) {
                    next_active.insert(neighbor);
                }
            }
        }

        active = next_active;

        log::info!("{_i} {}", active.len());

        #[cfg(debug_assertions)]
        {
            let mut s = String::new();

            s.push_str("=== {_i} ===");
            let bounds = wall_bounds + Bounds::from(active.iter());

            for y in bounds.min_y..=bounds.max_y {
                for x in bounds.min_x..=bounds.max_x {
                    let p = Point::new(x, y);
                    if active.contains(&p) {
                        s.push('O');
                    } else if wall_mod_contains(&p) {
                        s.push('#');
                    } else {
                        s.push('.');
                    }
                }
                s.push('\n');
            }
            log::info!("{s}");
        }
    }

    Ok(active.len().to_string())
}
