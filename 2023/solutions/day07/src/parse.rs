use crate::types::*;
use nom::{
    character::complete::{self, anychar, newline, space1},
    multi::{count, separated_list1},
    *,
};

fn hand(s: &str) -> IResult<&str, Hand> {
    let (s, cards) = count(anychar, 5)(s)?;
    let cards = cards.iter().map(|c| Card::from(*c)).collect::<Vec<_>>();
    let (s, _) = space1(s)?;
    let (s, bid) = complete::u64(s)?;

    Ok((
        s,
        Hand {
            cards: cards.try_into().unwrap(),
            bid,
        },
    ))
}

pub fn hands(s: &str) -> IResult<&str, Vec<Hand>> {
    let (s, hands) = separated_list1(newline, hand)(s)?;
    Ok((s, hands))
}
