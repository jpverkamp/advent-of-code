pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

mod grid;
use grid::Grid;

mod direction;
use direction::Direction;

mod point;
use point::Point;

mod make_test;

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

aoc_lib! { year = 2024 }
