use aoc2024::{
    day18::{self, Puzzle},
    Direction, Grid, Point,
};
use image::imageops;

const SCALE: usize = 4;
const FRAME_SKIP: usize = 1;
const FRAME_DUP: usize = 4;
static mut FRAME_COUNT: usize = 0;

#[allow(clippy::modulo_one)]
fn render(
    puzzle: &Puzzle,
    cutoff: usize,
    added: &[Point],
    removed: &[Point],
    points: &[Point],
    force: bool,
) {
    let path = unsafe {
        FRAME_COUNT += 1;
        &format!("output/{:0>8}.png", FRAME_COUNT / FRAME_SKIP)
    };
    if !force && unsafe { FRAME_COUNT } % FRAME_SKIP != 0 {
        return;
    }

    println!("Rendering frame: {path}...");

    let mut image = puzzle.render_image(cutoff, points);

    for p in added {
        image.put_pixel(p.x as u32, p.y as u32, image::Rgb([0, 255, 0]));
    }

    for p in removed {
        image.put_pixel(p.x as u32, p.y as u32, image::Rgb([255, 0, 0]));
    }

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

    let mut lower_bound = input.part1_cutoff;
    let mut upper_bound = input.points.len();

    let mut guess = (lower_bound + upper_bound) / 2;
    let mut previous_guess = guess;

    loop {
        let mut grid = Grid::new(input.width, input.height);
        for p in input.points.iter().take(guess) {
            grid.set(*p, true);
        }

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
                for _ in 0..FRAME_DUP {
                    if guess < previous_guess {
                        render(
                            &input,
                            guess,
                            &[],
                            &input.points[guess..previous_guess],
                            &[],
                            false,
                        );
                    } else {
                        render(
                            &input,
                            guess,
                            &input.points[previous_guess..guess],
                            &[],
                            &[],
                            false,
                        );
                    }
                }
                previous_guess = guess;

                if upper_bound - lower_bound <= 1 {
                    render(&input, guess, &[], &[], &path, false);
                    break;
                }
                lower_bound = guess;
                guess = (upper_bound + guess) / 2;
            }
            None => {
                for _ in 0..FRAME_DUP {
                    if guess < previous_guess {
                        render(
                            &input,
                            guess,
                            &[],
                            &input.points[guess..previous_guess],
                            &[],
                            false,
                        );
                    } else {
                        render(
                            &input,
                            guess,
                            &input.points[previous_guess..guess],
                            &[],
                            &[],
                            false,
                        );
                    }
                }
                previous_guess = guess;

                if upper_bound - lower_bound <= 1 {
                    break;
                }
                upper_bound = guess;
                guess = (lower_bound + guess) / 2;
            }
        }
    }

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
        day18-part2-v5.mp4";

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
