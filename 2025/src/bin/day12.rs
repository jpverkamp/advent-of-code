use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};

use aoc2025::grid::Grid;
use rayon::prelude::*;

aoc::main!(day12);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Tile {
    id: usize,
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl Tile {
    fn rotate_cw(&self) -> Self {
        let mut new_data = vec![false; self.data.len()];

        for y in 0..self.height {
            for x in 0..self.width {
                let old_index = x + y * self.width;
                let new_x = self.height - 1 - y;
                let new_y = x;
                let new_index = new_x + new_y * self.height;
                new_data[new_index] = self.data[old_index];
            }
        }

        Tile {
            id: self.id,
            width: self.height,
            height: self.width,
            data: new_data,
        }
    }

    fn flip_h(&self) -> Self {
        let mut new_data = vec![false; self.data.len()];

        for y in 0..self.height {
            for x in 0..self.width {
                let old_index = x + y * self.width;
                let new_x = self.width - 1 - x;
                let new_y = y;
                let new_index = new_x + new_y * self.width;
                new_data[new_index] = self.data[old_index];
            }
        }

        Tile {
            id: self.id,
            width: self.width,
            height: self.height,
            data: new_data,
        }
    }

    fn variations(&self) -> Vec<Tile> {
        let mut vars = Vec::new();
        let mut current = self.clone();
        vars.push(current.clone());

        for _ in 0..4 {
            let rotated = current.rotate_cw();
            if !vars.contains(&rotated) {
                vars.push(rotated.clone());
            }

            let flipped = current.flip_h();
            if !vars.contains(&flipped) {
                vars.push(flipped.clone());
            }

            current = current.rotate_cw();
        }

        vars
    }
}

impl From<&str> for Tile {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let id = lines
            .next()
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut data = Vec::new();

        let mut height = 0;
        while let Some(line) = lines.next()
            && !line.trim().is_empty()
        {
            height += 1;

            for c in line.chars() {
                data.push(c == '#');
            }
        }
        let width = data.len() / height;

        Tile {
            id,
            width,
            height,
            data,
        }
    }
}

fn id_to_char(id: usize) -> char {
    if id < 26 {
        ((id as u8) + b'A') as char
    } else if id < 52 {
        (((id - 26) as u8) + b'a') as char
    } else {
        panic!("Too many tile IDs to convert to char")
    }
}

fn stringify_tile(tile: &Tile) -> String {
    let mut result = String::new();

    for y in 0..tile.height {
        for x in 0..tile.width {
            let c = if tile.data[x + y * tile.width] {
                id_to_char(tile.id)
            } else {
                '.'
            };
            result.push(c);
        }
        result.push('\n');
    }

    result
}

fn stringify_grid(grid: &Grid<Option<usize>>) -> String {
    let mut result = String::new();

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            match grid.get(x as isize, y as isize).unwrap() {
                Some(id) => result.push(id_to_char(id)),
                None => result.push('.'),
            }
        }
        result.push('\n');
    }

    result
}

#[tracing::instrument(skip(grid, tile), fields(tile_id = tile.id, x, y), ret)]
fn try_place(grid: &mut Grid<Option<usize>>, tile: &Tile, x: isize, y: isize) -> bool {
    if log::log_enabled!(log::Level::Debug) {
        log::debug!(
            "Try place tile:\n{}  \non:\n{}",
            stringify_tile(tile),
            stringify_grid(grid)
        );
    }

    for ty in 0..tile.height as isize {
        for tx in 0..tile.width as isize {
            let gx = x + tx;
            let gy = y + ty;

            if gx < 0 || gy < 0 || gx >= grid.width() || gy >= grid.height() {
                return false;
            }

            if !tile.data[(tx as usize) + (ty as usize) * tile.width] {
                continue;
            }

            let grid_value = grid.get(gx, gy).unwrap();
            if grid_value.is_some() {
                return false;
            }
        }
    }

    for ty in 0..tile.height as isize {
        for tx in 0..tile.width as isize {
            let gx = x + tx;
            let gy = y + ty;

            if tile.data[(tx as usize) + (ty as usize) * tile.width] {
                grid.set(gx, gy, Some(tile.id));
            }
        }
    }

    true
}

