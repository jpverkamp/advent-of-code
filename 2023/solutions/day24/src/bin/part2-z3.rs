use anyhow::Result;
use std::io;
use z3::{
    ast::{Ast, Int},
    Config, Context, Solver,
};

use day24::parse;

// #[aoc_test("data/test/24.txt", "2")] // with first bounds
// #[aoc_test("data/24.txt", "")]
fn main() {
    env_logger::init();

    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, lines) = parse::lines(input).unwrap();
    assert!(s.trim().is_empty());

    // Make a giant system of equations
    // X + XD * [t] = [x] + [xd] * [t]

    // X and XD are scalars
    // [t] is a vector we're solving for
    // [x] and [xd] are the input vectors

    let config = Config::new();
    let context = Context::new(&config);
    let solver = Solver::new(&context);

    let origin_x = Int::new_const(&context, "ox");
    let origin_y = Int::new_const(&context, "oy");
    let origin_z = Int::new_const(&context, "oz");

    let direction_x = Int::new_const(&context, "dx");
    let direction_y = Int::new_const(&context, "dy");
    let direction_z = Int::new_const(&context, "dz");

    for line in lines {
        let line_origin_x = Int::from_i64(&context, line.origin.x as i64);
        let line_origin_y = Int::from_i64(&context, line.origin.y as i64);
        let line_origin_z = Int::from_i64(&context, line.origin.z as i64);

        let line_direction_x = Int::from_i64(&context, line.direction.x as i64);
        let line_direction_y = Int::from_i64(&context, line.direction.y as i64);
        let line_direction_z = Int::from_i64(&context, line.direction.z as i64);

        let t = Int::fresh_const(&context, "t");

        solver.assert(
            &(&line_origin_x + &line_direction_x * &t)._eq(&(&origin_x + &direction_x * &t)),
        );
        solver.assert(
            &(&line_origin_y + &line_direction_y * &t)._eq(&(&origin_y + &direction_y * &t)),
        );
        solver.assert(
            &(&line_origin_z + &line_direction_z * &t)._eq(&(&origin_z + &direction_z * &t)),
        );
    }

    solver.check();
    let model = solver.get_model().unwrap();

    let result_x = model.get_const_interp(&origin_x).unwrap().as_i64().unwrap();
    let result_y = model.get_const_interp(&origin_y).unwrap().as_i64().unwrap();
    let result_z = model.get_const_interp(&origin_z).unwrap().as_i64().unwrap();

    let result_dx = model
        .get_const_interp(&direction_x)
        .unwrap()
        .as_i64()
        .unwrap();
    let result_dy = model
        .get_const_interp(&direction_y)
        .unwrap()
        .as_i64()
        .unwrap();
    let result_dz = model
        .get_const_interp(&direction_z)
        .unwrap()
        .as_i64()
        .unwrap();

    log::info!(
        "result: {:?} + {:?}",
        (result_x, result_y, result_z),
        (result_dx, result_dy, result_dz),
    );

    let result = result_x + result_y + result_z;

    Ok(format!("{result:?}"))
}
