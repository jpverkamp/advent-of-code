use anyhow::Result;
use std::{fs::create_dir_all, io};

use day16::types::*;

use grid::Grid;
use point::Point;

// #[aoc_test("data/test/16.txt", "")]
// #[aoc_test("data/16.txt", "")]
#[allow(dead_code)]
fn main() -> Result<()> {
    let stdin = io::stdin();

    let input = io::read_to_string(stdin.lock())?;
    let mirrors = Grid::read(&input, |c| match c {
        '|' => Some(Mirror::VerticalSplitter),
        '-' => Some(Mirror::HorizontalSplitter),
        '/' => Some(Mirror::ForwardReflector),
        '\\' => Some(Mirror::BackwardReflector),
        _ => None,
    });

    let result = illuminate(&mirrors, (Point::new(0, 0), Direction::East))
        .iter()
        .count();

    // Do final rendering
    use std::process::Command;
    let commands = vec![
        // Cycle animation
        "ffmpeg -y -framerate 240 -i 'frames/%04d.png' -vf 'scale=iw*4:ih*4:flags=neighbor' -c:v libx264 -r 30 aoc23-16.raw.mp4",
        "ffmpeg -y -i aoc23-16.raw.mp4 -c:v libx264 -preset slow -crf 20 -vf format=yuv420p -movflags +faststart aoc23-16.mp4",

        // Clean up
        "rm -rf frames/",
        "rm aoc23-16.raw.mp4",
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

    println!("{result}");
    Ok(())
}

const RENDER_FRAMES: usize = 1000;

pub(crate) fn illuminate(mirrors: &Grid<Mirror>, start: (Point, Direction)) -> Grid<bool> {
    use Direction::*;
    use Mirror::*;

    let mut queue = Vec::new();
    queue.push(vec![start]);

    // let mut visited = fxhash::FxHashSet::default();
    let mut illuminated = Grid::new();

    let background = mirrors.to_image(image::Rgba([0, 0, 0, 0]), |_| {
        image::Rgba([255, 255, 255, 255])
    });
    let barely_black = image::RgbaImage::from_pixel(
        background.width(),
        background.height(),
        image::Rgba([0, 0, 0, 8]),
    );
    let mut foreground = image::RgbaImage::new(background.width(), background.height());

    let mut frame = 0;
    while let Some(points) = queue.pop() {
        if frame > RENDER_FRAMES {
            break;
        }

        // Render the current frame
        {
            frame += 1;
            let filename = format!("frames/{:04}.png", frame);
            println!("Rendering {}", filename);

            create_dir_all("frames").ok();

            let mut frame = image::RgbaImage::new(background.width(), background.height());

            // Darken the previous foreground frames slightly
            image::imageops::overlay(&mut foreground, &barely_black, 0, 0);

            for (p, _) in &points {
                if mirrors.bounds.contains(p) {
                    foreground.put_pixel(p.x as u32, p.y as u32, image::Rgba([255, 0, 0, 255]));
                }
            }

            image::imageops::overlay(&mut frame, &foreground, 0, 0);
            image::imageops::overlay(&mut frame, &background, 0, 0);
            frame.save(filename).unwrap();
        }

        let mut next = vec![];
        for (p, d) in points {
            // Ignore points that have gone out of bounds
            if !mirrors.bounds.contains(&p) {
                continue;
            }

            // Don't evaluate the same point + direction more than once
            // if visited.contains(&(p, d)) {
            //     continue;
            // }
            // visited.insert((p, d));

            illuminated.insert(p, true);

            match (mirrors.get(&p), d) {
                // If you hit a splitter side on (ex >-), you continue in the same direction.
                (Some(VerticalSplitter), North) | (Some(VerticalSplitter), South) => {
                    next.push((p + d.into(), d));
                }
                (Some(HorizontalSplitter), East) | (Some(HorizontalSplitter), West) => {
                    next.push((p + d.into(), d));
                }
                // Otherwise (ex >|), split to the two directions it points
                (Some(VerticalSplitter), _) => {
                    next.push((p + North.into(), North));
                    next.push((p + South.into(), South));
                }
                (Some(HorizontalSplitter), _) => {
                    next.push((p + East.into(), East));
                    next.push((p + West.into(), West));
                }
                // Diagonal reflectors just change, so >\ goes South, >/ goes North etc
                (Some(ForwardReflector), North) => next.push((p + East.into(), East)),
                (Some(ForwardReflector), East) => next.push((p + North.into(), North)),
                (Some(ForwardReflector), South) => next.push((p + West.into(), West)),
                (Some(ForwardReflector), West) => next.push((p + South.into(), South)),

                (Some(BackwardReflector), North) => next.push((p + West.into(), West)),
                (Some(BackwardReflector), East) => next.push((p + South.into(), South)),
                (Some(BackwardReflector), South) => next.push((p + East.into(), East)),
                (Some(BackwardReflector), West) => next.push((p + North.into(), North)),
                // If there's nothing there, keep going
                (None, _) => next.push((p + d.into(), d)),
            }
        }
        if !next.is_empty() {
            queue.push(next);
        }
    }

    illuminated
}