#[tracing::instrument(skip(grid, tiles, memo, memo_stats, last_debug), ret)]
fn can_place(
    grid: &Grid<Option<usize>>,
    tiles: &Vec<Tile>,
    counts: &Vec<usize>,
    memo: Arc<Mutex<HashMap<(Grid<Option<usize>>, Vec<usize>), bool>>>,
    memo_stats: Arc<Mutex<(usize, usize)>>,
    last_debug: Arc<Mutex<Instant>>,
) -> bool {
    let cache_key = (grid.clone(), counts.clone());
    if log::log_enabled!(log::Level::Info) {
        if log::log_enabled!(log::Level::Debug)
            || last_debug.lock().unwrap().elapsed().as_secs() >= 5
        {
            log::info!(
                "Current progress [counts: {counts:?}, memo size: {}, hits: {}, misses: {}]:\n{}",
                memo.lock().unwrap().len(),
                memo_stats.lock().unwrap().0,
                memo_stats.lock().unwrap().1,
                stringify_grid(grid)
            );
            *last_debug.lock().unwrap() = Instant::now();
        }
    }

    if memo.lock().unwrap().contains_key(&cache_key) {
        memo_stats.lock().unwrap().0 += 1;
        return memo.lock().unwrap()[&cache_key];
    } else {
        memo_stats.lock().unwrap().1 += 1;
    }

    if counts.iter().all(|&c| c == 0) {
        memo.lock().unwrap().insert(cache_key, true);
        return true;
    }

    // If the sum of tiles we have left is not enough to fill the remaining empty cells, fail early
    let empty_cells = grid.iter().filter(|(_, _, v)| v.is_none()).count();
    let tiles_left = counts
        .iter()
        .zip(tiles.iter())
        .map(|(&c, t)| c * t.data.iter().filter(|&&b| b).count())
        .sum::<usize>();
    if tiles_left > empty_cells {
        memo.lock().unwrap().insert(cache_key, false);
        return false;
    }

    // If an entire edge row or column is full, we can remove it to reduce the problem size
    if (0..grid.width() as isize).all(|x| grid.get(x, 0).unwrap().is_some()) {
        let mut new_grid = grid.clone();
        new_grid.drop_row(0);
        return can_place(&new_grid, tiles, counts, memo, memo_stats, last_debug);
    }
    if (0..grid.width() as isize).all(|x| grid.get(x, grid.height() - 1).unwrap().is_some()) {
        let mut new_grid = grid.clone();
        new_grid.drop_row(grid.height() - 1);
        return can_place(&new_grid, tiles, counts, memo, memo_stats, last_debug);
    }
    if (0..grid.height() as isize).all(|y| grid.get(0, y).unwrap().is_some()) {
        let mut new_grid = grid.clone();
        new_grid.drop_column(0);
        return can_place(&new_grid, tiles, counts, memo, memo_stats, last_debug);
    }
    if (0..grid.height() as isize).all(|y| grid.get(grid.width() - 1, y).unwrap().is_some()) {
        let mut new_grid = grid.clone();
        new_grid.drop_column(grid.width() - 1);
        return can_place(&new_grid, tiles, counts, memo, memo_stats, last_debug);
    }

    // Always try to place the first tile we have available
    let tile_index = counts.iter().position(|&c| c > 0).unwrap();
    let tile = &tiles[tile_index];

    for y in 0..grid.height() as isize {
        for x in 0..grid.width() as isize {
            for variation in tile.variations() {
                let mut grid_clone = grid.clone();
                if try_place(&mut grid_clone, &variation, x, y) {
                    let mut new_counts = counts.clone();
                    new_counts[tile_index] -= 1;

                    if can_place(
                        &grid_clone,
                        tiles,
                        &new_counts,
                        memo.clone(),
                        memo_stats.clone(),
                        last_debug.clone(),
                    ) {
                        memo.lock().unwrap().insert(cache_key, true);
                        return true;
                    }
                }
            }
        }
    }

    memo.lock().unwrap().insert(cache_key, false);
    false
}

