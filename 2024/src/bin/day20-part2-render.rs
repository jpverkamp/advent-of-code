use aoc2024::{day20::Puzzle, Direction, Point};
use hashbrown::HashMap;
use image::imageops;

use pathfinding::prelude::{astar, dijkstra_all};

const SCALE: usize = 4;
static mut FRAME_COUNT: usize = 0;
static mut NEXT_TO_RENDER: f64 = 0.0;

#[allow(clippy::modulo_one)]
fn render(
    puzzle: &Puzzle,
    best_path: &[Point],
    progress: usize,
    scanning: &[Point],
    skips: &HashMap<Point, usize>,
    force: bool,
) {
    let path = unsafe {
        let path = format!("output/{:0>8}.png", FRAME_COUNT);

        FRAME_COUNT += 1;

        let count = FRAME_COUNT as f64;
        let count = count.powf(0.45);

        if !force && count < NEXT_TO_RENDER {
            return;
        } else {
            NEXT_TO_RENDER += 1.0;
        }

        println!(
            "Rendering frame: {path}... (progress={}/{}, next={})",
            progress,
            best_path.len(),
            NEXT_TO_RENDER
        );
        path
    };

    std::fs::create_dir_all("output").unwrap();

    let mut image = image::ImageBuffer::new(puzzle.walls.width as u32, puzzle.walls.height as u32);

    // Draw walls
    for (p, &wall) in puzzle.walls.iter_enumerate() {
        let color = if wall {
            image::Rgb([0, 0, 0])
        } else {
            image::Rgb([255, 255, 255])
        };
        image.put_pixel(p.x as u32, p.y as u32, color);
    }

    // Draw path up to the progress point
    for (i, p) in best_path.iter().enumerate() {
        #[allow(clippy::comparison_chain)]
        let color = if i < progress {
            image::Rgb([128, 128, 128])
        } else if progress == i {
            image::Rgb([255, 255, 255])
        } else {
            image::Rgb([64, 64, 64])
        };

        image.put_pixel(p.x as u32, p.y as u32, color);
    }

    // Highlight all found skips
    let max_skips = skips.values().max().unwrap_or(&0).max(&10);
    for (point, count) in skips.iter() {
        let hue = 360.0 * (*count as f64 / *max_skips as f64);
        let rgb = hsv::hsv_to_rgb(hue, 1.0, 1.0);

        image.put_pixel(
            point.x as u32,
            point.y as u32,
            image::Rgb::<u8>([rgb.0, rgb.1, rgb.2]),
        );
    }

    // Highlight the radius we're scanning
    for point in scanning.iter() {
        image.put_pixel(
            point.x as u32,
            point.y as u32,
            image::Rgb::<u8>([0, 196, 0]),
        );
    }

    // Write the active point one last time since it's covered now
    if progress < best_path.len() {
        image.put_pixel(
            best_path[progress].x as u32,
            best_path[progress].y as u32,
            image::Rgb::<u8>([255, 255, 255]),
        );
    }

    let image = imageops::resize(
        &image,
        image.width() * SCALE as u32,
        image.height() * SCALE as u32,
        image::imageops::Nearest,
    );
    image.save(path).unwrap();
}

fn main() {
    let input = include_str!("../../input/2024/day20.txt");
    let input = aoc2024::day20::parse(input);

    let skiplength = 20_i32;
    let cutoff = 100;

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
    let mut skip_counts = HashMap::new();
    for (i, p) in path.iter().enumerate() {
        // Populate scanning radius
        let mut scanning: Vec<Point> = vec![];
        for xd in -skiplength..=skiplength {
            for yd in -skiplength..=skiplength {
                // Ignore skipping to yourself or skipping too far
                if xd == 0 && yd == 0 || xd.abs() + yd.abs() != skiplength {
                    continue;
                }

                let d: Point = (xd, yd).into();
                let p = *p + d;
                if input.walls.get(p).is_some() {
                    scanning.push(p);
                }
            }
        }

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

                // if let Some(index) = scanning.iter().position(|p| *p == p2) {
                //     scanning.remove(index);
                // }

                // Cannot end on a wall
                // This is covered by the distanced map, but this lookup is faster
                // With: 112.44 ms, Without: 189.61 ms
                if input.walls.get(p2) != Some(&false) {
                    continue;
                }

                // Cannot get from the target to the end
                if !distances.contains_key(&p2) {
                    continue;
                }

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
                // Add one to that skip
                *skip_counts.entry(p2).or_insert(0) += 1;
                render(&input, &path, i, &scanning, &skip_counts, false);
            }
        }
    }

    render(&input, &path, path.len(), &[], &skip_counts, true);

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
        day20-part2.mp4";

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
