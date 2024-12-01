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

#[aoc(day1, part1, i32)]
pub fn part1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    // Unfortunate, but we do need a separate copy since we're going to sort them
    let mut ls1 = input.0.to_vec();
    let mut ls2 = input.1.to_vec();

    ls1.sort();
    ls2.sort();

    ls1.iter()
        .zip(ls2.iter())
        .map(|(v1, v2)| (v1 - v2).abs())
        .sum::<i32>()
}

#[aoc(day1, part2, i32)]
pub fn part2(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    input.0.iter()
        .map(|v1| input.1.iter().filter(|v2| v1 == *v2).count() as i32 * v1)
        .sum::<i32>()
}
