use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Equation {
    result: u64,
    input: Vec<u64>,
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Equation> {
    use nom::{
        character::complete::{self, newline, space1},
        multi::separated_list1,
        sequence::{preceded, separated_pair},
    };

    separated_list1(
        newline::<_, (_, nom::error::ErrorKind)>,
        separated_pair(
            complete::u64,
            complete::char(':'),
            preceded(space1, separated_list1(space1, complete::u64)),
        ),
    )(input)
    .expect("Failed to parse")
    .1
    .into_iter()
    .map(|(result, input)| Equation { result, input })
    .collect()
}

// Original, direct version

#[aoc(day7, part1, v1)]
fn part1_v1(input: &[Equation]) -> u64 {
    fn is_solvable(target: u64, acc: u64, values: &[u64]) -> bool {
        if values.is_empty() {
            return target == acc;
        }

        is_solvable(target, acc + values[0], &values[1..])
            || is_solvable(target, acc * values[0], &values[1..])
    }

    input
        .iter()
        .filter(|eq| is_solvable(eq.result, 0, &eq.input))
        .map(|eq| eq.result)
        .sum::<u64>()
}

#[aoc(day7, part2, v1)]
fn part2_v1(input: &[Equation]) -> u64 {
    fn is_solvable(target: u64, acc: u64, values: &[u64]) -> bool {
        if values.is_empty() {
            return target == acc;
        }

        is_solvable(target, acc + values[0], &values[1..])
            || is_solvable(target, acc * values[0], &values[1..])
            || {
                let digits = values[0].ilog10() + 1;
                let multiplier = 10_u64.pow(digits);
                is_solvable(target, acc * multiplier + values[0], &values[1..])
            }
    }

    input
        .iter()
        .filter(|eq| is_solvable(eq.result, 0, &eq.input))
        .map(|eq| eq.result)
        .sum::<u64>()
}

// Direct version with an explicit queue instead of recursion

#[aoc(day7, part1, queue)]
fn part1_queue(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|eq| {
            let mut queue = Vec::with_capacity(2_usize.pow(eq.input.len() as u32));
            queue.push((eq.result, 0, eq.input.as_slice()));

            while let Some((target, acc, values)) = queue.pop() {
                if values.is_empty() {
                    if target == acc {
                        return true;
                    }
                } else {
                    queue.push((target, acc + values[0], &values[1..]));
                    queue.push((target, acc * values[0], &values[1..]));
                }
            }

            false
        })
        .map(|eq| eq.result)
        .sum::<u64>()
}

#[aoc(day7, part2, queue)]
fn part2_queue(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|eq| {
            let mut queue = Vec::with_capacity(3_usize.pow(eq.input.len() as u32));
            queue.push((eq.result, 0, eq.input.as_slice()));

            while let Some((target, acc, values)) = queue.pop() {
                if values.is_empty() {
                    if target == acc {
                        return true;
                    }
                } else {
                    queue.push((target, acc + values[0], &values[1..]));
                    queue.push((target, acc * values[0], &values[1..]));
                    queue.push((
                        target,
                        {
                            let digits = values[0].ilog10() + 1;
                            10_u64.pow(digits) * acc + values[0]
                        },
                        &values[1..],
                    ));
                }
            }

            false
        })
        .map(|eq| eq.result)
        .sum::<u64>()
}

// A 'cleaner' version using a struct, but it's slower

struct OpSet {
    ops: Vec<fn(u64, u64) -> u64>,
}

impl OpSet {
    fn new() -> Self {
        Self { ops: vec![] }
    }

    fn include(&mut self, op: fn(u64, u64) -> u64) {
        self.ops.push(op);
    }

    fn can_solve(&self, target: u64, args: &[u64]) -> bool {
        fn recur(me: &OpSet, target: u64, acc: u64, args: &[u64]) -> bool {
            if args.is_empty() {
                return target == acc;
            }

            me.ops
                .iter()
                .any(|op| recur(me, target, op(acc, args[0]), &args[1..]))
        }

        recur(self, target, 0, args)
    }
}

#[aoc(day7, part1, opset)]
fn part1_opset(input: &[Equation]) -> u64 {
    let mut op_set = OpSet::new();
    op_set.include(|a, b| a + b);
    op_set.include(|a, b| a * b);

    input
        .iter()
        .filter(|eq| op_set.can_solve(eq.result, &eq.input))
        .map(|eq| eq.result)
        .sum::<u64>()
}

#[aoc(day7, part2, opset)]
fn part2_opset(input: &[Equation]) -> u64 {
    let mut op_set = OpSet::new();
    op_set.include(|a, b| a + b);
    op_set.include(|a, b| a * b);
    op_set.include(|a, b| {
        let digits = b.ilog10() + 1;
        10_u64.pow(digits) * a + b
    });

    input
        .iter()
        .filter(|eq| op_set.can_solve(eq.result, &eq.input))
        .map(|eq| eq.result)
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    make_test!([part1_v1, part1_opset, part1_queue] => "day7.txt", 3749, "975671981569");
    make_test!([part2_v1, part2_opset, part2_queue] => "day7.txt", 11387, "223472064194845");
    // 975671981569 too low
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_v1(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_v1(&parse(input)).to_string()
}
