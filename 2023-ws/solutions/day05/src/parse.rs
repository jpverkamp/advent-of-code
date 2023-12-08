use crate::types::{Category::*, *};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline, space0, space1},
    combinator::map,
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

fn category(s: &str) -> IResult<&str, Category> {
    alt((
        map(tag("seed"), |_| Seed),
        map(tag("soil"), |_| Soil),
        map(tag("fertilizer"), |_| Fertilizer),
        map(tag("water"), |_| Water),
        map(tag("light"), |_| Light),
        map(tag("temperature"), |_| Temperature),
        map(tag("humidity"), |_| Humidity),
        map(tag("location"), |_| Location),
    ))(s)
}

fn range_map(s: &str) -> IResult<&str, RangeMap> {
    let (s, (dst, src, len)) = tuple((
        complete::u64,
        preceded(space0, complete::u64),
        preceded(space0, complete::u64),
    ))(s)?;
    Ok((s, RangeMap { src, dst, len }))
}

fn category_map(s: &str) -> IResult<&str, CategoryMap> {
    let (s, (src_cat, dst_cat)) = separated_pair(
        category,
        tag("-to-"),
        terminated(category, terminated(preceded(space1, tag("map:")), newline)),
    )(s)?;
    let (s, range_maps) = separated_list1(newline, range_map)(s)?;

    Ok((
        s,
        CategoryMap {
            src_cat,
            dst_cat,
            range_maps,
        },
    ))
}

pub fn simulation(s: &str) -> IResult<&str, Simulation> {
    let (s, seeds) = delimited(
        preceded(tag("seeds:"), space1),
        separated_list1(space1, complete::u64),
        many1(newline),
    )(s)?;

    let (s, range_maps) = separated_list1(many1(newline), category_map)(s)?;
    let (s, _) = many0(newline)(s)?;

    Ok((
        s,
        Simulation {
            seeds,
            category_maps: range_maps,
        },
    ))
}
