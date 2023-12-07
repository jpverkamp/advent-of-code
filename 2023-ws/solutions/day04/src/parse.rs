use crate::types::Card;
use nom::{
    bytes::complete::*,
    character::complete,
    character::complete::{newline, space0, space1},
    multi::*,
    sequence::*,
    *,
};

pub fn cards(s: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(newline, card)(s)
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn card(s: &str) -> IResult<&str, Card> {
    let (s, _) = tag("Card")(s)?;
    let (s, _) = space1(s)?;
    let (s, id) = complete::u32(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, _) = space0(s)?;
    let (s, winning_numbers) = separated_list1(space1, complete::u8)(s)?;
    let (s, _) = delimited(space0, tag("|"), space0)(s)?;
    let (s, guesses) = separated_list1(space1, complete::u8)(s)?;

    Ok((
        s,
        Card {
            id,
            winning_numbers,
            guesses,
        },
    ))
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_card() {
        let input = "Card 1: 1 2 3 4 5 6 7 8 9 10 | 1 2 3 4 5 6 7 8 9 10";
        let (_, card) = crate::parse::card(input).unwrap();
        assert_eq!(card.id, 1);
        assert_eq!(card.winning_numbers, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(card.guesses, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_parse_card_whitepsace() {
        let input = "Card   1:  1  2  3  4  5  6  7  8  9 10 |  1  2  3  4  5  6  7  8  9 10";
        let (_, card) = crate::parse::card(input).unwrap();
        assert_eq!(card.id, 1);
        assert_eq!(card.winning_numbers, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(card.guesses, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}