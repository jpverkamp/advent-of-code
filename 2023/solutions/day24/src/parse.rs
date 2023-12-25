use crate::types::*;

use nom::{
    character::complete::{self, line_ending, space0},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

fn point(input: &str) -> IResult<&str, Point> {
    map(
        tuple((
            complete::i128,
            preceded(terminated(complete::char(','), space0), complete::i128),
            preceded(terminated(complete::char(','), space0), complete::i128),
        )),
        |(x, y, z)| Point {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        },
    )(input)
}

fn line(input: &str) -> IResult<&str, Line> {
    map(
        tuple((point, delimited(space0, complete::char('@'), space0), point)),
        |(origin, _, direction)| Line { origin, direction },
    )(input)
}

pub fn lines(input: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(line_ending, line)(input)
}
