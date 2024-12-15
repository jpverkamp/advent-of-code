use aoc2024::day15::{self, State, Tile};
use image::{imageops, ImageBuffer};

fn render(state: &State, path: &str, scale: usize) {
    println!("Rendering frame: {path}...");

    let mut image = ImageBuffer::new(state.tiles.width as u32, state.tiles.height as u32);

    for (point, tile) in state.tiles.iter_enumerate() {
        let color: image::Rgb<u8> = match tile {
            Tile::Empty => image::Rgb([0, 0, 0]),
            Tile::Wall => image::Rgb([255, 255, 255]),
            Tile::Box | Tile::BigBoxLeft => image::Rgb([0, 127, 0]),
            Tile::BigBoxRight => image::Rgb([0, 196, 0]),
        };

        image.put_pixel(point.x as u32, point.y as u32, color);
    }

    image.put_pixel(
        state.position.x as u32,
        state.position.y as u32,
        image::Rgb([255, 0, 255]),
    );

    let image = imageops::resize(
        &image,
        state.tiles.width as u32 * scale as u32,
        state.tiles.height as u32 * scale as u32,
        image::imageops::Nearest,
    );
    image.save(path).unwrap();
}

fn main() {
    let input = include_str!("../../input/2024/day15.txt");
    let input = day15::parse(input);

    std::fs::create_dir_all("output/part1").unwrap();
    std::fs::create_dir_all("output/part2").unwrap();
    std::fs::create_dir_all("output/part2plus").unwrap();

    let frame_skip = 10;

    let mut frame = 0;
    let mut state = input.clone();

    while let Some((_d, _p)) = state.next() {
        if frame % frame_skip == 0 {
            render(
                &state.clone(),
                &format!("output/part1/{:0>8}.png", frame),
                6,
            );
        }
        frame += 1;
    }

    let mut frame = 0;
    let mut state = input.clone_but_wider();

    while let Some((_d, _p)) = state.next() {
        if frame % frame_skip == 0 {
            render(
                &state.clone(),
                &format!("output/part2/{:0>8}.png", frame),
                6,
            );
        }
        frame += 1;
    }

    let mut frame = 0;
    let mut state = input.clone_but_wider().clone_but_wider();

    while let Some((_d, _p)) = state.next() {
        if frame % frame_skip == 0 {
            render(
                &state.clone(),
                &format!("output/part2plus/{:0>8}.png", frame),
                3,
            );
        }
        frame += 1;
    }

    // Render to mp4
    println!("Rendering video...");
    for part in ["1", "2", "2plus"] {
        let cmd = format!(
            "ffmpeg -y \
            -framerate 24 \
            -pattern_type glob \
            -i 'output/part{part}/*.png' \
            -c:v libx264 \
            -crf 24 \
            -vf format=yuv420p \
            -movflags +faststart \
            day15-part{part}.mp4"
        );

        match std::process::Command::new("sh").arg("-c").arg(cmd).status() {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Failed to run ffmpeg: {:?}", err);
                std::process::exit(1);
            }
        }
    }

    // Clean up
    println!("Cleaning up...");
    std::fs::remove_dir_all("output").unwrap();
}
