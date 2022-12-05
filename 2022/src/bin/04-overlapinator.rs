use std::path::Path;
use aoc::*;

struct Span {
    min: isize,
    max: isize,
}

impl Span {
    fn new(s: &str) -> Span {
        let (min, max) = s.split_at(s.find("-").expect("missing dash in span"));

        Span {
            min: min.parse::<isize>().expect("min is not an integer"),
            max: max.strip_prefix("-").expect("malformed prefix, missing dash").parse::<isize>().expect("max is not an integer"),
        }
    }

    fn contains(&self, other: &Span) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &Span) -> bool {
        (self.min >= other.min && self.min <= other.max) 
            || (self.max >= other.min && self.max <= other.max)
            || (other.max >= self.min && other.max <= self.max)
            || (other.max >= self.min && other.max <= self.max)
    }
}

fn parse(lines: &Vec<String>) -> Vec<(Span, Span)> {
    let mut result = Vec::new();

    for line in lines {
        let (left, right) = line.split_at(line.find(",").expect("missing comma in line"));
        result.push((Span::new(left), Span::new(right.strip_prefix(",").expect("malformed prefix, missing comma"))))
    }

    result
}

fn part1(filename: &Path) -> String {
    let span_pairs = parse(&read_lines(filename));

    span_pairs.iter().filter(
        |pair| pair.0.contains(&pair.1) || pair.1.contains(&pair.0)
    ).count().to_string()
}

fn part2(filename: &Path) -> String {
    let span_pairs = parse(&read_lines(filename));

    span_pairs.iter().filter(
        |pair| pair.0.overlaps(&pair.1)
    ).count().to_string()
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use aoc::aoc_test;
    use crate::{part1, part2};


    #[test]   
    fn test1() { aoc_test("04", part1, "466") }

    #[test]
    fn test2() { aoc_test("04", part2, "865") }
}
