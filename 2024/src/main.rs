extern crate aoc_runner;
extern crate aoc_runner_derive;

use aoc_runner_derive::aoc_main;

aoc_main! { lib = aoc2024 }

// fn main() {
//     // Enable tracing without timestamps
//     tracing_subscriber::fmt().without_time().init();

//     const INPUT: &str = "\
// r, wr, b, g, bwu, rb, gb, br

// brwrr
// bggr
// gbbr
// rrbgbr
// ubwu
// bwurrg
// brgr
// bbrgwb";

//     assert_eq!(aoc2024::day19::part2_split_memo(INPUT), 16);
// }
