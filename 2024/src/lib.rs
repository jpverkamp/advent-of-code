pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

mod grid;
pub use grid::Grid;

mod direction;
pub use direction::Direction;

mod point;
pub use point::Point;

mod make_test;

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

aoc_lib! { year = 2024 }
