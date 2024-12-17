use core::panic;

use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashSet;
use priority_queue::PriorityQueue;

use crate::{Direction, Grid, Point};

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub start: Point,
    pub end: Point,
    pub walls: Grid<bool>,
}

impl Puzzle {
    // Calculate the next 'interesting' node when going in a specific direction
    // An interesting node is the start, end, and any branch in the path
    // Returns the point and the cost to go there
    // Errors on dead ends
    #[allow(dead_code)]
    fn next_branch(&self, from: Point, facing: Direction) -> Result<(usize, Direction, Point), ()> {
        let mut point = from + facing;
        let mut facing = facing;
        let mut cost = 1;

        loop {
            // If we're ever on start or end, return the cost
            if point == self.start || point == self.end {
                return Ok((cost, facing, point));
            }

            // Figure out how many points lead from here (but not back)
            // This is used for branch and dead end detection
            let mut options = vec![];

            // Straight
            if self.walls.get(point + facing).is_some_and(|&b| !b) {
                options.push((point + facing, facing, 1));
            }

            // Left
            if self
                .walls
                .get(point + facing.rotate_left())
                .is_some_and(|&b| !b)
            {
                options.push((point + facing.rotate_left(), facing.rotate_left(), 1001));
            }

            // Right
            if self
                .walls
                .get(point + facing.rotate_right())
                .is_some_and(|&b| !b)
            {
                options.push((point + facing.rotate_right(), facing.rotate_right(), 1001));
            }

            // If we have no responses, that's a dead end, error
            if options.is_empty() {
                return Err(());
            }

            // If we have more than one response, the current point is a branch, return the cost
            if options.len() > 1 {
                return Ok((cost, facing, point));
            }

            // Otherwise, advance
            let (new_point, new_facing, new_cost) = options.pop().unwrap();
            point = new_point;
            facing = new_facing;
            cost += new_cost;
        }
    }
}

#[aoc_generator(day16)]
pub fn parse(input: &str) -> Puzzle {
    let walls = Grid::read(input, &|c| c == '#');

    let newline_width = walls.width + 1;

    let start = input.chars().position(&|c| c == 'S').unwrap();
    let start = (start % newline_width, start / newline_width).into();

    let end = input.chars().position(&|c| c == 'E').unwrap();
    let end = (end % newline_width, end / newline_width).into();

    Puzzle { start, end, walls }
}

#[aoc(day16, part1, pq)]
fn part1_pq(input: &Puzzle) -> usize {
    let mut pq = PriorityQueue::new();
    pq.push((input.start, Direction::Right), 0_isize);

    let mut checked = HashSet::new();

    while let Some(((point, direction), cost)) = pq.pop() {
        if point == input.end {
            return (-cost) as usize;
        }

        if !checked.insert((point, direction)) {
            continue;
        }

        // Walk straight
        let new_point = point + direction;
        if input.walls.get(new_point) != Some(&true) {
            pq.push((new_point, direction), cost - 1);
        }

        // Turn left or right
        // Optimize slightly by only queueing a turn if there's no wall
        // TODO: This might fail on the starting condition?

        let new_d = direction.rotate_left();
        if input.walls.get(point + new_d) != Some(&true) {
            pq.push((point, new_d), cost - 1000);
        }

        let new_d = direction.rotate_right();
        if input.walls.get(point + new_d) != Some(&true) {
            pq.push((point, new_d), cost - 1000);
        }
    }

    // If we've made it here, the maze is unsolvable
    // Or we wrote something wrong :smile:
    panic!("unsolvable maze");
}

#[aoc(day16, part1, astar)]
fn part1_astar(input: &Puzzle) -> i32 {
    match pathfinding::prelude::astar(
        &(input.start, Direction::Right),
        |(point, direction)| {
            let mut successors = vec![];

            // Walk straight
            let new_point = *point + *direction;
            if input.walls.get(new_point) != Some(&true) {
                successors.push(((new_point, *direction), 1));
            }

            // Turn left or right
            // Optimize slightly by only queueing a turn if there's no wall
            let new_direction = direction.rotate_left();
            if input.walls.get(*point + new_direction) != Some(&true) {
                successors.push(((*point, new_direction), 1000));
            }

            let new_direction = direction.rotate_right();
            if input.walls.get(*point + new_direction) != Some(&true) {
                successors.push(((*point, new_direction), 1000));
            }

            successors
        },
        |(point, _)| point.manhattan_distance(&input.end),
        |(point, _)| *point == input.end,
    ) {
        Some((_, cost)) => cost,
        _ => panic!("unsolvable maze"),
    }
}

#[aoc(day16, part2, astar)]
fn part2_astar(input: &Puzzle) -> usize {
    match pathfinding::prelude::astar_bag(
        &(input.start, Direction::Right),
        |(point, direction)| {
            let mut successors = vec![];

            // Walk straight
            let new_point = *point + *direction;
            if input.walls.get(new_point) != Some(&true) {
                successors.push(((new_point, *direction), 1));
            }

            // Turn left or right
            // Optimize slightly by only queueing a turn if there's no wall
            let new_direction = direction.rotate_left();
            if input.walls.get(*point + new_direction) != Some(&true) {
                successors.push(((*point, new_direction), 1000));
            }

            let new_direction = direction.rotate_right();
            if input.walls.get(*point + new_direction) != Some(&true) {
                successors.push(((*point, new_direction), 1000));
            }

            successors
        },
        |(point, _)| point.manhattan_distance(&input.end),
        |(point, _)| *point == input.end,
    ) {
        Some((solutions, _)) => {
            let mut all_best_points = HashSet::new();
            for solution in solutions {
                for (point, _) in solution {
                    all_best_points.insert(point);
                }
            }
            all_best_points.len()
        }
        _ => panic!("unsolvable maze"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    //     const EXAMPLE: &str = "\
    // #######
    // #S....#
    // #.#.#.#
    // #...#E#
    // #######";

    const EXAMPLE: &str = "\
    ###############
    #.......#....E#
    #.#.###.#.###.#
    #.....#.#...#.#
    #.###.#####.#.#
    #.#.#.......#.#
    #.#.#####.###.#
    #...........#.#
    ###.#.#####.#.#
    #...#.....#.#.#
    #.#.#.###.#.#.#
    #.....#...#.#.#
    #.###.#.#.#.#.#
    #S..#.....#...#
    ###############";

    make_test!([part1_pq, part1_astar] => "day16.txt", 7036, 65436);
    make_test!([/*part1_pq, */part2_astar] => "day16.txt", 45, 489);
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_astar(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_astar(&parse(input)).to_string()
}
