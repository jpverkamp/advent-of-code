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
fn part1(input: &str) -> i32 {
    let (mut ls1, mut ls2) = parse(input);

    ls1.sort();
    ls2.sort();

    ls1.iter()
        .zip(ls2.iter())
        .map(|(v1, v2)| (v1 - v2).abs())
        .sum::<i32>()
}

#[aoc(day1, part2, i32)]
fn part2(input: &str) -> i32 {
    let (ls1, ls2) = parse(input);

    ls1.into_iter()
        .map(|v1| ls2.iter().filter(|v2| v1 == **v2).count() as i32 * v1)
        .sum::<i32>()
}
