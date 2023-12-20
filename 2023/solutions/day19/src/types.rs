use std::ops::RangeInclusive;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Label<'a> {
    Input,
    Accept,
    Reject,
    Node(&'a str),
}

#[derive(Debug, Copy, Clone)]
pub enum RatingCategory {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Copy, Clone)]
pub struct Part {
    pub x: u64,
    pub m: u64,
    pub a: u64,
    pub s: u64,
}

#[derive(Debug, Clone)]
pub struct RangedPart {
    pub x: RangeInclusive<u64>,
    pub m: RangeInclusive<u64>,
    pub a: RangeInclusive<u64>,
    pub s: RangeInclusive<u64>,
}

#[derive(Debug, Copy, Clone)]
pub enum Comparator {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Copy, Clone)]
pub struct Comparison<'a> {
    pub category: RatingCategory,
    pub comparator: Comparator,
    pub value: u64,
    pub label: Label<'a>,
}

#[derive(Debug)]
pub struct Rule<'a> {
    pub label: Label<'a>,
    pub comparisons: Vec<Comparison<'a>>,
    pub default: Label<'a>,
}
