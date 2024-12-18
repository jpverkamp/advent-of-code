use aoc2024::day9::{Block, Disk};
use hsv::hsv_to_rgb;
use image::ImageBuffer;

fn render(disk: &Disk, path: &str, left_index: usize) {
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
    }

    image.save(path).unwrap();
}

fn main() {
    let input = include_str!("../../input/2024/day9.txt");
    let mut disk = Disk::from(input);

    std::fs::create_dir_all("output").unwrap();
    render(&disk, &format!("output/{:0>8}.png", 0), 0);

    let mut frame = 0;
    let mut rendered_frame = 0;

    let mut leftmost_empty = 0;

    // We're going to try to move each file from right to left exactly once
    'each_file: for moving_id in (0..disk.files.len()).rev() {
        println!("Moving file {}...", moving_id);

        // Advance the leftmost empty block
        while leftmost_empty < disk.blocks.len() && disk.blocks[leftmost_empty] != Block::Empty {
            leftmost_empty += 1;
        }

        let mut left_index = leftmost_empty;
        let mut empty_starts_at = None;

        while left_index < disk.files[moving_id].start {
            frame += 1;
            if frame % 100000 == 0 {
                rendered_frame += 1;
                render(
                    &disk,
                    &format!("output/{:0>8}.png", rendered_frame),
                    left_index,
                );
            }

            match disk.blocks[left_index] {
                Block::File(_) => {
                    left_index += 1;
                    empty_starts_at = None;
                }
                Block::Empty => {
                    if empty_starts_at.is_none() {
                        empty_starts_at = Some(left_index);
                    }

                    // Found a large enough space
                    if empty_starts_at.is_some_and(|empty_starts_at| {
                        left_index - empty_starts_at + 1 >= disk.files[moving_id].size
                    }) {
                        for i in 0..disk.files[moving_id].size {
                            disk.blocks.swap(
                                disk.files[moving_id].start + i,
                                empty_starts_at.unwrap() + i,
                            );
                        }
                        disk.files[moving_id].start = empty_starts_at.unwrap();
                        continue 'each_file;
                    } else {
                        left_index += 1;
                    }
                }
            }
        }
    }

    render(&disk, &format!("output/{:0>8}.png", frame + 1), 0);

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
        day9-part2.mp4";

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
