use aoc2024::{
    day18::{self, Puzzle},
    Direction, Grid, Point,
};
use image::imageops;

const SCALE: usize = 4;
const FRAME_SKIP: usize = 1;
static mut FRAME_COUNT: usize = 0;

#[allow(clippy::modulo_one)]
fn render(puzzle: &Puzzle, cutoff: usize, points: &[Point], force: bool) {
    let path = unsafe {
        FRAME_COUNT += 1;
        &format!("output/{:0>8}.png", FRAME_COUNT / FRAME_SKIP)
    };
    if !force && unsafe { FRAME_COUNT } % FRAME_SKIP != 0 {
        return;
    }

    println!("Rendering frame: {path}...");

    let image = puzzle.render_image(cutoff, points);
    let image = imageops::resize(
        &image,
        puzzle.width as u32 * SCALE as u32,
        puzzle.height as u32 * SCALE as u32,
        image::imageops::Nearest,
    );
    image.save(path).unwrap();
}

fn main() {
    let input = include_str!("../../input/2024/day18.txt");
    let input = day18::parse(input);

    std::fs::create_dir_all("output").unwrap();

    let end = (input.width - 1, input.height - 1).into();

    let mut previous_best_path = None;

    let mut grid = Grid::new(input.width, input.height);
    for p in input.points.iter().take(input.part1_cutoff) {
        grid.set(*p, true);
    }

    (input.part1_cutoff..).find_map(|cutoff| {
        let new_point = input.points[cutoff - 1];
        grid.set(new_point, true);

        // To be a cutoff, the new point must have exactly two open neighbors
        if new_point
            .neighbors()
            .iter()
            .filter(|&p| grid.get(*p) != Some(&false))
            .count()
            != 2
        {
            return None;
        }

        // And it must have been on the previous best path (if there was one)
        if previous_best_path
            .as_ref()
            .map_or(false, |path: &Vec<Point>| !path.contains(&new_point))
        {
            return None;
        }

        // Verify if the new point *actually* cut us off
        match pathfinding::prelude::astar(
            &Point::ZERO,
            |&point| {
                Direction::all()
                    .iter()
                    .filter_map(|d| {
                        if grid.get(point + *d) == Some(&false) {
                            Some((point + *d, 1))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            },
            |point| point.manhattan_distance(&end),
            |point| *point == end,
        ) {
            Some((path, _)) => {
                render(&input, cutoff, &path, false);
                previous_best_path = Some(path);
                None
            }
            _ => Some(input.points[cutoff - 1]),
        }
    });

    // Render to mp4
    println!("Rendering video...");
    let cmd = "ffmpeg -y \
        -framerate 24 \
        -pattern_type glob \
        -i 'output/*.png' \
        -c:v libx264 \
        -crf 24 \
        -vf format=yuv420p \
        -movflags +faststart \
        day18-part2-v4.mp4";

    match std::process::Command::new("sh").arg("-c").arg(cmd).status() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to run ffmpeg: {:?}", err);
            std::process::exit(1);
        }
    }

    // Clean up
    println!("Cleaning up...");
    std::fs::remove_dir_all("output").unwrap();
}
