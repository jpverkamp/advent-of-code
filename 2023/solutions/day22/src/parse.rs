use crate::types::*;

use nom::{
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

fn point(input: &str) -> IResult<&str, Point> {
    map(
        tuple((
            complete::i64,
            delimited(complete::char(','), complete::i64, complete::char(',')),
            complete::i64,
        )),
        |(x, y, z)| Point::new(x as isize, y as isize, z as isize),
    )(input)
}

fn block(input: &str) -> IResult<&str, Block> {
    map(
        separated_pair(point, complete::char('~'), point),
        |(min, max)| Block::new(min, max),
    )(input)
}

pub fn blocks(input: &str) -> IResult<&str, Vec<Block>> {
    separated_list1(line_ending, block)(input)
}
