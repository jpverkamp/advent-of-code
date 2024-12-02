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

#[aoc(day1, part1, i32)]
pub fn part1(input: &str) -> i32 {
    let (mut ls1, mut ls2) = parse(input);

    ls1.sort();
    ls2.sort();

    ls1.iter()
        .zip(ls2.iter())
        .map(|(v1, v2)| (v1 - v2).abs())
        .sum::<i32>()
}

#[aoc(day1, part2, i32)]
pub fn part2(input: &str) -> i32 {
    let (ls1, ls2) = parse(input);

    ls1.iter()
        .map(|v1| ls2.iter().filter(|v2| v1 == *v2).count() as i32 * v1)
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn parse_example() {
        let input = parse(TEST_INPUT);
        assert_eq!(input, (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]));
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&TEST_INPUT), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&TEST_INPUT), 31);
    }
}
