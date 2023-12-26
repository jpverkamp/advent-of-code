use fxhash::FxHashMap;
use image::ImageBuffer;
use std::{fs::create_dir_all, io};

use day14::types::*;

fn main() {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let mut platform = PlatformV2::from(Platform::from(input.as_str()));

    let mut seen = FxHashMap::default();

    const TARGET: i32 = 1_000_000_000;

    create_dir_all("frames/cycle").ok();
    create_dir_all("frames/direction").ok();

    for cycle in 0..=TARGET {
        // Render the current (partial) cycle as an image
        {
            let filename = format!("frames/cycle/{:04}.png", cycle);
            println!("Rendering {}", filename);

            let width = platform.bounds.max_x - platform.bounds.min_x + 1;
            let height = platform.bounds.max_y - platform.bounds.min_y + 1;

            let image = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
                let p = Point {
                    x: x as isize + platform.bounds.min_x,
                    y: y as isize + platform.bounds.min_y,
                };

                if platform.round_rocks.contains(&p) {
                    image::Rgb([255_u8, 255, 255])
                } else if platform.occupied.contains(&p) {
                    image::Rgb([127, 127, 127])
                } else {
                    image::Rgb([0, 0, 0])
                }
            });
            image.save(filename).expect("save frame");
        }

        // Check if we've seen this platform state before (it's deterministic, thus cycling)
        // Keep going until the cycle is in the same phase as the TARGET
        let key = platform.to_string();
        if let Some(cycle_start) = seen.get(&key) {
            let cycle_length = cycle - cycle_start;

            if cycle > 250 && (TARGET - cycle_start) % cycle_length == 0 {
                break;
            }
        }
        seen.insert(key, cycle);

        // The rocks will slide N, W, S, E
        for (_subcycle, direction) in [Point::NORTH, Point::WEST, Point::SOUTH, Point::EAST]
            .into_iter()
            .enumerate()
        {
            // Render the current (partial) cycle as an image
            {
                let filename = format!("frames/direction/{:04}.png", cycle * 4 + _subcycle as i32);
                println!("Rendering {}", filename);

                let width = platform.bounds.max_x - platform.bounds.min_x + 1;
                let height = platform.bounds.max_y - platform.bounds.min_y + 1;

                let image = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
                    let p = Point {
                        x: x as isize + platform.bounds.min_x,
                        y: y as isize + platform.bounds.min_y,
                    };

                    if platform.round_rocks.contains(&p) {
                        image::Rgb([255_u8, 255, 255])
                    } else if platform.occupied.contains(&p) {
                        image::Rgb([127, 127, 127])
                    } else {
                        image::Rgb([0, 0, 0])
                    }
                });
                image.save(filename).expect("save frame");
            }

            // Let the rocks slide until they stop moving
            loop {
                let mut changed = false;

                for i in 0..platform.round_rocks.len() {
                    let r = platform.round_rocks[i];

                    // Move in that direction until we hit something (or a wall)
                    let mut next = r;
                    loop {
                        next = next + direction;

                        if !platform.bounds.contains(&next) || platform.occupied.contains(&next) {
                            // Have to step back to the last valid point
                            next = next - direction;
                            break;
                        }
                    }

                    // If we didn't actually move, do nothing
                    if next == r {
                        continue;
                    }

                    // If we get here, we can move; do it
                    platform.round_rocks[i].x = next.x;
                    platform.round_rocks[i].y = next.y;

                    platform.occupied.remove(&r);
                    platform.occupied.insert(next);

                    changed = true;
                }

                if !changed {
                    break;
                }
            }
        }
    }

    // Calculate final score
    platform
        .round_rocks
        .iter()
        .map(|r| platform.bounds.max_y - r.y + 1)
        .sum::<isize>();

    // Do final rendering
    use std::process::Command;
    let commands = vec![
        // Cycle animation
        "ffmpeg -y -framerate 240 -i 'frames/cycle/%04d.png' -vf 'scale=iw*4:ih*4:flags=neighbor' -c:v libx264 -r 30 aoc23-14-cycle.raw.mp4",
        "ffmpeg -y -i aoc23-14-cycle.raw.mp4 -c:v libx264 -preset slow -crf 20 -vf format=yuv420p -movflags +faststart aoc23-14-cycle.mp4",

        // Direction animation
        "ffmpeg -y -framerate 240 -i 'frames/direction/%04d.png' -vf 'scale=iw*4:ih*4:flags=neighbor' -c:v libx264 -r 30 aoc23-14-direction.raw.mp4",
        "ffmpeg -y -i aoc23-14-direction.raw.mp4 -c:v libx264 -preset slow -crf 20 -vf format=yuv420p -movflags +faststart aoc23-14-direction.mp4",

        // Clean up
        "rm -rf frames/",
        "rm aoc23-14-cycle.raw.mp4",
        "rm aoc23-14-direction.raw.mp4",
    ];

    for cmd in commands.into_iter() {
        println!("$ {}", cmd);
        let mut child = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .spawn()
            .expect("command failed");
        child.wait().expect("process didn't finish");
    }
}
