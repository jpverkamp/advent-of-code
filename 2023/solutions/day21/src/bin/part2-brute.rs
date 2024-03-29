use anyhow::Result;
use fxhash::FxHashSet;
use std::io;

use day21::parse;

use bounds::Bounds;
use point::Point;

const STEPS: i32 = 100;

// #[aoc_test("data/test/21.txt", "16")]        // if steps = 6
// #[aoc_test("data/test/21.txt", "50")]        // if steps = 10
// #[aoc_test("data/test/21.txt", "1594")]      // if steps = 50
// #[aoc_test("data/test/21.txt", "6536")]      // if steps = 100
// #[aoc_test("data/test/21.txt", "167004")]    // if steps = 500
// #[aoc_test("data/test/21.txt", "668697")]    // if steps = 1000
// #[aoc_test("data/test/21.txt", "16733044")]  // if steps = 5000
// #[aoc_test("data/21.txt", "")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;

    let (walls, start) = parse::read(&input);
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

        println!("{_i} {}", active.len());

        #[cfg(debug_assertions)]
        {
            println!("=== {_i} ===");
            let bounds = wall_bounds + Bounds::from(active.iter());

            for y in bounds.min_y..=bounds.max_y {
                for x in bounds.min_x..=bounds.max_x {
                    let p = Point::new(x, y);
                    if active.contains(&p) {
                        print!("O");
                    } else if wall_mod_contains(&p) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }
    }

    let result = active.len();

    println!("{result}");
    Ok(())
}
