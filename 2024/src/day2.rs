use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

// Initial version takes in a vector and checks for 'safeness'
// Must be either strictly increasing or decreasing with no difference greater than 3
fn safe(report: &[i32]) -> bool {
    (report.is_sorted() || report.iter().rev().is_sorted())
        && report
            .iter()
            .zip(report.iter().skip(1))
            .all(|(a, b)| a != b && (a - b).abs() <= 3)
}

#[aoc(day2, part1, initial)]
fn part1_initial(input: &[Vec<i32>]) -> usize {
    input.iter().filter(|report| safe(report)).count()
}

#[aoc(day2, part2, initial)]
fn part2_initial(input: &[Vec<i32>]) -> usize {
    input
        .iter()
        .filter(|report| {
            for n in 0..report.len() {
                let mut sub_report = (*report).clone();
                sub_report.remove(n);
                if safe(&sub_report) {
                    return true;
                }
            }
            false
        })
        .count()
}

// Optimized version that takes in a reversible iterator and does the same
// This will allow us to skip values in the middle of the list
// And because we're only cloning the iter (not the entire vec) can be faster for part 2
fn safe_iter<'a, I>(report_iter: I) -> bool
where
    I: DoubleEndedIterator<Item = &'a i32> + Clone,
{
    (report_iter.clone().is_sorted() || report_iter.clone().rev().is_sorted())
        && report_iter
            .clone()
            .zip(report_iter.clone().skip(1))
            .all(|(a, b)| a != b && (a - b).abs() <= 3)
}

#[aoc(day2, part1, iterator)]
fn part1_iter(input: &[Vec<i32>]) -> usize {
    input
        .iter()
        .filter(|report| safe_iter(report.iter()))
        .count()
}

#[aoc(day2, part2, iterator)]
fn part2_iter(input: &[Vec<i32>]) -> usize {
    input
        .iter()
        .filter(|report| {
            (0..report.len())
                .any(|n| safe_iter(report.iter().take(n).chain(report.iter().skip(n + 1))))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn parse_example() {
        let input = parse(EXAMPLE);
        assert_eq!(
            input,
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]
        );
    }

    make_test!([part1_initial, part1_iter] => "day2.txt", 2, 432);
    make_test!([part2_initial, part2_iter] => "day2.txt", 4, 488);
}
