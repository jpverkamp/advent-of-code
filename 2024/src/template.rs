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
    use super::*;


    const EXAMPLE: &str = "\
hello
world";

    #[test]
    fn part1_example() {
        assert_eq!(part1_v1(&parse(EXAMPLE)), "<RESULT>");
    }

    #[test]
    fn part1_final() {
        assert_eq!(part1_v1(&parse(include_str!("../input/2024/day6.txt"))), "<RESULT>");
    }
    

    #[test]
    fn part2_example() {
        assert_eq!(part2_v1(&parse(EXAMPLE)), "<RESULT>");
    }

    #[test]
    fn part2_final() {
        assert_eq!(part2_v1(&parse(include_str!("../input/2024/day6.txt"))), "<RESULT>");
    }
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_v1(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_v1(&parse(input)).to_string()
}