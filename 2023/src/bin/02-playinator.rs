use anyhow::Result;
use aoc::*;
use std::{fs::read_to_string, path::Path};

// A game consists of an ID and some number of rounds each with some number of dice
#[derive(Debug, PartialEq)]
pub struct Game {
    id: u32,
    rounds: Vec<Round>,
}

// A single round can have some number each of red/green/blue dice
#[derive(Debug, PartialEq)]
pub struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

// Represents colors of dice
#[derive(Debug, PartialEq)]
pub enum Colors {
    Red,
    Green,
    Blue,
}

mod parse {
    use crate::*;
    use nom::{
        branch::*, bytes::complete::*, character::complete, character::complete::newline,
        combinator::*, multi::*, sequence::*, *,
    };

    pub fn games(s: &str) -> IResult<&str, Vec<Game>> {
        separated_list1(newline, game)(s)
    }

    fn game(s: &str) -> IResult<&str, Game> {
        let (s, _) = tag("Game ")(s)?;
        let (s, id) = complete::u32(s)?;
        let (s, _) = tag(": ")(s)?;

        let (s, rounds) = separated_list1(tag("; "), round)(s)?;

        Ok((s, Game { id, rounds }))
    }

    fn round(s: &str) -> IResult<&str, Round> {
        let (s, counts) = separated_list1(
            tag(", "),
            sequence::tuple((complete::u32, preceded(tag(" "), color))),
        )(s)?;

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for (count, color) in counts {
            match color {
                Colors::Red => red = count,
                Colors::Green => green = count,
                Colors::Blue => blue = count,
            }
        }

        Ok((s, Round { red, green, blue }))
    }

    fn color(s: &str) -> IResult<&str, Colors> {
        alt((
            map(tag("red"), |_| Colors::Red),
            map(tag("green"), |_| Colors::Green),
            map(tag("blue"), |_| Colors::Blue),
        ))(s)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_color() {
            assert_eq!(color("red"), Ok(("", Colors::Red)));
            assert_eq!(color("green"), Ok(("", Colors::Green)));
            assert_eq!(color("blue"), Ok(("", Colors::Blue)));
        }

        #[test]
        fn test_round() {
            assert_eq!(
                round("3 blue, 4 red"),
                Ok((
                    "",
                    Round {
                        red: 4,
                        green: 0,
                        blue: 3
                    }
                ))
            );
            assert_eq!(
                round("1 red, 2 green, 6 blue"),
                Ok((
                    "",
                    Round {
                        red: 1,
                        green: 2,
                        blue: 6
                    }
                ))
            );
            assert_eq!(
                round("2 green"),
                Ok((
                    "",
                    Round {
                        red: 0,
                        green: 2,
                        blue: 0
                    }
                ))
            );
        }

        #[test]
        fn test_game() {
            assert_eq!(
                game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
                Ok((
                    "",
                    Game {
                        id: 1,
                        rounds: vec![
                            Round {
                                red: 4,
                                green: 0,
                                blue: 3
                            },
                            Round {
                                red: 1,
                                green: 2,
                                blue: 6
                            },
                            Round {
                                red: 0,
                                green: 2,
                                blue: 0
                            },
                        ]
                    }
                ))
            );
        }

        #[test]
        fn test_games() {
            assert_eq!(separated_list1(tag("\n"), game)("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), Ok(("", vec![
                Game { id: 1, rounds: vec![
                    Round { red: 4, green: 0, blue: 3 },
                    Round { red: 1, green: 2, blue: 6 },
                    Round { red: 0, green: 2, blue: 0 },
                ] },
                Game { id: 2, rounds: vec![
                    Round { red: 4, green: 0, blue: 3 },
                    Round { red: 1, green: 2, blue: 6 },
                    Round { red: 0, green: 2, blue: 0 },
                ] },
            ])));
        }
    }
}

// The power of a game is the product of the minumum number of cubes of each color
impl Game {
    fn power(&self) -> u32 {
        self.rounds
            .iter()
            .fold([0, 0, 0], |acc, round| {
                [
                    acc[0].max(round.red),
                    acc[1].max(round.green),
                    acc[2].max(round.blue),
                ]
            })
            .into_iter()
            .product()
    }
}

fn part1(filename: &Path) -> Result<String> {
    let input = read_to_string(filename)?;
    let (_, games) = parse::games(&input).unwrap();

    // Return sum of ID of game that contained no more than
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    Ok(games
        .into_iter()
        .filter(|game| {
            game.rounds
                .iter()
                .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
        })
        .map(|game| game.id)
        .sum::<u32>()
        .to_string())
}

fn part2(filename: &Path) -> Result<String> {
    let input = read_to_string(filename)?;
    let (_, games) = parse::games(&input).unwrap();

    // Calculate the sum of powers of the rounds
    Ok(games
        .into_iter()
        .map(|game| game.power())
        .sum::<u32>()
        .to_string())
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use crate::*;
    use aoc::aoc_test;

    #[test]
    fn test_power() {
        assert_eq!(
            Game {
                id: 1,
                rounds: vec![
                    Round {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    Round {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Round {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                ]
            }
            .power(),
            4 * 2 * 6
        );
        assert_eq!(
            Game {
                id: 2,
                rounds: vec![
                    Round {
                        red: 4,
                        green: 1,
                        blue: 3
                    },
                    Round {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Round {
                        red: 1,
                        green: 2,
                        blue: 1
                    },
                ]
            }
            .power(),
            4 * 6 * 2
        );
    }

    #[test]
    fn test1() {
        aoc_test("test/02", part1, "8");
        aoc_test("02", part1, "2061")
    }

    #[test]
    fn test2() {
        aoc_test("test/02", part2, "2286");
        aoc_test("02", part2, "72596")
    }
}
