use aoc_runner_derive::aoc;

use regex::Regex;

use nom::{
    bytes::complete::tag,
    character::complete,
    character::complete::anychar,
    multi::{many1, many_till},
    sequence::delimited,
    sequence::separated_pair,
};

#[aoc(day3, part1, regex)]
fn part1_regex(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
        .sum::<u32>()
}

#[aoc(day3, part1, nom)]
fn part1_nom(input: &str) -> u32 {
    many1(many_till(
        anychar::<_, (_, nom::error::ErrorKind)>,
        delimited(
            tag("mul("),
            separated_pair(complete::u32, tag(","), complete::u32),
            tag(")"),
        ),
    ))(input)
    .unwrap()
    .1
    .iter()
    .map(|(_, (a, b))| a * b)
    .sum()
}

#[aoc(day3, part1, iterator)]
fn part1_iterator(input: &str) -> u32 {
    let input = input.chars().collect::<Vec<_>>();

    #[derive(Debug)]
    enum State {
        Scanning,
        ReadingA,
        ReadingB,
    }

    let mut sum = 0;
    let mut a = 0;
    let mut b = 0;

    let mut state = State::Scanning;
    let mut index = 0;

    loop {
        match state {
            State::Scanning => {
                if input[index..].starts_with(&['m', 'u', 'l', '(']) {
                    state = State::ReadingA;
                    a = 0;
                    b = 0;
                    index += 3;
                }
            }
            State::ReadingA => {
                if input[index] == ',' {
                    state = State::ReadingB;
                } else if input[index].is_ascii_digit() {
                    a = a * 10 + input[index] as u32 - '0' as u32;
                } else {
                    state = State::Scanning;
                    index -= 1; // Recheck incase we start another mul
                }
            }
            State::ReadingB => {
                if input[index] == ')' {
                    sum += a * b;
                    state = State::Scanning;
                } else if input[index].is_ascii_digit() {
                    b = b * 10 + input[index] as u32 - '0' as u32;
                } else {
                    state = State::Scanning;
                    index -= 1; // Recheck incase we start another mul
                }
            }
        }

        index += 1;
        if index >= input.len() {
            break;
        }
    }

    sum
}

#[aoc(day3, part2, regex)]
fn part2_regex(input: &str) -> u32 {
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

#[aoc(day3, part2, iterator)]
fn part2_iterator(input: &str) -> u32 {
    let input = input.chars().collect::<Vec<_>>();

    #[derive(Debug)]
    enum State {
        Scanning,
        Disabled,
        ReadingA,
        ReadingB,
    }

    let mut sum = 0;
    let mut a = 0;
    let mut b = 0;

    let mut state = State::Scanning;
    let mut index = 0;

    loop {
        match state {
            State::Scanning => {
                if input[index..].starts_with(&['m', 'u', 'l', '(']) {
                    state = State::ReadingA;
                    a = 0;
                    b = 0;
                    index += 3;
                } else if input[index..].starts_with(&['d', 'o', 'n', '\'', 't', '(', ')']) {
                    state = State::Disabled;
                    index += 6;
                }
            }
            State::Disabled => {
                if input[index..].starts_with(&['d', 'o', '(', ')']) {
                    state = State::Scanning;
                    index += 3;
                }
            }
            State::ReadingA => {
                if input[index] == ',' {
                    state = State::ReadingB;
                } else if input[index].is_ascii_digit() {
                    a = a * 10 + input[index] as u32 - '0' as u32;
                } else {
                    state = State::Scanning;
                    index -= 1; // Recheck incase we start another mul
                }
            }
            State::ReadingB => {
                if input[index] == ')' {
                    sum += a * b;
                    state = State::Scanning;
                } else if input[index].is_ascii_digit() {
                    b = b * 10 + input[index] as u32 - '0' as u32;
                } else {
                    state = State::Scanning;
                    index -= 1; // Recheck incase we start another mul
                }
            }
        }

        index += 1;
        if index >= input.len() {
            break;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_regex_example() {
        assert_eq!(
            part1_regex("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn part1_iterator_example() {
        assert_eq!(
            part1_iterator("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn part2_regex_example() {
        assert_eq!(
            part2_regex(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            48
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2_iterator("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
