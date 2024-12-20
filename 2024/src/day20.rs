use std::{cell::RefCell, rc::Rc};

use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashSet;

use crate::{Direction, Grid, Point};
use pathfinding::prelude::{astar, dijkstra_all};

#[derive(Debug, Clone)]
struct Puzzle {
    example: bool,
    walls: Grid<bool>,
    start: Point,
    end: Point,
}

#[aoc_generator(day20)]
fn parse(input: &str) -> Puzzle {
    // Override for cutoff, for the example we don't, but for *reasons*
    // > How many cheats would save you at least 100 picoseconds?
    let (example, input) = if input.contains("example") {
        let (_, input) = input.split_once('\n').unwrap();
        (true, input)
    } else {
        (false, input)
    };

    let walls = Grid::read(input, &|c| c == '#');

    let line_width = walls.width + 1;
    let start = input
        .find('S')
        .map(|i| (i % line_width, i / line_width).into())
        .unwrap();
    let end = input
        .find('E')
        .map(|i| (i % line_width, i / line_width).into())
        .unwrap();

    Puzzle {
        example,
        walls,
        start,
        end,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    PreSkip,
    Skip0,
    Skip1(Point),
    PostSkip(Point),
    NoSkip,
}

#[allow(dead_code)]
fn debug_print(input: &Puzzle, path: &[(Point, State)], last_state: State) {
    let mut to_print = Grid::new(input.walls.width, input.walls.height);
    for x in 0..input.walls.width {
        for y in 0..input.walls.height {
            to_print.set(
                (x, y),
                if *input.walls.get((x, y)).unwrap() {
                    '#'
                } else {
                    '.'
                },
            );
        }
    }

    to_print.set(input.start, 'S');
    to_print.set(input.end, 'E');
    for (point, _) in path {
        to_print.set(*point, 'o');
    }
    if let State::PostSkip(p) = last_state {
        to_print.set(p, '*');
    }

    println!("{}", to_print.to_string(&|c| c.to_string()));
}

// This works in theory but is *very very slow* on the main input
// #[aoc(day20, part1, v1)]
#[allow(dead_code)]
fn part1_v1(input: &Puzzle) -> usize {
    // Keep track of skips we've used
    // We cannot use the same one more than once
    let applied_skips = Rc::new(RefCell::new(HashSet::new()));

    // Shared successor function, combining the point we're at and the state we're in

    let successor = |(point, state): &(Point, State)| {
        let mut successors = Vec::new();

        // If we're pre-skip, we can always start the skip
        // Do not include the move as part of this
        if *state == State::PreSkip {
            successors.push(((*point, State::Skip0), 0));
        }

        // If we're out of skip points, transition to post skip without moving
        // This means there are *no more cases*
        // Also, we can't skip the exact same set of walls more than once
        if let State::Skip1(p1) = state {
            if !applied_skips.borrow().contains(p1) {
                successors.push(((*point, State::PostSkip(*p1)), 0));
            }
            return successors;
        }

        // Try to move in each direction
        // If we're in a skip state, we can ignore walls
        Direction::all()
            .iter()
            .map(|&dir| (*point + dir, *state))
            .for_each(|(new_point, state)| {
                // We can never walk off the edge of the map
                if input.walls.get(new_point).is_none() {
                    return;
                }

                match state {
                    // Pre-skip, post-skip, and no-skip states have to obey walls
                    State::PreSkip | State::PostSkip(_) | State::NoSkip => {
                        if input.walls.get(new_point) == Some(&false) {
                            successors.push(((new_point, state), 1));
                        }
                    }

                    // Skip means we can only walk on walls
                    State::Skip0 => {
                        successors.push(((new_point, State::Skip1(new_point)), 1));
                    }
                    // Skip 2 should not be evalulate here
                    State::Skip1(_) => unreachable!("Skip1 should not be evaluated here"),
                }
            });

        successors
    };

    // The initial time doesn't include skipping, so use the NoSkip state
    let initial_time = match astar(
        &(input.start, State::NoSkip),
        successor,
        |(point, _)| point.manhattan_distance(&input.end) as u32,
        |(point, _)| point == &input.end,
    ) {
        Some((_path, cost)) => cost,
        None => panic!("No initial path found"),
    };

    // Keep going so long as we find 'cheating' skips
    while let Some((path, cost)) = astar(
        &(input.start, State::PreSkip),
        successor,
        |(point, _)| point.manhattan_distance(&input.end) as u32,
        |(point, state)| point == &input.end && matches!(state, State::PostSkip(_)),
    ) {
        // The last point on the path stores the skip we used
        let last_state = path.last().unwrap().1;
        let skip_used = match last_state {
            State::PostSkip(p) => p,
            _ => {
                unreachable!("We should have ended in a post-skip state, was in {last_state:?}")
            }
        };

        // How much time did we save?
        let savings = initial_time - cost;

        // If we didn't save enough time (or any time!), we're done
        if savings < if input.example { 1 } else { 100 } {
            break;
        }

        // We have a new skip, so store that we used it
        applied_skips.borrow_mut().insert(skip_used);
    }

    let result = applied_skips.borrow().len();
    result
}

// This doesn't work because it doesn't account for the skip bound
// #[aoc(day20, part1, floodfill)]
#[allow(dead_code)]
fn part1_floodfill(input: &Puzzle) -> usize {
    let mut point = input.start;
    let mut visited = Grid::new(input.walls.width, input.walls.height);
    let mut skipped = HashSet::new();
    let mut shortcuts = 0;

    'next_point: loop {
        visited.set(point, true);

        // Are there any walls that we can skip that will lead us back on to the path
        // It has to be straight two steps, otherwise it will end up the same length
        // (We'd be cutting off a corner)
        for d in Direction::all() {
            if input.walls.get(point + d) == Some(&true)
                && input.walls.get(point + d + d) == Some(&false)
                && !visited.get(point + d + d).unwrap_or(&false)
                && !skipped.contains(&(point + d, point + d + d))
            {
                // Can only skip the same point once
                skipped.insert((point + d, point + d + d));

                shortcuts += 1;
            }
        }

        // If we're at the end, stop
        if point == input.end {
            break;
        }

        // Otherwise, find the one point that we've not already visited
        for d in Direction::all() {
            if input.walls.get(point + d) == Some(&false)
                && !visited.get(point + d).unwrap_or(&false)
            {
                point = point + d;
                continue 'next_point;
            }
        }

        // If we make it here, we failed to find the path
        unreachable!("No path found at {point:?}");
    }

    shortcuts
}

