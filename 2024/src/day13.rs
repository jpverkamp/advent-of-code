use aoc_runner_derive::{aoc, aoc_generator};

use crate::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ClawMachine {
    a: Point,
    b: Point,
    p: Point,
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<ClawMachine> {
    let mut lines = input.lines();
    let mut machines = vec![];

    fn parse_equation(s: &str) -> Point {
        let (_, input) = s.split_once("X").unwrap();
        let (xs, ys) = input.split_once("Y").unwrap();

        let x = xs[1..(xs.len() - 2)]
            .parse::<i32>()
            .expect("failed to parse x part");
        let y = ys[1..].parse::<i32>().expect("failed to parse y part");

        (x, y).into()
    }

    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }

        let a = parse_equation(line.unwrap());
        let b = parse_equation(lines.next().unwrap());
        let p = parse_equation(lines.next().unwrap());

        machines.push(ClawMachine { a, b, p });

        // Empty line or end of file
        if lines.next().is_none() {
            break;
        }
    }

    machines
}

#[aoc(day13, part1, bruteforce)]
fn part1_bruteforce(input: &[ClawMachine]) -> i32 {
    let mut tokens = 0;

    for machine in input {
        for a_presses in 0..=100 {
            let after_a = machine.p - machine.a * a_presses;

            if after_a.x % machine.b.x == 0
                && after_a.y % machine.b.y == 0
                && after_a.x / machine.b.x == after_a.y / machine.b.y
            {
                let b_presses = after_a.x / machine.b.x;

                tokens += a_presses * 3 + b_presses;
                break;
            }
        }
    }

    tokens
}

fn cramer_integer_solve(
    ax: i128,
    ay: i128,
    bx: i128,
    by: i128,
    px: i128,
    py: i128,
) -> Option<(i128, i128)> {
    let det = ax * by - ay * bx;
    let det_sub_a = px * by - py * bx;
    let det_sub_b = ax * py - ay * px;

    let a = det_sub_a / det;
    let b = det_sub_b / det;

    if det_sub_a % det == 0 && det_sub_b % det == 0 {
        Some((a, b))
    } else {
        None
    }
}

#[aoc(day13, part1, cramer)]
fn part1_cramer(input: &[ClawMachine]) -> u128 {
    let mut tokens = 0;

    for machine in input {
        if let Some((a_presses, b_presses)) = cramer_integer_solve(
            machine.a.x as i128,
            machine.a.y as i128,
            machine.b.x as i128,
            machine.b.y as i128,
            machine.p.x as i128,
            machine.p.y as i128,
        ) {
            if a_presses >= 0 && b_presses >= 0 {
                tokens += a_presses as u128 * 3 + b_presses as u128;
            }
        }
    }

    tokens
}

#[aoc(day13, part2, cramer)]
fn part2_cramer(input: &[ClawMachine]) -> u128 {
    let mut tokens = 0;

    for machine in input {
        if let Some((a_presses, b_presses)) = cramer_integer_solve(
            machine.a.x as i128,
            machine.a.y as i128,
            machine.b.x as i128,
            machine.b.y as i128,
            machine.p.x as i128 + 10000000000000,
            machine.p.y as i128 + 10000000000000,
        ) {
            if a_presses >= 0 && b_presses >= 0 {
                tokens += a_presses as u128 * 3 + b_presses as u128;
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    make_test!([part1_bruteforce, part1_cramer] => "day13.txt", 480, 26810);
    make_test!([part2_cramer] => "day13.txt", "875318608908", "108713182988244");
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_cramer(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_cramer(&parse(input)).to_string()
}
