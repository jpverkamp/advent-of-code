use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};

use crate::types::*;

fn equation(s: &str) -> IResult<&str, Equation> {
    let (s, terms) = separated_list1(space1, complete::i64)(s)?;
    Ok((s, Equation { terms }))
}

pub fn equations(s: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(newline, equation)(s)
}
