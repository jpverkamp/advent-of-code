use aoc2024::{
    day18::{self, Puzzle},
    Direction, Point,
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

    (input.part1_cutoff..).find(|cutoff| {
        match pathfinding::prelude::astar(
            &Point::ZERO,
            |&point| {
                Direction::all()
                    .iter()
                    .filter_map(|d| {
                        let new_p = point + *d;
                        if new_p.x < 0
                            || new_p.y < 0
                            || new_p.x >= input.width as i32
                            || new_p.y >= input.height as i32
                            || input.points[..*cutoff].contains(&new_p)
                        {
                            None
                        } else {
                            Some((new_p, 1))
                        }
                    })
                    .collect::<Vec<_>>()
            },
            |point| point.manhattan_distance(&end),
            |point| *point == end,
        ) {
            Some((path, _)) => {
                render(&input, *cutoff, &path, false);
                false
            }
            _ => true,
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
        day18-part2-v1.mp4";

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