#[aoc(day20, part1, pathscan)]
fn part1_pathscan(input: &Puzzle) -> usize {
    // First, find the one true path
    let path = astar(
        &input.start,
        |point| {
            Direction::all()
                .iter()
                .map(|&dir| *point + dir)
                .filter(|&new_point| input.walls.get(new_point) == Some(&false))
                .map(|new_point| (new_point, 1))
                .collect::<Vec<_>>()
        },
        |point| point.manhattan_distance(&input.end),
        |point| *point == input.end,
    )
    .expect("No path found")
    .0;

    let cutoff = if input.example { 1 } else { 100 };

    // Now, for each point in that path, see if we can skip to a point further along the path
    let mut shortcut_count = 0;
    for (i, p) in path.iter().enumerate() {
        // Are there any walls that we can skip that will lead us back on to the path
        // It has to be straight two steps, otherwise it will end up the same length
        // (We'd be cutting off a corner)
        for d in Direction::all() {
            if input.walls.get(*p + d) == Some(&true)
                && input.walls.get(*p + d + d) == Some(&false)
                && path
                    .iter()
                    .position(|&p2| p2 == *p + d + d)
                    .is_some_and(|i2| i2 > i && i2 - i > cutoff)
            {
                shortcut_count += 1;
            }
        }
    }

    shortcut_count
}

#[aoc(day20, part1, grid)]
fn part1_grid(input: &Puzzle) -> usize {
    // First, find the one true path
    let path = astar(
        &input.start,
        |point| {
            Direction::all()
                .iter()
                .map(|&dir| *point + dir)
                .filter(|&new_point| input.walls.get(new_point) == Some(&false))
                .map(|new_point| (new_point, 1))
                .collect::<Vec<_>>()
        },
        |point| point.manhattan_distance(&input.end),
        |point| *point == input.end,
    )
    .expect("No path found")
    .0;

    // Store distances as a grid
    let mut distances = Grid::new(input.walls.width, input.walls.height);
    for (i, p) in path.iter().enumerate() {
        distances.set(*p, i);
    }

    let cutoff = if input.example { 1 } else { 100 };

    // Now, for each point in that path, see if we can skip to a point further along the path
    let mut shortcut_count = 0;
    for (i, p) in path.iter().enumerate() {
        // Are there any walls that we can skip that will lead us back on to the path
        // It has to be straight two steps, otherwise it will end up the same length
        // (We'd be cutting off a corner)
        for d in Direction::all() {
            if input.walls.get(*p + d) == Some(&true)
                && input.walls.get(*p + d + d) == Some(&false)
                && distances
                    .get(*p + d + d)
                    .map_or(false, |i2| *i2 > i + cutoff)
            {
                shortcut_count += 1;
            }
        }
    }

    shortcut_count
}

