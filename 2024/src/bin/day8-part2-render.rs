use core::ascii;

use aoc2024::{day8::Tile, Grid};

use hsv::hsv_to_rgb;
use image::ImageBuffer;

fn render(grid: &Grid<Tile>, antinodes: &Grid<bool>, chars: &[char], path: String) {
    println!("Rendering frame: {path}...");

    let mut image = ImageBuffer::new(grid.width as u32, grid.height as u32);

    for (point, tile) in grid.iter_enumerate() {
        let color = match tile {
            Tile::Empty => image::Rgb([0, 0, 0]),
            Tile::Tower(c) => {
                let index = chars.iter().position(|x| x == c).unwrap();
                let hue = (index as f64 / chars.len() as f64) * 270.0;
                let rgb = hsv_to_rgb(hue, 1.0, 1.0);
                image::Rgb([rgb.0, rgb.1, rgb.2])
            }
        };

        image.put_pixel(point.x as u32, point.y as u32, color);
    }

    for (point, &is_antinode) in antinodes.iter_enumerate() {
        if is_antinode {
            image.put_pixel(point.x as u32, point.y as u32, image::Rgb([255, 255, 255]));
        }
    }

    image.save(path).unwrap();
}

fn main() {
    std::fs::create_dir_all("output").unwrap();

    let mut frame = 0;

    let input = include_str!("../../input/2024/day8.txt");
    let grid = Grid::read(input, &|c| match c {
        '.' => Tile::Empty,
        _ => Tile::Tower(c),
    });

    let mut all_chars = hashbrown::HashSet::new();
    for tile in grid.iter() {
        if let Tile::Tower(c) = tile {
            all_chars.insert(*c);
        }
    }
    let all_chars = all_chars.into_iter().collect::<Vec<_>>();

    let mut towers = hashbrown::HashMap::new();

    for (point, tile) in grid.iter_enumerate() {
        if let Tile::Tower(c) = tile {
            towers.entry(c).or_insert_with(Vec::new).push(point);
        }
    }

    let mut antinodes = Grid::new(grid.width, grid.height);

    for (_, points) in towers.iter() {
        for p1 in points {
            frame += 1;
            render(
                &grid,
                &antinodes,
                &all_chars,
                format!("output/{:0>8}.png", frame),
            );

            for p2 in points {
                if p1 != p2 {
                    let delta = *p2 - *p1;

                    let mut p = *p1 + delta;
                    while grid.in_bounds(p) {
                        antinodes.set(p, true);
                        p += delta;
                    }

                    let mut p = *p1 - delta;
                    while grid.in_bounds(p) {
                        antinodes.set(p, true);
                        p -= delta;
                    }
                }
            }
        }
    }

    let scale = 10;
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
        day8-part2.mp4"
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
