use aoc_runner_derive::{aoc, aoc_generator};

use crate::Grid;

#[aoc_generator(day4)]
fn parse(input: &str) -> Grid<char> {
    fn id(c: char) -> char {
        c
    }

    Grid::read(input, &id)
}

#[aoc(day4, part1, inner_looping)]
fn part1_original(grid: &Grid<char>) -> i32 {
    let mut count = 0;

    // For each starting point
    for x in 0..grid.width {
        for y in 0..grid.height {
            // Ignore any that don't start with X
            if grid.get((x, y)) != Some(&'X') {
                continue;
            }

            // For each direction
            for dx in -1..=1 {
                'one_direction: for dy in -1..=1 {
                    // But have to be moving
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    // Iterate up to the remaining 3 characters in that direction
                    let mut xi = x as isize;
                    let mut yi = y as isize;

                    for target in ['M', 'A', 'S'].iter() {
                        xi += dx;
                        yi += dy;

                        if let Some(c) = grid.get((xi, yi)) {
                            if c != target {
                                continue 'one_direction;
                            }
                        } else {
                            continue 'one_direction;
                        }
                    }

                    count += 1;
                }
            }
        }
    }

    count
}

#[aoc(day4, part1, inline)]
fn part1_inline(grid: &Grid<char>) -> i32 {
    let mut count = 0;

    // For each starting point
    for x in 0..grid.width {
        for y in 0..grid.height {
            // Ignore any that don't start with X
            if grid.get((x, y)) != Some(&'X') {
                continue;
            }

            // Local (shadowing) signed copies
            let x = x as isize;
            let y = y as isize;

            // For each direction
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    if grid.get((x + dx, y + dy)) == Some(&'M')
                        && grid.get((x + 2 * dx, y + 2 * dy)) == Some(&'A')
                        && grid.get((x + 3 * dx, y + 3 * dy)) == Some(&'S')
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

#[aoc(day4, part2)]
fn part2_inline(grid: &Grid<char>) -> i32 {
    let mut count = 0;

    // Each center point of the X
    for x in 1..(grid.width - 1) {
        for y in 1..(grid.height - 1) {
            // All grids have an A in the center
            if grid.get((x, y)) != Some(&'A') {
                continue;
            }

            // Local (shadowing) signed copies
            let x = x as isize;
            let y = y as isize;

            // Each direction
            // This could be an || but I think this is easier to read :shrug:
            #[allow(clippy::if_same_then_else)]
            for delta in [-1, 1] {
                // Check the 4 corners horizontally
                if grid.get((x + delta, y + 1)) == Some(&'M')
                    && grid.get((x + delta, y - 1)) == Some(&'M')
                    && grid.get((x - delta, y + 1)) == Some(&'S')
                    && grid.get((x - delta, y - 1)) == Some(&'S')
                {
                    count += 1;
                }
                // And vertically
                else if grid.get((x + 1, y + delta)) == Some(&'M')
                    && grid.get((x - 1, y + delta)) == Some(&'M')
                    && grid.get((x + 1, y - delta)) == Some(&'S')
                    && grid.get((x - 1, y - delta)) == Some(&'S')
                {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    make_test!([part1_original, part1_inline] => "day4.txt", 18, 2406);
    make_test!([part2_inline] => "day4.txt", 9, 1807);
}
