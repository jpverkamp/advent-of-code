use aoc2024::Grid;
use image::{imageops, ImageBuffer};

fn render(grid: &Grid<u8>, checked: &Grid<bool>, path: &str) {
    println!("Rendering frame: {path}...");

    let mut image = ImageBuffer::new(grid.width as u32, grid.height as u32);

    // Background is grayscale based on heights
    for (p, &height) in grid.iter_enumerate() {
        // Height ranges 0 to 9, map this to 64-196
        let height_color = 64 + height * 16;
        let color = image::Rgb([height_color, height_color, height_color]);
        image.put_pixel(p.x as u32, p.y as u32, color);
    }

    // Outline checked with red
    for (p, &is_checked) in checked.iter_enumerate() {
        if is_checked {
            if grid.get(p).unwrap() == &9 {
                image.put_pixel(p.x as u32, p.y as u32, image::Rgb([255, 0, 0]));
            } else {
                image.put_pixel(p.x as u32, p.y as u32, image::Rgb([127, 0, 0]));
            }
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

    heights
        .iter_enumerate()
        .filter(|(_, &v)| v == 0)
        .for_each(|(p, _)| {
            // For each 0, search for how man 9s are reachable
            let mut checked = Grid::new(heights.width, heights.height);
            let mut queue = vec![p];

            while let Some(p) = queue.pop() {
                if frame % frame_skip == 0 {
                    render(&heights, &checked, &format!("output/{:0>8}.png", frame));
                }
                frame += 1;

                if heights.get(p) == Some(&9) {
                    continue; // no points higher than 9
                }

                p.neighbors()
                    .iter()
                    .filter(|&p2| {
                        heights.in_bounds(*p2)
                            && heights.get(p).unwrap() + 1 == *heights.get(*p2).unwrap()
                    })
                    .for_each(|p2| {
                        if !checked.get(*p2).unwrap_or(&false) {
                            checked.set(*p2, true);
                            queue.push(*p2);
                        }
                    });
            }

            render(&heights, &checked, &format!("output/{:0>8}.png", frame));
            frame += 1;
        });

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
        day10-part1-search.mp4";

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
