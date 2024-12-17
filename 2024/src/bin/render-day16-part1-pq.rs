use aoc2024::{
    day16::{self, Puzzle},
    Direction, Point,
};
use hashbrown::HashSet;
use image::{imageops, ImageBuffer};
use priority_queue::PriorityQueue;

const SCALE: usize = 4;
const FRAME_SKIP: usize = 25;
static mut FRAME_COUNT: usize = 0;

fn render(puzzle: &Puzzle, points: &[Point], force: bool) {
    let path = unsafe {
        FRAME_COUNT += 1;
        &format!("output/{:0>8}.png", FRAME_COUNT / FRAME_SKIP)
    };
    if !force && unsafe { FRAME_COUNT } % FRAME_SKIP != 0 {
        return;
    }

    println!("Rendering frame: {path}...");

    let mut image = ImageBuffer::new(puzzle.walls.width as u32, puzzle.walls.height as u32);

    for (point, tile) in puzzle.walls.iter_enumerate() {
        if *tile {
            image.put_pixel(point.x as u32, point.y as u32, image::Rgb([0, 0, 0]));
        } else {
            image.put_pixel(point.x as u32, point.y as u32, image::Rgb([255, 255, 255]));
        }
    }

    for point in points.iter() {
        let hue = 360.0 * (point.x as f64 / puzzle.walls.width as f64);
        let color = hsv::hsv_to_rgb(hue, 1.0, 1.0);

        image.put_pixel(
            point.x as u32,
            point.y as u32,
            image::Rgb([color.0, color.1, color.2]),
        );
    }

    let image = imageops::resize(
        &image,
        puzzle.walls.width as u32 * SCALE as u32,
        puzzle.walls.height as u32 * SCALE as u32,
        image::imageops::Nearest,
    );
    image.save(path).unwrap();
}

fn main() {
    let input = include_str!("../../input/2024/day16.txt");
    let input = day16::parse(input);

    let mut points_checked = vec![];

    std::fs::create_dir_all("output").unwrap();

    let mut pq = PriorityQueue::new();
    pq.push((input.start, Direction::Right), 0_isize);

    let mut checked = HashSet::new();

    while let Some(((point, direction), cost)) = pq.pop() {
        points_checked.push(point);
        render(&input, &points_checked, false);

        if point == input.end {
            break;
        }

        if !checked.insert((point, direction)) {
            continue;
        }

        // Walk straight
        let new_point = point + direction;
        if input.walls.get(new_point) != Some(&true) {
            pq.push((new_point, direction), cost - 1);
        }

        // Turn left or right
        // Optimize slightly by only queueing a turn if there's no wall
        // TODO: This might fail on the starting condition?

        let new_d = direction.rotate_left();
        if input.walls.get(point + new_d) != Some(&true) {
            pq.push((point, new_d), cost - 1000);
        }

        let new_d = direction.rotate_right();
        if input.walls.get(point + new_d) != Some(&true) {
            pq.push((point, new_d), cost - 1000);
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
        day16-part1-pq.mp4";

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
