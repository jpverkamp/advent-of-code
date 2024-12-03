use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
        .sum::<u32>()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    re.captures_iter(input)
        .fold((0, true), |(sum, capturing), cap| match &cap[0] {
            "do()" => (sum, true),
            "don't()" => (sum, false),
            _ if capturing => (
                sum + cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap(),
                capturing,
            ),
            _ => (sum, capturing),
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
