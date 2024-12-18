use anyhow::Result;
use std::io;

use day03::types::*;

aoc_test::generate!{day03_part2_test_03 as "test/03.txt" => "467835"}
aoc_test::generate!{day03_part2_03 as "03.txt" => "81166799"}

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let schematic = Schematic::from(input);

    Ok(schematic
        .symbols
        .iter()
        .filter(|s| s.value == '*')
        .map(|s| {
            schematic
                .numbers
                .iter()
                .filter_map(|n| {
                    if n.is_neighbor(s.x, s.y) {
                        Some(n.value)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .filter(|ratios| ratios.len() == 2)
        .map(|ratios| ratios[0] * ratios[1])
        .sum::<usize>()
        .to_string())
}
