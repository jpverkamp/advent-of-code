use anyhow::Result;
use aoc::*;
use std::{fs::read_to_string, path::Path};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Card {
    pub id: u32,
    pub winning_numbers: Vec<u8>,
    pub guesses: Vec<u8>,
}

impl Card {
    fn matches(&self) -> usize {
        self.guesses
            .iter()
            .filter(|guess: &&u8| self.winning_numbers.contains(guess))
            .count()
    }
}

mod parse {
    use crate::*;
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
}

fn part1(filename: &Path) -> Result<String> {
    let input = read_to_string(filename)?;
    let (_, cards) = parse::cards(&input).unwrap();

    // Wrapper to avoid calculating 2^(-1) or 2^(usize::MAX)
    fn score(matches: usize) -> usize {
        if matches == 0 {
            0
        } else {
            2_usize.pow((matches - 1) as u32)
        }
    }

    Ok(cards
        .iter()
        .map(|card| score(card.matches()))
        .sum::<usize>()
        .to_string())
}

fn part2(filename: &Path) -> Result<String> {
    let input = read_to_string(filename)?;
    let (_, cards) = parse::cards(&input).unwrap();

    let mut total = 0;
    let mut counts = vec![1; cards.len()];
    let mut next_counts = vec![0; cards.len()];

    // Earn new cards until stable
    loop {
        // Count all cards earned before updating
        total += counts.iter().sum::<usize>();

        // Each card earns
        // NOTE: We're explicitly guaranteed that next_counts[i + j + 1] doesn't overflow
        for (i, card) in cards.iter().enumerate() {
            for j in 0..card.matches() {
                next_counts[i + j + 1] += counts[i];
            }
        }

        // If no cards were earned, we're done
        if next_counts.iter().all(|&c| c == 0) {
            break;
        }

        // Swap buffers and clear
        // This could be a std::mem::swap, but we'd still need to init the new next_counts
        for i in 0..cards.len() {
            counts[i] = next_counts[i];
            next_counts[i] = 0;
        }
    }

    Ok(total.to_string())
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc::aoc_test;

    #[test]
    fn test1() {
        aoc_test("test/04", part1, "13");
        aoc_test("04", part1, "23028");
    }

    #[test]
    fn test2() {
        aoc_test("test/04", part2, "30");
        aoc_test("04", part2, "9236992");
    }
}
