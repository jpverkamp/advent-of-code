use aoc2024::Grid;
use image::{imageops, ImageBuffer};

fn render(grid: &Grid<u8>, trail_counts: &Grid<(u128, u128)>, path: &str) {
    println!("Rendering frame: {path}...");

    let mut image = ImageBuffer::new(grid.width as u32, grid.height as u32);

    // Background is grayscale based on heights
    for (p, &height) in grid.iter_enumerate() {
        // Height ranges 0 to 9, map this to 64-196
        let height_color = 64 + height * 16;
        let color = image::Rgb([height_color, height_color, height_color]);
        image.put_pixel(p.x as u32, p.y as u32, color);
    }

    // Any trail counts that are set are highlighted in red based on number of bits set
    // The 16 is a magic number based on the max number of trails to any given point
    for (p, &(a, b)) in trail_counts.iter_enumerate() {
        let count = a.count_ones() as u8 + b.count_ones() as u8;
        let red = 127 + count * 16;

        if count != 0 {
            image.put_pixel(p.x as u32, p.y as u32, image::Rgb([red, 0, 0]));
        }
    }

    let image = imageops::resize(
        &image,
        grid.width as u32 * 8,
        grid.height as u32 * 8,
        image::imageops::Nearest,
    );
    image.save(path).unwrap();
}

fn main() {
    let input = include_str!("../../input/2024/day10.txt");
    let heights = Grid::read(input, &|c| c.to_digit(10).unwrap() as u8);

    std::fs::create_dir_all("output").unwrap();
    let mut frame = 0;
    let frame_skip = 4;

    let mut trail_counts: Grid<(u128, u128)> = Grid::new(heights.width, heights.height);

    // Flag each 9 with a unique bit
    let mut index = 0;
    heights.iter_enumerate().for_each(|(p, &v)| {
        if v == 9 {
            trail_counts.set(
                p,
                if index < 128 {
                    (1 << index, 0)
                } else {
                    (0, 1 << (index - 128))
                },
            );
            index += 1;
        }
    });

    // For each height, we're going to OR the bits of reachable 9s together
    for height in (0..=8).rev() {
        heights.iter_enumerate().for_each(|(p, &v)| {
            if v == height {
                trail_counts.set(
                    p,
                    p.neighbors()
                        .iter()
                        .filter(|&p2| heights.get(*p2).is_some_and(|&v| v == height + 1))
                        .map(|&p2| *trail_counts.get(p2).unwrap())
                        .reduce(|(a1, a2), (b1, b2)| (a1 | b1, a2 | b2))
                        .unwrap_or((0, 0)),
                );

                if frame % frame_skip == 0 {
                    render(
                        &heights,
                        &trail_counts,
                        &format!("output/{:0>8}.png", frame),
                    );
                }
                frame += 1;
            }
        });
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
        day10-part1-dynamic.mp4";

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
