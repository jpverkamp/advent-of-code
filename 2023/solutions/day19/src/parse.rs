use crate::types::*;

use fxhash::FxHashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, char, line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

// A category for part ratings, always a literal char from 'xmas'
fn rating_category(input: &str) -> IResult<&str, RatingCategory> {
    alt((
        map(char('x'), |_| RatingCategory::X),
        map(char('m'), |_| RatingCategory::M),
        map(char('a'), |_| RatingCategory::A),
        map(char('s'), |_| RatingCategory::S),
    ))(input)
}

// A label for a rule or part, an alphabetic string or the literals A and R
fn label(input: &str) -> IResult<&str, Label> {
    alt((
        map(tag("in"), |_| Label::Input),
        map(char('A'), |_| Label::Accept),
        map(char('R'), |_| Label::Reject),
        map(alpha1, Label::Node),
    ))(input)
}

// Comparison operators
fn comparator(input: &str) -> IResult<&str, Comparator> {
    alt((
        map(char('<'), |_| Comparator::LessThan),
        map(char('>'), |_| Comparator::GreaterThan),
    ))(input)
}

// A comparison takes a rating category, comparator, and a value
fn comparison(input: &str) -> IResult<&str, Comparison> {
    let (input, (category, comparator, value, label)) = tuple((
        rating_category,
        comparator,
        complete::u64,
        preceded(char(':'), label),
    ))(input)?;

    Ok((
        input,
        Comparison {
            category,
            comparator,
            value,
            label,
        },
    ))
}

// A rule has a label, a list of comparisons, and a default if no comparison matches
fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, (label, comparisons, default)) = tuple((
        label,
        preceded(char('{'), separated_list1(char(','), comparison)),
        delimited(char(','), label, char('}')),
    ))(input)?;

    Ok((
        input,
        Rule {
            label,
            comparisons,
            default,
        },
    ))
}

// A part has a score for each of the four rating categories
// For now, assume they are ordered
fn part(input: &str) -> IResult<&str, Part> {
    let (input, (x, m, a, s)) = delimited(
        char('{'),
        tuple((
            preceded(tag("x="), complete::u64),
            preceded(tag(",m="), complete::u64),
            preceded(tag(",a="), complete::u64),
            preceded(tag(",s="), complete::u64),
        )),
        char('}'),
    )(input)?;

    Ok((input, Part { x, m, a, s }))
}

// A simulation contains a list of rules and a list of parts
pub fn simulation(input: &str) -> IResult<&str, (FxHashMap<Label, Rule>, Vec<Part>)> {
    let (input, (rules, parts)) = separated_pair(
        separated_list1(line_ending, rule),
        many1(line_ending),
        separated_list1(line_ending, part),
    )(input)?;

    let rules = rules
        .into_iter()
        .map(|r| (r.label, r))
        .collect::<FxHashMap<_, _>>();

    Ok((input, (rules, parts)))
}