#[aoc(day20, part1, dijkstra)]
fn part1_dijkstra(input: &Puzzle) -> usize {
    // Find every point's distance to the end using dijkstra's algorithm
    let distances = dijkstra_all(&input.end, |point| {
        Direction::all()
            .iter()
            .map(|&dir| *point + dir)
            .filter(|&new_point| input.walls.get(new_point) == Some(&false))
            .map(|new_point| (new_point, 1))
            .collect::<Vec<_>>()
    });

    let cutoff = if input.example { 1 } else { 100 };
    let mut p = input.start;
    let mut shortcut_count = 0;

    // Follow the shortest path via dijkstra's algorithm
    while let Some((next_point, current_distance)) = distances.get(&p) {
        // Are there any walls that we can skip that will lead us back on to the path
        // It has to be straight two steps, otherwise it will end up the same length
        // (We'd be cutting off a corner)
        for d in Direction::all() {
            if input.walls.get(p + d) == Some(&true) && input.walls.get(p + d + d) == Some(&false) {
                // Special case the exit (it's not in the distances map)
                if p + d + d == input.end {
                    shortcut_count += 1;
                    continue;
                }

                // For all other cases, calculate up how much we're saving
                match distances.get(&(p + d + d)) {
                    Some((_, new_distance)) if *new_distance > current_distance + cutoff + 2 => {
                        shortcut_count += 1;
                    }
                    _ => {}
                }
            }
        }

        // Advance along dijkstra's path
        p = *next_point;
    }

    shortcut_count
}

#[aoc(day20, part2, pathscan)]
fn part2_pathscan(input: &Puzzle) -> usize {
    let (cutoff, skiplength) = if input.example {
        (50, 20_i32)
    } else {
        (100, 20_i32)
    };

    // First, find the one true path
    let path = astar(
        &input.start,
        |point| {
            Direction::all()
                .iter()
                .map(|&dir| *point + dir)
                .filter(|&new_point| input.walls.get(new_point) == Some(&false))
                .map(|new_point| (new_point, 1))
                .collect::<Vec<_>>()
        },
        |point| point.manhattan_distance(&input.end),
        |point| *point == input.end,
    )
    .expect("No path found")
    .0;

    // Find the distance from the exit to every point
    // This will be used to verify 'better' paths
    // We need this because it's possible to take a shortcut to a previous dead end
    let mut distances = dijkstra_all(&input.end, |point| {
        Direction::all()
            .iter()
            .map(|&dir| *point + dir)
            .filter(|&new_point| input.walls.get(new_point) == Some(&false))
            .map(|new_point| (new_point, 1))
            .collect::<Vec<_>>()
    });

    // Add the exit :)
    distances.insert(input.end, (input.end, 0));

    // Now, for each point in that path, see if we can skip to a point further along the path
    // We can skip up to 20, so any point that within manhattan distance 20 is valid
    let mut shortcut_count = 0;
    let mut used_shortcuts = HashSet::new();
    for (i, p) in path.iter().enumerate() {
        // Are there any walls that we can skip that will lead us back on to the path
        // It has to be straight two steps, otherwise it will end up the same length
        // (We'd be cutting off a corner)
        for xd in -skiplength..=skiplength {
            for yd in -skiplength..=skiplength {
                // Ignore skipping to yourself or skipping too far
                if xd == 0 && yd == 0 || xd.abs() + yd.abs() > skiplength {
                    continue;
                }

                let d: Point = (xd, yd).into();
                let p2: Point = *p + d;

                // Cannot end on a wall
                // TODO: This is covered by the distanced map
                if input.walls.get(p2) != Some(&false) {
                    continue;
                }

                // Cannot get from the target to the end
                if !distances.contains_key(&p2) {
                    println!("Cannot get from {p2:?} to the exit");
                    continue;
                }

                // Cannot already have been used
                if used_shortcuts.contains(&(p, p2)) {
                    continue;
                }
                used_shortcuts.insert((p, p2));

                // The distance using the shortcut
                let new_distance = i // To start
                    + d.manhattan_distance(&Point::ZERO) as usize // Shortcut
                    + distances.get(&p2).unwrap().1 as usize // To end
                    + 1;

                // Doesn't cut off enough
                if new_distance > path.len() - cutoff {
                    continue;
                }

                // If we've made it this far, we can shortcut!
                shortcut_count += 1;
                used_shortcuts.insert((p, p2));
            }
        }
    }

    shortcut_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
example
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    make_test!([part1_pathscan, part1_grid, part1_dijkstra] => "day20.txt", 44, 1399);
    make_test!([part2_pathscan] => "day20.txt", 285, 994807);
}
