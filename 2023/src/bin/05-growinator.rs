use anyhow::Result;
use aoc::*;
use std::{ops::RangeInclusive, path::Path};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

// A range mapping; defines a range from src..=(src+len) to dst..=(dst+len)
#[derive(Debug)]
pub struct RangeMap {
    src: u64,
    dst: u64,
    len: u64,
}

impl RangeMap {
    // If x is in the source range, map to the destination
    pub fn apply(&self, x: u64) -> Option<u64> {
        if x < self.src || x >= self.src + self.len {
            None
        } else {
            Some(self.dst + x - self.src)
        }
    }

    // Apply over an input range
    // Returns three optional ranges:
    // 1. The portion of the original range below self's range
    // 2. The portion of the original range overlapping self's range mapped to destination
    // 3. The portion of the original range above self's range
    #[allow(clippy::type_complexity)]
    pub fn apply_range(
        &self,
        input: RangeInclusive<u64>,
    ) -> (
        Option<RangeInclusive<u64>>,
        Option<RangeInclusive<u64>>,
        Option<RangeInclusive<u64>>,
    ) {
        let (input_start, input_end) = input.clone().into_inner();
        let src_end = self.src + self.len - 1;

        let below = if input_start < self.src {
            Some(input_start..=self.src.saturating_sub(1).min(input_end))
        } else {
            None
        };

        let overlap = if input_end >= self.src && input_start <= src_end {
            let overlap_start = input_start.max(self.src);
            let overlap_end = input_end.min(src_end);
            Some((self.dst + overlap_start - self.src)..=(self.dst + overlap_end - self.src))
        } else {
            None
        };

        let above = if input_end > src_end {
            Some(src_end.saturating_add(1).max(input_start)..=input_end)
        } else {
            None
        };

        (below, overlap, above)
    }
}

#[derive(Debug)]
pub struct CategoryMap {
    src_cat: Category,
    dst_cat: Category,
    range_maps: Vec<RangeMap>,
}

impl CategoryMap {
    pub fn apply(&self, x: u64) -> u64 {
        self.range_maps
            .iter()
            .find_map(|range_map| range_map.apply(x))
            .unwrap_or(x)
    }

    pub fn apply_range(&self, input: RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
        let mut ranges = vec![input.clone()];
        let mut result = vec![];

        for range_map in self.range_maps.iter() {
            let mut unchanged = vec![];

            // Mapped ranges are ready to return
            // Anything else passes to the next range map
            for range in ranges.iter() {
                let (below, overlap, above) = range_map.apply_range(range.clone());
                if let Some(below) = below {
                    unchanged.push(below);
                }
                if let Some(overlap) = overlap {
                    result.push(overlap);
                }
                if let Some(above) = above {
                    unchanged.push(above);
                }
            }

            ranges.clear();
            ranges.append(&mut unchanged);
        }

        // Any unchanged ranges after all maps are returned
        result.append(&mut ranges);

        result
    }
}

#[derive(Debug)]
pub struct Simulation {
    seeds: Vec<u64>,
    range_maps: Vec<CategoryMap>,
}

mod parse {
    use crate::{
        Category::{self, *},
        *,
    };
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

    pub fn simulation(s: &str) -> IResult<&str, super::Simulation> {
        let (s, seeds) = delimited(
            preceded(tag("seeds:"), space1),
            separated_list1(space1, complete::u64),
            many1(newline),
        )(s)?;

        let (s, range_maps) = separated_list1(many1(newline), category_map)(s)?;
        let (s, _) = many0(newline)(s)?;

        Ok((s, super::Simulation { seeds, range_maps }))
    }
}

fn part1(filename: &Path) -> Result<String> {
    let input = std::fs::read_to_string(filename)?;
    let (s, simulation) = parse::simulation(&input).unwrap();
    assert_eq!(s, "");

    let (cat, values) = simulation.range_maps.iter().fold(
        (Category::Seed, simulation.seeds),
        |(cat, values), range_map| {
            assert_eq!(cat, range_map.src_cat);
            (
                range_map.dst_cat,
                values.iter().map(|x| range_map.apply(*x)).collect(),
            )
        },
    );
    assert_eq!(cat, Category::Location);
    Ok(values.iter().min().unwrap().to_string())
}

