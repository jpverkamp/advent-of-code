use anyhow::Result;
use aoc::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::map,
    multi::separated_list1,
    sequence::{self, preceded},
    IResult,
};
use std::{fs::read_to_string, path::Path};

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

fn color(s: &str) -> IResult<&str, Color> {
    alt((
        map(tag("red"), |_| Color::Red),
        map(tag("green"), |_| Color::Green),
        map(tag("blue"), |_| Color::Blue),
    ))(s)
}

// 3 blue, 4 red
#[derive(Debug, PartialEq)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
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
            Color::Red => red = count,
            Color::Green => green = count,
            Color::Blue => blue = count,
        }
    }

    Ok((s, Round { red, green, blue }))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn game(s: &str) -> IResult<&str, Game> {
    let (s, _) = tag("Game ")(s)?;
    let (s, id) = complete::u32(s)?;
    let (s, _) = tag(": ")(s)?;

    let (s, rounds) = separated_list1(tag("; "), round)(s)?;

    Ok((s, Game { id, rounds }))
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
    let (_, games) = separated_list1(tag("\n"), game)(&input).unwrap();

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
    let (_, games) = separated_list1(tag("\n"), game)(&input).unwrap();

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
    fn test_color() {
        assert_eq!(color("red"), Ok(("", Color::Red)));
        assert_eq!(color("green"), Ok(("", Color::Green)));
        assert_eq!(color("blue"), Ok(("", Color::Blue)));
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
