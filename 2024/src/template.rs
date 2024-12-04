use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day5, part1, original)]
fn part1_original(input: &str) -> String {
    todo!()
}

#[aoc(day5, part2, original)]
fn part2_original(input: &str) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_original(&parse(input))
}

pub fn part2(input: &str) -> String {
    part2_original(&parse(input))
}