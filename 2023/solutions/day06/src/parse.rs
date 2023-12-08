use crate::types::*;
use nom::{
    bytes::complete::tag,
    character::complete,
    character::complete::{newline, space1},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    *,
};

pub fn races(s: &str) -> IResult<&str, Vec<Race>> {
    let (s, times) = delimited(
        tuple((tag("Time:"), space1)),
        separated_list1(space1, complete::u64),
        newline,
    )(s)?;
    let (s, records) = preceded(
        tuple((tag("Distance:"), space1)),
        separated_list1(space1, complete::u64),
    )(s)?;

    Ok((
        s,
        times
            .into_iter()
            .zip(records)
            .map(|(time, record)| Race { time, record })
            .collect::<Vec<_>>(),
    ))
}
