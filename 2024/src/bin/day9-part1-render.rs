use aoc2024::day9::{Block, Disk};
use hsv::hsv_to_rgb;
use image::ImageBuffer;

fn render(disk: &Disk, path: &str, left_index: usize, right_index: usize) {
    println!("Rendering frame: {path}...");

    let grid_size = 4;
    let max_width = 1200;
    let blocks_per_row = max_width / grid_size;

    let block_count = disk.blocks.len();
    let row_count = block_count / blocks_per_row + 1;

    let mut image = ImageBuffer::new(
        (blocks_per_row * grid_size) as u32,
        (row_count * grid_size) as u32,
    );

    for (index, block) in disk.blocks.iter().enumerate() {
        let x = (index % blocks_per_row) * grid_size;
        let y = (index / blocks_per_row) * grid_size;

        let color = match block {
            Block::Empty => image::Rgb([0, 0, 0]),
            Block::File(id) => {
                let hue = (*id as f64 / disk.files.len() as f64) * 270.0;
                let rgb = hsv_to_rgb(hue, 1.0, 1.0);
                image::Rgb([rgb.0, rgb.1, rgb.2])
            }
        };

        for dx in 0..grid_size {
            for dy in 0..grid_size {
                image.put_pixel(x as u32 + dx as u32, y as u32 + dy as u32, color);
            }
        }
    }

    // Outline left and right index with white
    let left_x = (left_index % blocks_per_row) * grid_size;
    let left_y = (left_index / blocks_per_row) * grid_size;
    let right_x = (right_index % blocks_per_row) * grid_size;
    let right_y = (right_index / blocks_per_row) * grid_size;

    for delta in 0..grid_size {
        image.put_pixel(
            left_x as u32 + delta as u32,
            left_y as u32,
            image::Rgb([255, 255, 255]),
        );
        image.put_pixel(
            left_x as u32 + delta as u32,
            left_y as u32 + grid_size as u32 - 1,
            image::Rgb([255, 255, 255]),
        );
        image.put_pixel(
            right_x as u32 + delta as u32,
            right_y as u32,
            image::Rgb([255, 255, 255]),
        );
        image.put_pixel(
            right_x as u32 + delta as u32,
            right_y as u32 + grid_size as u32 - 1,
            image::Rgb([255, 255, 255]),
        );
    }

    image.save(path).unwrap();
}

fn main() {
    let input = include_str!("../../input/2024/day9.txt");
    let mut disk = Disk::from(input);

    let mut left_index = 0;
    let mut right_index = disk.blocks.len() - 1;

    std::fs::create_dir_all("output").unwrap();
    let mut frame = 0;

    while left_index < right_index {
        frame += 1;
        if frame % 100 == 0 {
            render(
                &disk,
                &format!("output/{:0>8}.png", frame),
                left_index,
                right_index,
            );
        }

        // Right index should always point at a file node
        match disk.blocks[right_index] {
            Block::Empty => {
                right_index -= 1;
                continue;
            }
            Block::File { .. } => {}
        }

        // If left index is empty, swap the right index into it
        // Otherwise, advance
        match disk.blocks[left_index] {
            Block::Empty => {
                disk.blocks.swap(left_index, right_index);
                left_index += 1;
                right_index -= 1;
            }
            Block::File(id) => {
                left_index += disk.files[id].size;
            }
        }
    }

    render(
        &disk,
        &format!("output/{:0>8}.png", frame + 1),
        left_index,
        right_index,
    );

    // Render to mp4
    println!("Rendering video...");
    let cmd = "ffmpeg -y \
        -framerate 24 \
        -pattern_type glob \
        -i 'output/*.png' \
        -c:v libx264 \
        -vf format=yuv420p \
        -movflags +faststart \
        day9-part1.mp4";

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
