use aoc2024::{
    day6::{self, Map, Tile},
    Grid, Point,
};
use image::ImageBuffer;

fn render(grid: &Grid<Tile>, guard: &Point, visited: &[Point], path: String) {
    println!("Rendering frame: {path}...");

    let mut image = ImageBuffer::new(grid.width as u32, grid.height as u32);

    const EMPTY: image::Rgb<u8> = image::Rgb([0, 0, 0]);
    const WALL: image::Rgb<u8> = image::Rgb([255, 255, 255]);
    const GUARD: image::Rgb<u8> = image::Rgb([255, 0, 0]);

    for (point, tile) in grid.iter_enumerate() {
        let color = match tile {
            Tile::Empty => EMPTY,
            Tile::Wall => WALL,
        };

        image.put_pixel(point.x as u32, point.y as u32, color);
    }

    for (i, point) in visited.iter().enumerate() {
        let hue = (i as f64 / visited.len() as f64) * 360.0;
        let rgb = hsv::hsv_to_rgb(hue, 1.0, 1.0);
        let color = image::Rgb([rgb.0, rgb.1, rgb.2]);
        image.put_pixel(point.x as u32, point.y as u32, color);
    }

    image.put_pixel(guard.x as u32, guard.y as u32, GUARD);

    image.save(path).unwrap();
}

fn main() {
    std::fs::create_dir_all("output").unwrap();

    let mut frame = 0;
    const FRAME_SKIP: u32 = 10;

    let input = include_str!("../../input/2024/day6.txt");
    let map = day6::parse(input);

    let Map {
        mut guard,
        mut facing,
        grid,
    } = map;

    let mut visited = Vec::new();
    visited.push(guard);

    while grid.in_bounds(guard) {
        frame += 1;
        if frame % FRAME_SKIP == 0 {
            render(&grid, &guard, &visited, format!("output/{:0>8}.png", frame));
        }

        match grid.get(guard + facing) {
            Some(Tile::Empty) => {
                guard += facing.into();
                visited.push(guard);
            }
            Some(Tile::Wall) => {
                facing = facing.rotate_cw();
            }
            None => break,
        }
    }

    let scale = 2;
    let output_width = grid.width * scale;
    let output_height = grid.height * scale;

    // Render to mp4
    println!("Rendering video...");
    let cmd = format!(
        "ffmpeg -y \
        -framerate 24 \
        -pattern_type glob \
        -i 'output/*.png' \
        -s {output_width}:{output_height} \
        -sws_flags neighbor \
        -c:v libx264 \
        -crf 24 \
        -vf format=yuv420p \
        -movflags +faststart \
        day8-part1.mp4"
    );

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
