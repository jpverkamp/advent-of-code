use anyhow::Result;
use fxhash::FxHashSet;
use std::io;

use day21::parse;

use bounds::Bounds;
#[allow(unused_imports)]
use point::Point;

const STEPS: i32 = 64;

// aoc_test::generate!{day21_part1_test_21 as "test/21.txt" => "16")]    // if steps = 6
aoc_test::generate!{day21_part1_test_21 as "test/21.txt" => "4056"}
aoc_test::generate!{day21_part1_21 as "21.txt" => "3649"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (walls, start) = parse::read(input);

    #[allow(unused_variables)]
    let wall_bounds = Bounds::from(walls.iter());

    let mut active = FxHashSet::default();
    active.insert(start);

    for _i in 1..=STEPS {
        let mut next_active = FxHashSet::default();

        for pos in active {
            for neighbor in pos.neighbors() {
                if !walls.contains(&neighbor) {
                    next_active.insert(neighbor);
                }
            }
        }

        active = next_active;

        // {
        //     println!("=== {_i} ===");
        //     let bounds = wall_bounds + Bounds::from(active.iter());
        //     for y in bounds.min_y..=bounds.max_y {
        //         for x in bounds.min_x..=bounds.max_x {
        //             let p = Point::new(x, y);
        //             if active.contains(&p) {
        //                 print!("O");
        //             } else if walls.contains(&p) {
        //                 print!("#");
        //             } else {
        //                 print!(".");
        //             }
        //         }
        //         println!();
        //     }
        //     println!();
        // }
    }

    Ok(active.len().to_string())
}
