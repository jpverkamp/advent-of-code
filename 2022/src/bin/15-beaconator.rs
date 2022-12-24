use aoc::*;
use regex::Regex;
use std::path::Path;

#[derive(Copy, Clone, Debug)]
struct Range {
    min: isize,
    max: isize,
}

impl Range {
    fn new(min: isize, max: isize) -> Self {
        Range { min, max }
    }

    fn union(self, other: Range) -> Option<Range> {
        // One range completely includes the other
        if other.min >= self.min && other.max <= self.max {
            return Some(self);
        }
        if self.min >= other.min && self.max <= other.max {
            return Some(other);
        }

        // One range is partially inside the other
        if other.min >= self.min && other.max <= self.max {
            return Some(Range {
                min: self.min,
                max: other.max,
            });
        }
        if other.max >= self.min && other.max <= self.max {
            return Some(Range {
                min: other.min,
                max: self.max,
            });
        }

        // No overlap
        None
    }

    fn len(&self) -> usize {
        return 1 + (self.max - self.min) as usize;
    }
}

#[derive(Debug)]
struct Ranges {
    data: Vec<Range>,
}

impl Ranges {
    fn new() -> Self {
        Ranges { data: Vec::new() }
    }

    fn union(&mut self, r: Range) {
        self.data.push(r);
        self.collapse();
    }

    fn intersection(&mut self, r: Range) {
        self.data = self
            .data
            .iter()
            .filter_map(|c| {
                // One range completely includes the other
                if r.min >= c.min && r.max <= c.max {
                    return Some(r);
                }
                if c.min >= r.min && c.max <= r.max {
                    return Some(*c);
                }

                // One range is partially inside the other
                if r.min >= c.min && r.min <= c.max {
                    return Some(Range {
                        min: r.min,
                        max: c.max,
                    });
                }
                if r.max >= c.min && r.max <= c.max {
                    return Some(Range {
                        min: c.min,
                        max: r.max,
                    });
                }

                // No overlap
                None
            })
            .collect();
    }

    fn collapse(&mut self) {
        loop {
            let mut to_merge = None;

            'find_merge: for (i, a) in self.data.iter().enumerate() {
                for (j, b) in self.data.iter().enumerate() {
                    if i == j {
                        continue;
                    } else if let Some(c) = a.union(*b) {
                        to_merge = Some((i, j, c));
                        break 'find_merge;
                    }
                }
            }

            if let Some((i, j, c)) = to_merge {
                self.data.remove(i.max(j));
                self.data.remove(i.min(j));
                self.data.push(c);
            } else {
                break;
            }
        }
    }

    fn len(&self) -> usize {
        self.data.iter().map(|r| r.len()).sum::<usize>()
    }
}

#[derive(Debug)]
struct Map {
    sensors: Vec<(Point, Point)>,
}

impl<I> From<&mut I> for Map
where
    I: Iterator<Item = String>,
{
    fn from(iter: &mut I) -> Self {
        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .expect("regex creation failed");

        let mut sensors = Vec::new();

        for line in iter {
            let cap = re.captures(&line).expect("regex doesn't match line");

            sensors.push((
                Point {
                    x: cap[1].parse::<isize>().expect("sensor x must be number"),
                    y: cap[2].parse::<isize>().expect("sensor y must be number"),
                },
                Point {
                    x: cap[3].parse::<isize>().expect("beacon x must be number"),
                    y: cap[4].parse::<isize>().expect("beacon y must be number"),
                },
            ))
        }

        Map { sensors }
    }
}

impl Map {
    fn ranges_for(&self, target_row: isize) -> Ranges {
        let mut ranges = Ranges::new();

        for (sensor, beacon) in &self.sensors {
            // Distance = Distance to beacon
            // Offset = How much of that is included in offset distance to target
            // Remaining = How much is in the side to side range
            let distance = sensor.manhattan_distance(&beacon);
            let offset = (sensor.y - target_row).abs();
            let remaining = distance - offset;

            // If we don't have any side to side, the beacon is too far from the target row
            if remaining <= 0 {
                continue;
            }

            // Calculate the range of values in the target row a beacon could not be in
            let mut min_x = sensor.x - remaining;
            let mut max_x = sensor.x + remaining;

            // Special case if the beacon is in the target row
            if beacon.y == target_row && beacon.x == min_x {
                min_x += 1;
            }
            if beacon.y == target_row && beacon.x == max_x {
                max_x -= 1;
            }

            ranges.union(Range::new(min_x, max_x));
        }

        ranges
    }
}

fn part1(filename: &Path) -> String {
    let map = Map::from(&mut iter_lines(filename));

    let target_row = if filename
        .to_str()
        .unwrap()
        .contains("test")
    {
        10
    } else {
        2000000
    };

    map.ranges_for(target_row).len().to_string()
}

fn part2(filename: &Path) -> String {
    let map = Map::from(&mut iter_lines(filename));

    let bound = if filename
        .to_str()
        .unwrap()
        .contains("test")
    {
        20
    } else {
        4000000
    };

    let mut p = None;

    for y in 0..=bound {
        let mut ranges = map.ranges_for(y);
        ranges.intersection(Range::new(0, bound));

        // If we don't have a full range, we have a candidate
        // Candidates have exactly two Range, 0 to x-1 and x+1 to bound
        if ranges.data.len() > 1 {
            let x = ranges
                .data
                .into_iter()
                .find(|r| r.min > 0)
                .expect("must have non-zero x")
                .min
                - 1;

            // Check if the candidate is exactly equal to a beacon
            let new_p = Point { x, y };
            if map.sensors.iter().any(|(_, b)| new_p == *b) {
                continue;
            }

            // If not, we found the solution
            p = Some(new_p);
            break;
        }
    }

    match p {
        Some(Point { x, y }) => (x * 4000000 + y).to_string(),
        None => panic!("no answer found"),
    }
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
        aoc_test("15", part1, "4724228")
    }

    #[test]
    fn test2() {
        aoc_test("15", part2, "13622251246513")
    }
}