#[aoc::register]
fn part1(input: &str) -> impl Into<String> {
    let mut tiles = Vec::new();

    let mut remaining_input = input;
    while let Some(next_end) = remaining_input.find("\n\n") {
        let tile_str = &remaining_input[..next_end];
        let tile = Tile::from(tile_str);
        tiles.push(tile);
        remaining_input = &remaining_input[next_end + 2..];
    }

    let memo = Arc::new(Mutex::new(HashMap::default()));
    let memo_stats = Arc::new(Mutex::new((0, 0)));

    remaining_input
        .lines()
        .enumerate()
        .par_bridge()
        .filter(|(line_index, line)| {
            let (size, rest) = line.split_once(": ").unwrap();
            let (width, height) = size.split_once('x').unwrap();
            let width: usize = width.parse().unwrap();
            let height: usize = height.parse().unwrap();

            log::info!("Line {line_index}: {width}x{height}");

            let counts: Vec<usize> = rest.split(' ').map(|part| part.parse().unwrap()).collect();
            let initial_grid = Grid::new(width, height, None);
            let last_debug = Arc::new(Mutex::new(Instant::now()));

            can_place(
                &initial_grid,
                &tiles,
                &counts,
                memo.clone(),
                memo_stats.clone(),
                last_debug.clone(),
            )
        })
        .count()
        .to_string()
}

