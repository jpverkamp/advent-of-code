use crate::types::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, char, line_ending, space1},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn condition(s: &str) -> IResult<&str, Condition> {
    alt((
        map(tag("#"), |_| Condition::Operational),
        map(tag("."), |_| Condition::Damaged),
        map(tag("?"), |_| Condition::Unknown),
    ))(s)
}

fn spring(s: &str) -> IResult<&str, Spring> {
    let (s, conditions) = many1(condition)(s)?;
    let (s, _) = space1(s)?;
    let (s, groups) = separated_list1(char(','), complete::u64)(s)?;

    Ok((s, Spring { conditions, groups }))
}

pub fn springs(s: &str) -> IResult<&str, Vec<Spring>> {
    separated_list1(line_ending, spring)(s)
}
