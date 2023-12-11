use std::{
    collections::{BTreeMap, HashSet},
    fmt::{Display, Formatter},
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    pub x: i128,
    pub y: i128,
}

impl Point {
    pub fn manhattan_distance(&self, other: &Point) -> i128 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
pub struct Galaxy {
    pub stars: Vec<Point>,
}

impl Galaxy {
    pub fn expand(&mut self, age: i128) {
        let get_err = |f: fn(&Point) -> i128| {
            self.stars
                .iter()
                .map(f)
                .collect::<HashSet<_>>()
                .iter()
                .sorted()
                .fold((None, 0, BTreeMap::new()), |(last, err, mut errs), &v| {
                    let err = match last {
                        Some(last) => err + (age - 1).max(1) * (v - last - 1),
                        None => 0,
                    };
                    errs.insert(v, err);
                    (Some(v), err, errs)
                })
                .2
        };

        let x_err = get_err(|p| p.x);
        let y_err = get_err(|p| p.y);

        self.stars = self
            .stars
            .iter()
            .map(|p| Point {
                x: p.x + x_err[&p.x],
                y: p.y + y_err[&p.y],
            })
            .collect();
    }
}

impl From<String> for Galaxy {
    fn from(s: String) -> Self {
        let stars = s
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(j, c)| {
                        if c == '#' {
                            Some(Point {
                                x: j as i128,
                                y: i as i128,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Galaxy { stars }
    }
}

impl Display for Galaxy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x) = self
            .stars
            .iter()
            .map(|p| p.x)
            .minmax()
            .into_option()
            .unwrap();
        let (min_y, max_y) = self
            .stars
            .iter()
            .map(|p| p.y)
            .minmax()
            .into_option()
            .unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.stars.contains(&Point { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
