use std::collections::VecDeque;

use aoc2024::day22;
use hashbrown::HashMap;
use image::imageops;

const SCALE: usize = 1;
static mut FRAME_COUNT: usize = 0;
static mut NEXT_TO_RENDER: f64 = 0.0;

#[allow(clippy::modulo_one)]
fn render(
    global_scores: &HashMap<(i8, i8, i8, i8), usize>,
    local_scores: &HashMap<(i8, i8, i8, i8), usize>,
    force: bool,
) {
    let path = unsafe {
        let path = format!("output/{:0>8}.png", FRAME_COUNT);

        FRAME_COUNT += 1;

        let count = FRAME_COUNT as f64;
        let count = count.powf(0.45);

        if !force && count < NEXT_TO_RENDER {
            return;
        } else {
            NEXT_TO_RENDER += 1.0;
        }

        println!("Rendering frame: {path}...");
        path
    };

    std::fs::create_dir_all("output").unwrap();

    let size = 20 * 20;
    let mut image = image::ImageBuffer::new(size, size);

    // TODO: Space filling curve

    let mut scores: HashMap<(i8, i8, i8, i8), usize> = HashMap::new();
    scores.extend(global_scores.iter());
    scores.extend(local_scores.iter());

    let max_score = (*scores.values().max().unwrap_or(&0)).max(256);

    for ((a, b, c, d), value) in scores.iter() {
        let a = (a + 9) as u32;
        let b = (b + 9) as u32;
        let c = (c + 9) as u32;
        let d = (d + 9) as u32;

        let x = a * 20 + c;
        let y = b * 20 + d;

        let hue = 180.0 + 180.0 * (*value as f64 / max_score as f64);
        let rgb = hsv::hsv_to_rgb(hue, 1.0, 1.0);

        image.put_pixel(x, y, image::Rgb::<u8>([rgb.0, rgb.1, rgb.2]));
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
    let input = include_str!("../../input/2024/day22.txt");
    let input = aoc2024::day22::parse(input);

    let mut sequence_scores = HashMap::new();

    input.iter().for_each(|&seed| {
        // Find the first time each sequence appears and store the score for that sequence
        let mut local_sequence_scores = HashMap::new();

        let mut rng = day22::SuperSecretPseudoRandomNumberGenerator::new(seed);
        let mut previous_ones = (seed % 10) as i8;
        let mut delta_buffer = VecDeque::new();

        let mut index = 0;
        loop {
            index += 1;
            let value = rng.next().unwrap();
            let ones = (value % 10) as i8;

            delta_buffer.push_back(ones - previous_ones);
            if delta_buffer.len() > 4 {
                delta_buffer.pop_front();
            }

            previous_ones = ones;

            if delta_buffer.len() == 4 {
                let key = (
                    delta_buffer[0],
                    delta_buffer[1],
                    delta_buffer[2],
                    delta_buffer[3],
                );
                if !local_sequence_scores.contains_key(&key) {
                    local_sequence_scores.insert(key, ones as usize);

                    render(&sequence_scores, &local_sequence_scores, false);
                }
            }

            if index > 2_000 {
                break;
            }
        }

        // Add the new local sequence scores to the overall map
        local_sequence_scores.into_iter().for_each(|(key, value)| {
            sequence_scores
                .entry(key)
                .and_modify(|v| *v += value)
                .or_insert(value);
        });
    });

    render(&sequence_scores, &HashMap::new(), true);

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
        day22-part2.mp4";

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
