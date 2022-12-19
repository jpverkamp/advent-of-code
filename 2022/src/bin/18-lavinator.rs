use aoc::*;
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    path::Path,
};

#[derive(Debug)]
struct Point3DCloud {
    points: HashSet<Point3D>,
}

impl Point3DCloud {
    fn contains(&self, p: Point3D) -> bool {
        self.points.contains(&p)
    }

    fn bounds(&self) -> (Point3D, Point3D) {
        let mut min_bound = Point3D::new(isize::MAX, isize::MAX, isize::MAX);
        let mut max_bound = Point3D::new(isize::MIN, isize::MIN, isize::MIN);

        self.points.iter().for_each(|p| {
            min_bound.x = min_bound.x.min(p.x);
            min_bound.y = min_bound.y.min(p.y);
            min_bound.z = min_bound.z.min(p.z);

            max_bound.x = max_bound.x.max(p.x);
            max_bound.y = max_bound.y.max(p.y);
            max_bound.z = max_bound.z.max(p.z);
        });

        (min_bound, max_bound)
    }

    #[allow(dead_code)]
    fn in_bounds(&self, p: Point3D) -> bool {
        let (min_bound, max_bound) = self.bounds();

        p.x >= min_bound.x
            && p.x <= max_bound.x
            && p.y >= min_bound.y
            && p.y <= max_bound.y
            && p.z >= min_bound.z
            && p.z <= max_bound.z
    }
}

impl<I> From<&mut I> for Point3DCloud
where
    I: Iterator<Item = String>,
{
    fn from(iter: &mut I) -> Self {
        Point3DCloud {
            points: iter
                .map(|line| {
                    let (x, y, z) = line
                        .split(',')
                        .map(|v| v.parse::<isize>().expect("must be numbers"))
                        .collect_tuple()
                        .expect("must have three elements");
                    Point3D::new(x, y, z)
                })
                .collect::<HashSet<_>>(),
        }
    }
}

fn part1(filename: &Path) -> String {
    let cloud = Point3DCloud::from(&mut iter_lines(filename));

    cloud
        .points
        .iter()
        .map(|p| {
            Point3D::UNITS
                .iter()
                .map(|s| !cloud.contains(*p + *s))
                .filter(|v| *v)
                .count()
        })
        .sum::<usize>()
        .to_string()
}

fn part2(filename: &Path) -> String {
    let cloud = Point3DCloud::from(&mut iter_lines(filename));
    let (mut min_bound, mut max_bound) = cloud.bounds();
    min_bound = min_bound - Point3D::new(1, 1, 1);
    max_bound = max_bound + Point3D::new(1, 1, 1);

    // Calculate all cubes within bounds (expand by one) that are 'external'
    // Do this by starting in one corner and flood filling from the bounds inwards
    let mut external = HashSet::new();

    let mut q = VecDeque::new();
    q.push_back(min_bound);

    while !q.is_empty() {
        let p = q.pop_front().unwrap();

        // Ignore points we've already explored
        if external.contains(&p) {
            continue;
        }

        // Points in the cloud are not external
        if cloud.contains(p) {
            continue;
        }

        // Points out of bounds are ignored
        if p.x < min_bound.x
            || p.y < min_bound.y
            || p.z < min_bound.z
            || p.x > max_bound.x
            || p.y > max_bound.y
            || p.z > max_bound.z
        {
            continue;
        }

        // Otherwise, mark as external
        external.insert(p);

        // Check all neighbors
        Point3D::UNITS.iter().for_each(|s| q.push_back(p + *s));
    }

    // Any side adjacent to an external cube is external
    cloud
        .points
        .iter()
        .map(|p| {
            Point3D::UNITS
                .iter()
                .map(|s| external.contains(&(*p + *s)))
                .filter(|v| *v)
                .count()
        })
        .sum::<usize>()
        .to_string()
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
        aoc_test("18", part1, "4548")
    }

    #[test]
    fn test2() {
        aoc_test("18", part2, "2588")
    }
}