#[aoc::register_render(scale = 8)]
fn part1_render(input: &str) {
    let mut tiles = Vec::new();

    let mut remaining_input = input;
    while let Some(next_end) = remaining_input.find("\n\n") {
        let tile_str = &remaining_input[..next_end];
        let tile = Tile::from(tile_str);
        tiles.push(tile);
        remaining_input = &remaining_input[next_end + 2..];
    }

    fn seeded_random_rgb(seed: usize) -> (u8, u8, u8) {
        let mut state = seed as u64;
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let r = ((state >> 32) & 0xFF) as u8;
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let g = ((state >> 32) & 0xFF) as u8;
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let b = ((state >> 32) & 0xFF) as u8;
        (r, g, b)
    }

    fn can_place_render(
        grid: &Grid<Option<usize>>,
        tiles: &Vec<Tile>,
        counts: &Vec<usize>,
    ) -> bool {
        // If the sum of tiles we have left is not enough to fill the remaining empty cells, fail early
        let empty_cells = grid.iter().filter(|(_, _, v)| v.is_none()).count();
        let tiles_left = counts
            .iter()
            .zip(tiles.iter())
            .map(|(&c, t)| c * t.data.iter().filter(|&&b| b).count())
            .sum::<usize>();
        if tiles_left > empty_cells {
            return false;
        }

        aoc::render_frame!(64, 64, |x, y| {
            // Center the grid in the frame
            let grid_width = grid.width() as isize;
            let grid_height = grid.height() as isize;
            let offset_x = (64 - grid_width) / 2;
            let offset_y = (64 - grid_height) / 2;

            // Offset points off the grid are gray
            if x < offset_x
                || y < offset_y
                || x >= offset_x + grid_width
                || y >= offset_y + grid_height
            {
                return (128, 128, 128);
            }

            match grid.get(x as isize - offset_x, y as isize - offset_y) {
                Some(Some(id)) => seeded_random_rgb(id),
                Some(None) | None => (0, 0, 0),
            }
        });

        if counts.iter().all(|&c| c == 0) {
            return true;
        }

        // // Always try to place the first tile we have available
        let tile_index = counts.iter().position(|&c| c > 0).unwrap();
        let tile = &tiles[tile_index];

        // Place whichever tile we have the most of left to place
        // let tile_index = counts
        //     .iter()
        //     .enumerate()
        //     .max_by_key(|&(_, &c)| c)
        //     .map(|(i, _)| i)
        //     .unwrap();
        // let tile = &tiles[tile_index];

        // // Choose the next tile to place randomly
        // use rand::Rng;
        // let mut rng = rand::rng();
        // let mut tile_index = None;
        // while tile_index.is_none() {
        //     let candidate = rng.random_range(0..tiles.len());
        //     if counts[candidate] > 0 {
        //         tile_index = Some(candidate);
        //     }
        // }
        // let tile_index = tile_index.unwrap();
        // let tile = &tiles[tile_index];

        for y in 0..grid.height() as isize {
            for x in 0..grid.width() as isize {
                for variation in tile.variations() {
                    let mut grid_clone = grid.clone();
                    if try_place(&mut grid_clone, &variation, x, y) {
                        let mut new_counts = counts.clone();
                        new_counts[tile_index] -= 1;

                        if can_place_render(&grid_clone, tiles, &new_counts) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    remaining_input
        .lines()
        .enumerate()
        .take(32)
        .for_each(|(line_index, line)| {
            let (size, rest) = line.split_once(": ").unwrap();
            let (width, height) = size.split_once('x').unwrap();
            let width: usize = width.parse().unwrap();
            let height: usize = height.parse().unwrap();

            log::info!("Line {line_index}: {width}x{height}");

            let counts: Vec<usize> = rest.split(' ').map(|part| part.parse().unwrap()).collect();
            let initial_grid = Grid::new(width, height, None);

            can_place_render(&initial_grid, &tiles, &counts);
        });
}

#[aoc::register]
fn part1_trivial(input: &str) -> impl Into<String> {
    let mut tiles = Vec::new();

    let mut remaining_input = input;
    while let Some(next_end) = remaining_input.find("\n\n") {
        let tile_str = &remaining_input[..next_end];
        let tile = Tile::from(tile_str);
        tiles.push(tile);
        remaining_input = &remaining_input[next_end + 2..];
    }

    // This is the case for given input, this won't generalize to all possible sizes
    assert!(
        tiles.iter().all(|t| t.width == 3 && t.height == 3),
        "Only trivial 3x3 tiles supported"
    );

    let mut count = 0;

    for (line_index, line) in remaining_input.lines().enumerate() {
        let (size, rest) = line.split_once(": ").unwrap();
        let (width, height) = size.split_once('x').unwrap();
        let width: usize = width.parse().unwrap();
        let height: usize = height.parse().unwrap();

        log::debug!("Line {line_index}: {width}x{height}");

        let counts: Vec<usize> = rest.split(' ').map(|part| part.parse().unwrap()).collect();

        // Trivially allowed: all tiles fit into their own 3x3 cell
        let tiles_allowed = (width / 3) * (height / 3);
        let total_tiles_requested: usize = counts.iter().sum();
        if total_tiles_requested <= tiles_allowed {
            count += 1;
            continue;
        }

        // Trivially impossible: not enough cells to hold the tiles no matter what
        let total_hashes_requested: usize = counts
            .iter()
            .zip(tiles.iter())
            .map(|(&c, t)| c * t.data.iter().filter(|&&b| b).count())
            .sum();
        let total_hashes_possible = width * height;
        if total_hashes_requested > total_hashes_possible {
            continue;
        }

        panic!("Required non-trivial check for line {line_index}");
    }

    count.to_string()
}

aoc::test!(
    text = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
", 
    [part1] => "2"
);

aoc::test!(
    file = "input/2025/day12.txt",
    [part1_trivial] => "575"
);