#[allow(dead_code)]
fn part2_brute(filename: &Path) -> Result<String> {
    let input = std::fs::read_to_string(filename)?;
    let (s, mut simulation) = parse::simulation(&input).unwrap();
    assert_eq!(s, "");

    // Replace seeds with ranges
    simulation.seeds = simulation
        .seeds
        .chunks(2)
        .flat_map(|lo_hi| (lo_hi[0]..=(lo_hi[0] + lo_hi[1])).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (cat, values) = simulation.range_maps.iter().fold(
        (Category::Seed, simulation.seeds),
        |(cat, values), range_map| {
            assert_eq!(cat, range_map.src_cat);
            (
                range_map.dst_cat,
                values.iter().map(|x| range_map.apply(*x)).collect(),
            )
        },
    );
    assert_eq!(cat, Category::Location);
    Ok(values.iter().min().unwrap().to_string())
}

fn part2(filename: &Path) -> Result<String> {
    let input = std::fs::read_to_string(filename)?;
    let (s, simulation) = parse::simulation(&input).unwrap();
    assert_eq!(s, "");

    // Replace seeds with ranges
    let ranges = simulation
        .seeds
        .chunks(2)
        .map(|lo_hi| lo_hi[0]..=(lo_hi[0] + lo_hi[1]))
        .collect::<Vec<_>>();

    let (cat, values) =
        simulation
            .range_maps
            .iter()
            .fold((Category::Seed, ranges), |(cat, values), range_map| {
                assert_eq!(cat, range_map.src_cat);
                (
                    range_map.dst_cat,
                    values
                        .iter()
                        .flat_map(|r| range_map.apply_range(r.clone()))
                        .collect(),
                )
            });
    assert_eq!(cat, Category::Location);
    Ok(values
        .iter()
        .map(|r| r.clone().min().unwrap())
        .min()
        .unwrap()
        .to_string())
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc::aoc_test;

    #[test]
    fn test_range_map_apply() {
        let range_map = super::RangeMap {
            src: 5,
            dst: 10,
            len: 10,
        };
        assert_eq!(range_map.apply(4), None);
        assert_eq!(range_map.apply(5), Some(10));
        assert_eq!(range_map.apply(6), Some(11));
        assert_eq!(range_map.apply(14), Some(19));
        assert_eq!(range_map.apply(15), None);
    }

    #[test]
    fn test_category_map_apply() {
        let category_map = super::CategoryMap {
            src_cat: super::Category::Seed,
            dst_cat: super::Category::Soil,
            range_maps: vec![
                super::RangeMap {
                    src: 50,
                    dst: 98,
                    len: 2,
                },
                super::RangeMap {
                    src: 52,
                    dst: 50,
                    len: 48,
                },
            ],
        };
        assert_eq!(category_map.apply(79), 77);
    }

    #[test]
    fn test_range_map_range_apply() {
        let range_map = super::RangeMap {
            src: 5,
            dst: 10,
            len: 10,
        };
        // all low
        assert_eq!(range_map.apply_range(1..=4), (Some(1..=4), None, None));
        // low and mid
        assert_eq!(
            range_map.apply_range(1..=6),
            (Some(1..=4), Some(10..=11), None)
        );
        // all mid
        assert_eq!(range_map.apply_range(6..=14), (None, Some(11..=19), None));
        // mid and high
        assert_eq!(
            range_map.apply_range(6..=16),
            (None, Some(11..=19), Some(15..=16))
        );
        // all three
        assert_eq!(
            range_map.apply_range(1..=15),
            (Some(1..=4), Some(10..=19), Some(15..=15))
        );
    }

    #[test]
    fn test1() {
        aoc_test("test/05", part1, "35");
        aoc_test("05", part1, "825516882");
    }

    #[test]
    fn test2() {
        aoc_test("test/05", part2, "46");
        aoc_test("05", part2, "136096660");
    }
}
