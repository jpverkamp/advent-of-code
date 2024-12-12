use aoc_runner_derive::{aoc, aoc_generator};

use crate::{Direction, Grid, Point};

#[aoc_generator(day12)]
fn parse(input: &str) -> Grid<char> {
    Grid::read(input, &|c| c)
}

fn get_regions<T>(input: &Grid<T>) -> Vec<(&T, Vec<Point>)>
where
    T: Clone + Default + PartialEq,
{
    let mut assigned_regions = Grid::new(input.width, input.height);
    let mut regions = vec![];

    // Calculate the points in each region
    for x in 0..(input.width) {
        for y in 0..(input.height) {
            let p: Point = (x, y).into();

            if assigned_regions.get(p).is_some_and(|v| *v) {
                continue;
            }

            let c = input.get(p).unwrap();

            let region = input.flood_fill(p);
            for p in region.iter() {
                assigned_regions.set(*p, true);
            }
            regions.push((c, region));
        }
    }

    regions
}

#[aoc(day12, part1, v1)]
fn part1_v1(input: &Grid<char>) -> usize {
    let regions = get_regions(input);

    // For each region, find the perimeter, area, and then the score
    regions
        .iter()
        .map(|(&c, region)| {
            // For each point, each neighbor which doesn't match is an edge
            // Score is area times this perimeter
            region.len()
                * region
                    .iter()
                    .map(|p| {
                        p.neighbors()
                            .iter()
                            .map(|n| {
                                if let Some(&v) = input.get(*n) {
                                    if v == c {
                                        0
                                    } else {
                                        1
                                    }
                                } else {
                                    1
                                }
                            })
                            .sum::<usize>()
                    })
                    .sum::<usize>()
        })
        .sum::<usize>()
}

#[aoc(day12, part2, v1)]
fn part2_v1(input: &Grid<char>) -> usize {
    let regions = get_regions(input);

    // For each region, find the number of edges, area, and then the score
    regions
        .iter()
        .map(|(&c, region)| {
            // Edges in this version run along the border of the region
            // Score is area times number of edges
            region.len()
                * Direction::all()
                    .iter()
                    .map(|&direction| {
                        // Run edge detection in each direction once per region
                        // This will create a new grid that is true for edges in that direction

                        // By doing up/down separately we avoid the case:
                        // ......
                        // ...XX.
                        // ...XX.
                        // .XX...
                        // .XX...
                        // ......
                        // The center is two edges, not one (one is an up, one is a down)
                        let mut edges = Grid::new(input.width, input.height);
                        region.iter().for_each(|p| {
                            if input.get(*p + direction).is_none_or(|&v| v != c) {
                                edges.set(*p, true);
                            }
                        });

                        // For edges in that direction, identify 'regions'
                        // Each of those is a single contiguous edge
                        get_regions(&edges).iter().filter(|(&c, _)| c).count()
                    })
                    .sum::<usize>()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    make_test!([part1_v1] => "day12.txt", 1930, 1450816);
    make_test!([part2_v1] => "day12.txt", 1206, 865662);

    const EXAMPLE_MINI: &str = "\
AAAA
BBCD
BBCC
EEEC";

    const EXAMPLE_XOXO: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE_FIGURE8: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn test_part1_v1_example_mini() {
        assert_eq!(part1_v1(&parse(EXAMPLE_MINI)), 140);
    }

    #[test]
    fn test_part1_v1_example_xoxo() {
        assert_eq!(part1_v1(&parse(EXAMPLE_XOXO)), 772);
    }

    #[test]
    fn test_part2_v1_example_mini() {
        assert_eq!(part2_v1(&parse(EXAMPLE_MINI)), 80);
    }

    #[test]
    fn test_part2_v1_example_xoxo() {
        assert_eq!(part2_v1(&parse(EXAMPLE_XOXO)), 436);
    }

    #[test]
    fn test_part2_v1_example_figure8() {
        assert_eq!(part2_v1(&parse(EXAMPLE_FIGURE8)), 368);
    }
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_v1(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_v1(&parse(input)).to_string()
}
