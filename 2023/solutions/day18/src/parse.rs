use crate::types::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, char, hex_digit1, multispace1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

// Parse #rrggbb into Color { r, g, b }
fn hexcolor(input: &str) -> IResult<&str, HexColor> {
    preceded(
        char('#'),
        map_res(hex_digit1, |s: &str| {
            if s.len() == 6 {
                Ok(HexColor {
                    red: u8::from_str_radix(&s[0..2], 16).unwrap(),
                    green: u8::from_str_radix(&s[2..4], 16).unwrap(),
                    blue: u8::from_str_radix(&s[4..6], 16).unwrap(),
                })
            } else {
                Err("invalid hex color")
            }
        }),
    )(input)
}

fn direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("U"), |_| Direction::Up),
        map(tag("D"), |_| Direction::Down),
        map(tag("L"), |_| Direction::Left),
        map(tag("R"), |_| Direction::Right),
    ))(input)
}

// Parse a line into a command
// Lines look like R 5 (#rrggbb)
fn command(input: &str) -> IResult<&str, Command> {
    let (input, (direction, steps, color)) = tuple((
        direction,
        preceded(multispace1, complete::u64),
        preceded(multispace1, delimited(tag("("), hexcolor, tag(")"))),
    ))(input)?;

    Ok((
        input,
        Command {
            direction,
            steps,
            color,
        },
    ))
}

// Parse many commands, newline delimited
pub fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(newline, command)(input)
}
