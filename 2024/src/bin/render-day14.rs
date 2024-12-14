use aoc2024::day14::{self, Robot};
use image::{imageops, ImageBuffer};

fn render(width: usize, height: usize, robots: &[Robot], path: &str) {
    println!("Rendering frame: {path}...");

    let mut image = ImageBuffer::new(width as u32, height as u32);

    // Render any point that is a robot
    for robot in robots.iter() {
        image.put_pixel(
            robot.position.x as u32,
            robot.position.y as u32,
            image::Rgb([255_u8, 255_u8, 255_u8]),
        );
    }

    let image = imageops::resize(
        &image,
        width as u32 * 4,
        height as u32 * 4,
        image::imageops::Nearest,
    );
    image.save(path).unwrap();
}

fn main() {
    let input = include_str!("../../input/2024/day14.txt");
    let (width, height, mut robots) = day14::parse(input);

    let max_frames = std::env::args()
        .nth(1)
        .unwrap_or("100".to_string())
        .parse::<usize>()
        .unwrap();

    std::fs::create_dir_all("output").unwrap();
    let mut frame = 0;

    for _i in 0..max_frames {
        render(width, height, &robots, &format!("output/{:0>8}.png", frame));
        frame += 1;

        for robot in robots.iter_mut() {
            robot.position += robot.velocity;
            robot.position.x = robot.position.x.rem_euclid(width as i32);
            robot.position.y = robot.position.y.rem_euclid(height as i32);
        }
    }

    render(width, height, &robots, &format!("output/{:0>8}.png", frame));

    // Render to mp4
    println!("Rendering video...");
    let cmd = format!(
        "ffmpeg -y \
        -framerate 24 \
        -pattern_type glob \
        -i 'output/*.png' \
        -c:v libx264 \
        -crf 24 \
        -vf format=yuv420p \
        -movflags +faststart \
        day14-part2-{max_frames}.mp4"
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
