use anyhow::Result;
use aoc::*;
use std::path::Path;

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    #[allow(dead_code)]
    fn record_breakers_bf(&self) -> u64 {
        (0..=self.time)
            .filter(|x| x * (self.time - x) > self.record)
            .count() as u64
    }

    fn record_breakers(&self) -> u64 {
        // Race is D units long
        // Each option is hold the button for x seconds, maximum of T
        // Distance traveled is x for T-x seconds
        // We need to travel at least D
        // x(T-x) > D
        // xT - x^2 > D
        // x^2 - xT + D < 0
        // x in (T +/- sqrt(T^2 - 4D)) / 2

        let t = self.time as f64;
        let d = self.record as f64;

        let x1 = (t - (t * t - 4.0 * d).sqrt()) / 2.0;
        let x2 = (t + (t * t - 4.0 * d).sqrt()) / 2.0;

        let lo = x1.min(x2).ceil() as u64;
        let hi = x1.max(x2).floor() as u64;

        // If lo is an integer, we don't want it (< vs <=)
        // But it's a float, so check by epsilon difference
        // This isn't perfect, but it works
        let diff = ((lo as f64) - x1.min(x2)).abs();

        if diff < 1e-6 {
            hi - lo - 1
        } else {
            hi - lo + 1
        }
    }
}

mod parse {
    use crate::*;
    use nom::{
        bytes::complete::tag,
        character::complete,
        character::complete::{newline, space1},
        multi::separated_list1,
        sequence::{delimited, preceded, tuple},
        *,
    };

    pub(crate) fn races(s: &str) -> IResult<&str, Vec<Race>> {
        let (s, times) = delimited(
            tuple((tag("Time:"), space1)),
            separated_list1(space1, complete::u64),
            newline,
        )(s)?;
        let (s, records) = preceded(
            tuple((tag("Distance:"), space1)),
            separated_list1(space1, complete::u64),
        )(s)?;

        Ok((
            s,
            times
                .into_iter()
                .zip(records.into_iter())
                .map(|(time, record)| Race { time, record })
                .collect::<Vec<_>>(),
        ))
    }
}

fn part1(filename: &Path) -> Result<String> {
    let input = std::fs::read_to_string(filename)?;
    let (s, races) = parse::races(&input).unwrap();
    assert_eq!(s.trim(), "");

    Ok(races
        .iter()
        .map(|r| r.record_breakers())
        .product::<u64>()
        .to_string())
}

fn part2(filename: &Path) -> Result<String> {
    let input = std::fs::read_to_string(filename)?;
    let (s, races) = parse::races(&input).unwrap();
    assert_eq!(s.trim(), "");

    let race = Race {
        time: races
            .iter()
            .map(|r| r.time.to_string())
            .collect::<String>()
            .parse::<u64>()?,
        record: races
            .iter()
            .map(|r| r.record.to_string())
            .collect::<String>()
            .parse::<u64>()?,
    };

    Ok(race.record_breakers().to_string())
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
        aoc_test("test/06", part1, "288");
        aoc_test("06", part1, "741000");
    }

    #[test]
    fn test2() {
        aoc_test("test/06", part2, "71503");
        aoc_test("06", part2, "38220708");
    }
}
