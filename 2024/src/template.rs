use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day5, part1, v1)]
fn part1_v1(input: &str) -> String {
    todo!()
}

#[aoc(day5, part2, v1)]
fn part2_v1(input: &str) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use crate::make_test;
    use super::*;


    const EXAMPLE: &str = "\
hello
world";

    make_test!([part1_v1] => "day5.txt", "example output", "final output");
    make_test!([part2_v1] => "day5.txt", "example output", "final output");
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_v1(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_v1(&parse(input)).to_string()
}