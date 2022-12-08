use aoc::*;
use std::path::Path;

#[derive(Debug)]
struct Forest {
    trees: Matrix<u8>,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Forest {
    pub fn from(lines: Vec<String>) -> Forest {
        let width = lines.get(0).expect("must have at least 1 line").len();
        let height = lines.len();

        let mut trees = Matrix::<u8>::new(width, height);

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                trees[[x, y]] = c.to_string().parse::<u8>().expect("chars must be digits");
            }
        }

        Forest { trees }
    }

    pub fn width(&self) -> usize {
        self.trees.width()
    }

    pub fn height(&self) -> usize {
        self.trees.height()
    }

    // Test if the given x/y tree is visible from the given direction
    pub fn visible_from(&self, x: usize, y: usize, d: Direction) -> bool {
        use Direction::*;

        let (xd, yd) = match d {
            North => (0, -1),
            South => (0, 1),
            West => (-1, 0),
            East => (1, 0),
        };

        let height = self.trees[[x, y]];
        let mut xi = x as isize;
        let mut yi = y as isize;

        // Special case north/west edge
        if xi + xd < 0 || yi + yd < 0 {
            return true;
        }

        // Move off the current tree
        xi += xd;
        yi += yd;

        while self.trees.in_bounds(xi as usize, yi as usize) {
            if self.trees[[xi as usize, yi as usize]] >= height {
                return false;
            }

            // Deal with negative indexes
            if x == 0 && xd == -1 || y == 0 && yd == -1 {
                break;
            };

            xi += xd;
            yi += yd;
        }

        true
    }

    // Essentially the opposite of the above
    // Move in direction d and count how many trees are visible
    pub fn visible_count(&self, x: usize, y: usize, d: Direction) -> usize {
        use Direction::*;

        let (xd, yd) = match d {
            North => (0, -1),
            South => (0, 1),
            West => (-1, 0),
            East => (1, 0),
        };

        let height = self.trees[[x, y]];
        let mut xi = x as isize;
        let mut yi = y as isize;
        let mut count = 0;

        // Special case north/west edge
        if xi + xd < 0 || yi + yd < 0 {
            return 0;
        }

        // Move off the current tree
        xi += xd;
        yi += yd;

        while self.trees.in_bounds(xi as usize, yi as usize) {
            count += 1;

            if self.trees[[xi as usize, yi as usize]] >= height {
                break;
            }

            // Deal with negative indexes
            if x == 0 && xd == -1 || y == 0 && yd == -1 {
                break;
            };

            xi += xd;
            yi += yd;
        }

        count
    }
}

fn part1(filename: &Path) -> String {
    let forest = Forest::from(read_lines(filename));

    let mut count = 0;

    if cfg!(debug_assertions) {
        for d in DIRECTIONS.into_iter() {
            println!("{:?}", d);
            for y in 0..forest.height() {
                for x in 0..forest.width() {
                    print!(
                        "{}",
                        if forest.visible_from(x, y, d) {
                            '\u{025FC}'
                        } else {
                            '\u{025FB}'
                        }
                    );
                }
                println!();
            }
            println!();
        }
        println!();
    }

    for x in 0..forest.width() {
        for y in 0..forest.height() {
            if DIRECTIONS.iter().any(|d| forest.visible_from(x, y, *d)) {
                count += 1;
            }
        }
    }

    count.to_string()
}

fn part2(filename: &Path) -> String {
    let forest = Forest::from(read_lines(filename));

    let mut best_scenic_score = 0;

    for y in 0..forest.height() {
        for x in 0..forest.width() {
            let scenic_score = DIRECTIONS
                .into_iter()
                .map(|d| forest.visible_count(x, y, d))
                .product::<usize>();
            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }

    if cfg!(debug_assertions) {
        let mut scores = Matrix::<usize>::new(forest.width(), forest.height());

        for y in 0..forest.height() {
            for x in 0..forest.width() {
                scores[[x, y]] = DIRECTIONS
                    .into_iter()
                    .map(|d| forest.visible_count(x, y, d))
                    .product::<usize>();
            }
        }

        let width = (best_scenic_score as f64).log10().ceil() as usize + 1;

        for y in 0..forest.height() {
            for x in 0..forest.width() {
                print!("{:width$}", scores[[x, y]], width = width);
            }
            println!();
        }
    }

    best_scenic_score.to_string()
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
        aoc_test("08", part1, "1814")
    }

    #[test]
    fn test2() {
        aoc_test("08", part2, "330786")
    }
}
