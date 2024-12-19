use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|lss| {
            assert!(lss.len() == 2);
            (lss[0], lss[1])
        })
        .unzip()
}

#[aoc(day1, part1)]
fn part1_original(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut ls1 = input.0.clone();
    let mut ls2 = input.1.clone();

    ls1.sort();
    ls2.sort();

    ls1.iter()
        .zip(ls2.iter())
        .map(|(v1, v2)| (v1 - v2).abs())
        .sum::<i32>()
}

#[aoc(day1, part2)]
fn part2_original(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    input
        .0
        .iter()
        .map(|v1| input.1.iter().filter(|v2| v1 == *v2).count() as i32 * v1)
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn parse_example() {
        let input = parse(EXAMPLE);
        assert_eq!(input, (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]));
    }

    make_test!([part1_original] => "day1.txt", 11, 2742123);
    make_test!([part2_original] => "day1.txt", 31, 21328497);
}
