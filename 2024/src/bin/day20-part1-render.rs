use aoc2024::{day20::Puzzle, Direction, Grid, Point};
use image::imageops;

use pathfinding::prelude::astar;

const SCALE: usize = 4;
static mut FRAME_SKIP: usize = 1;
static mut FRAME_COUNT: usize = 0;

#[allow(clippy::modulo_one)]
fn render(
    puzzle: &Puzzle,
    best_path: &[Point],
    progress: usize,
    scanning: Option<(Point, bool)>,
    skips: &[Point],
    force: bool,
) {
    let path = unsafe {
        let path = format!("output/{:0>8}.png", FRAME_COUNT);

        FRAME_COUNT += 1;
        if !force && FRAME_COUNT % FRAME_SKIP != 0 {
            return;
        }

        if FRAME_COUNT % 100 == 0 {
            FRAME_SKIP += 1;
        }

        path
    };

    std::fs::create_dir_all("output").unwrap();
    println!("Rendering frame: {path}...");

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
    for skip in skips.iter() {
        image.put_pixel(skip.x as u32, skip.y as u32, image::Rgb::<u8>([0, 255, 0]));
    }

    // Highlight point we're scanning
    if let Some((point, is_good)) = scanning {
        image.put_pixel(
            point.x as u32,
            point.y as u32,
            if is_good {
                image::Rgb::<u8>([0, 255, 0])
            } else {
                image::Rgb::<u8>([255, 0, 0])
            },
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

    let cutoff = 100;
    let mut skips = vec![];

    // Now, for each point in that path, see if we can skip to a point further along the path
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
                skips.push(*p + d);
                render(&input, &path, i, Some((*p + d, true)), &skips, false);
            } else {
                render(&input, &path, i, Some((*p + d, false)), &skips, false);
            }
        }
    }
    render(&input, &path, path.len(), None, &skips, true);

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
        day20-part1.mp4";

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
