use std::collections::{HashMap, HashSet};

use aoc2025::grid::Grid;

aoc::main!(day7);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Start,
    Split,
    Empty,
}

#[aoc::register]
fn part1(input: &str) -> impl Into<String> {
    let splitter_grid = Grid::read(input, |c| match c {
        'S' => Tile::Start,
        '^' => Tile::Split,
        '.' => Tile::Empty,
        _ => unreachable!("Unknown character {c:?}"),
    });

    let splitter_x = splitter_grid
        .iter()
        .find_map(|(x, _, t)| if t == Tile::Start { Some(x) } else { None })
        .unwrap();

    let mut lasers = vec![false; splitter_grid.width() as usize];
    lasers[splitter_x as usize] = true;

    let mut buffer = vec![false; splitter_grid.width() as usize];
    let mut split_count = 0;

    for y in 1..splitter_grid.height() {
        buffer.fill(false);

        for x in 0..splitter_grid.width() {
            let idx = x as usize;
            if !lasers[idx] {
                continue;
            }

            match splitter_grid.get(x, y).unwrap() {
                Tile::Start | Tile::Empty => {
                    buffer[idx] = lasers[idx];
                }
                Tile::Split => {
                    split_count += 1;

                    if x > 0 {
                        buffer[idx - 1] = true;
                    }
                    if x < splitter_grid.width() - 1 {
                        buffer[idx + 1] = true;
                    }
                }
            }
        }

        std::mem::swap(&mut lasers, &mut buffer);
    }

    split_count.to_string()
}

#[aoc::register_render(scale = 2)]
fn part1_vid(input: &str) {
    let splitter_grid = Grid::read(input, |c| match c {
        'S' => Tile::Start,
        '^' => Tile::Split,
        '.' => Tile::Empty,
        _ => unreachable!("Unknown character {c:?}"),
    });

    let splitter_x = splitter_grid
        .iter()
        .find_map(|(x, _, t)| if t == Tile::Start { Some(x) } else { None })
        .unwrap();

    let mut lasers = vec![false; splitter_grid.width() as usize];
    lasers[splitter_x as usize] = true;

    let mut buffer = vec![false; splitter_grid.width() as usize];

    let mut laser_points = HashSet::new();

    for y in 1..splitter_grid.height() {
        buffer.fill(false);

        for x in 0..splitter_grid.width() {
            let idx = x as usize;
            if !lasers[idx] {
                continue;
            }

            match splitter_grid.get(x, y).unwrap() {
                Tile::Start | Tile::Empty => {
                    buffer[idx] = lasers[idx];
                    laser_points.insert((x, y));
                }
                Tile::Split => {
                    if x > 0 {
                        buffer[idx - 1] = true;
                        laser_points.insert((x - 1, y));
                    }
                    if x < splitter_grid.width() - 1 {
                        buffer[idx + 1] = true;
                        laser_points.insert((x + 1, y));
                    }
                }
            }
        }

        aoc::render_frame!(
            splitter_grid.width(),
            splitter_grid.height(),
            |x, y| {
                if splitter_grid.get(x, y) == Some(Tile::Start) {
                    (255, 0, 0)
                } else if splitter_grid.get(x, y) == Some(Tile::Split) {
                    (0, 0, 255)
                } else if laser_points.contains(&(x, y)) {
                    (255, 255, 0)
                } else {
                    (0, 0, 0)
                }
            }
        );

        std::mem::swap(&mut lasers, &mut buffer);
    }
}

