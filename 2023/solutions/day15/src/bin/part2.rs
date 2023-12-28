use anyhow::Result;
use std::io;

aoc_test::generate!{day15_part2_test_15 as "test/15.txt" => "145"}
aoc_test::generate!{day15_part2_15 as "15.txt" => "265462"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    fn hash(s: &str) -> u8 {
        s.chars()
            .fold(0, |v, c| ((v.wrapping_add(c as u8)).wrapping_mul(17)))
    }

    let mut boxes: Vec<Vec<(&str, u8)>> = (0..256).map(|_| vec![]).collect::<Vec<_>>();

    input.split(',').for_each(|s| {
        if s.ends_with('-') {
            // Remove (if exists)
            let s = s.strip_suffix('-').unwrap();
            let k = hash(s) as usize;
            if let Some(i) = boxes[k].iter().position(|b| b.0 == s) {
                boxes[k].remove(i);
            }
        } else {
            // Assuming the only other case is =
            // Update (if exists) or insert (if not)
            let (s, v) = s.split_once('=').unwrap();
            let k = hash(s) as usize;
            let v = v.parse::<u8>().unwrap();

            if let Some(i) = boxes[k].iter().position(|b| b.0 == s) {
                boxes[k][i] = (s, v);
            } else {
                boxes[k].push((s, v));
            }
        }
    });

    Ok(boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, (_, v))| (i + 1) * (j + 1) * (*v as usize))
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string())
}
