use crate::types::{Colors, Game, Round};
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
