use crate::types::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    *,
};

fn moves(s: &str) -> IResult<&str, Vec<Move>> {
    let (s, moves) = many1(alt((
        map(tag("L"), |_| Move::Left),
        map(tag("R"), |_| Move::Right),
    )))(s)?;
    Ok((s, moves))
}

fn label(s: &str) -> IResult<&str, Label> {
    let (s, label) = tuple((anychar, anychar, anychar))(s)?;
    Ok((s, label.into()))
}

fn mapping(s: &str) -> IResult<&str, (Label, Neighbors)> {
    let (s, (label, left, right)) = tuple((
        label,
        preceded(tag(" = ("), label),
        terminated(preceded(tag(", "), label), tag(")")),
    ))(s)?;

    Ok((s, (label, Neighbors { left, right })))
}

pub fn simulation(s: &str) -> IResult<&str, Simulation> {
    let (s, moves) = moves(s)?;
    let (s, _) = newline(s)?;
    let (s, _) = newline(s)?;
    let (s, neighbors) = separated_list1(newline, mapping)(s)?;
    Ok((
        s,
        Simulation {
            moves,
            neighbors: neighbors.into_iter().collect(),
        },
    ))
}