#[aoc::register]
fn part2(input: &str) -> impl Into<String> {
    let splitter_grid = Grid::read(input, |c| match c {
        'S' => Tile::Start,
        '^' => Tile::Split,
        '.' => Tile::Empty,
        _ => unreachable!("Unknown character {c:?}"),
    });

    let splitter_x = splitter_grid
        .iter()
        .find_map(|(x, _, t)| if t == Tile::Start { Some(x) } else { None })
        .unwrap();

    // This time, keep a count of how many ways a laser could get to this position
    let mut lasers = vec![0_usize; splitter_grid.width() as usize];
    lasers[splitter_x as usize] = 1;

    let mut buffer = vec![0_usize; splitter_grid.width() as usize];

    for y in 1..splitter_grid.height() {
        buffer.fill(0_usize);

        for x in 0..splitter_grid.width() {
            let idx = x as usize;
            if lasers[idx] == 0 {
                continue;
            }

            match splitter_grid.get(x, y).unwrap() {
                // Lasers shining down, just directly add to ways to get here
                Tile::Start | Tile::Empty => {
                    buffer[idx] += lasers[idx];
                }
                // But splitters add in both directions
                Tile::Split => {
                    if x > 0 {
                        buffer[idx - 1] += lasers[idx];
                    }
                    if x < splitter_grid.width() - 1 {
                        buffer[idx + 1] += lasers[idx];
                    }
                }
            }
        }

        std::mem::swap(&mut lasers, &mut buffer);
    }

    lasers.iter().sum::<usize>().to_string()
}

#[aoc::register_render(scale = 2)]
fn part2_vid(input: &str) {
    let splitter_grid = Grid::read(input, |c| match c {
        'S' => Tile::Start,
        '^' => Tile::Split,
        '.' => Tile::Empty,
        _ => unreachable!("Unknown character {c:?}"),
    });

    let splitter_x = splitter_grid
        .iter()
        .find_map(|(x, _, t)| if t == Tile::Start { Some(x) } else { None })
        .unwrap();

    // This time, keep a count of how many ways a laser could get to this position
    let mut lasers = vec![0_usize; splitter_grid.width() as usize];
    lasers[splitter_x as usize] = 1;

    let mut buffer = vec![0_usize; splitter_grid.width() as usize];
    let mut laser_counts = HashMap::new();

    for y in 1..splitter_grid.height() {
        buffer.fill(0_usize);

        for x in 0..splitter_grid.width() {
            let idx = x as usize;
            if lasers[idx] == 0 {
                continue;
            }

            match splitter_grid.get(x, y).unwrap() {
                // Lasers shining down, just directly add to ways to get here
                Tile::Start | Tile::Empty => {
                    buffer[idx] += lasers[idx];
                    *laser_counts.entry((x, y)).or_insert(0) += lasers[idx];
                }
                // But splitters add in both directions
                Tile::Split => {
                    if x > 0 {
                        buffer[idx - 1] += lasers[idx];
                        *laser_counts.entry((x - 1, y)).or_insert(0) += lasers[idx];
                    }
                    if x < splitter_grid.width() - 1 {
                        buffer[idx + 1] += lasers[idx];
                        *laser_counts.entry((x + 1, y)).or_insert(0) += lasers[idx];
                    }
                }
            }
        }

        aoc::render_frame!(
            splitter_grid.width(),
            splitter_grid.height(),
            |x, y| {
                if splitter_grid.get(x, y) == Some(Tile::Start) {
                    (255, 0, 0)
                } else if splitter_grid.get(x, y) == Some(Tile::Split) {
                    (0, 0, 255)
                } else if let Some(count) = laser_counts.get(&(x, y)) {
                    let max_value = 6878160441036_usize; // From part 2 answer
                    let intensity =
                        ((*count as f64).ln() / (max_value as f64).ln() * 255.0).min(255.0) as u8;
                    (intensity, intensity, 0)
                } else {
                    (0, 0, 0)
                }
            }
        );

        std::mem::swap(&mut lasers, &mut buffer);
    }

    // What's the max value?
    print!("Max laser count: ");
    let max_count = lasers.iter().max().unwrap();
    println!("{max_count}");
}

aoc::test!(
    text = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
", 
    [part1] => "21",
    [part2] => "40"
);

aoc::test!(
    file = "input/2025/day7.txt",
    [part1] => "1613",
    [part2] => "48021610271997"
);
