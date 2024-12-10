use aoc2024::Grid;
use image::{imageops, ImageBuffer};

fn render(grid: &Grid<u8>, ratings: &Grid<u32>, path: &str) {
    println!("Rendering frame: {path}...");

    let mut image = ImageBuffer::new(grid.width as u32, grid.height as u32);

    // Background is grayscale based on heights
    for (p, &height) in grid.iter_enumerate() {
        // Height ranges 0 to 9, map this to 64-196
        let height_color = 64 + height * 16;
        let color = image::Rgb([height_color, height_color, height_color]);
        image.put_pixel(p.x as u32, p.y as u32, color);
    }

    let max_count = 32;

    // Any trail counts that are set are highlighted in red based on number of bits set
    // The 16 is a magic number based on the max number of trails to any given point
    for (p, &v) in ratings.iter_enumerate() {
        let hue = (v as f64 / max_count as f64) * 360.0;
        let color = hsv::hsv_to_rgb(hue, 1.0, 0.5);

        if v != 0 {
            image.put_pixel(
                p.x as u32,
                p.y as u32,
                image::Rgb([color.0, color.1, color.2]),
            );
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

    let mut ratings = Grid::new(heights.width, heights.height);

    // All 9s can be reached one way
    heights.iter_enumerate().for_each(|(p, &v)| {
        if v == 9 {
            ratings.set(p, 1);
        }
    });

    // For each height, we're going to sum the ratings of all points one higher
    for height in (0..=8).rev() {
        heights.iter_enumerate().for_each(|(p, &v)| {
            if v == height {
                ratings.set(
                    p,
                    p.neighbors()
                        .iter()
                        .filter(|&p2| heights.get(*p2).is_some_and(|&v| v == height + 1))
                        .map(|p2| ratings.get(*p2).unwrap_or(&0))
                        .sum(),
                );

                if frame % frame_skip == 0 {
                    render(&heights, &ratings, &format!("output/{:0>8}.png", frame));
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
        day10-part2.mp4";

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
