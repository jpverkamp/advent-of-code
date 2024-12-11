use aoc2024::{Grid, Point};
use image::{imageops, ImageBuffer};

fn render(grid: &Grid<char>, active: &Point, scanning: &[Point], visited: &[Point], path: String) {
    println!("Rendering frame: {path}...");

    let mut image = ImageBuffer::new(grid.width as u32 * 6, grid.height as u32 * 6);

    const FONT: &str = "\
X   X M   M   A    SSSS
 X X  MM MM  A A  S    
  X   M M M  AAA   SSS 
 X X  M   M A   A     S
X   X M   M A   A SSSS 
";

    for (p, c) in grid.iter_enumerate() {
        let index = match c {
            'X' => 0,
            'M' => 1,
            'A' => 2,
            'S' => 3,
            _ => panic!("Unknown character: {}", c),
        };

        let color: image::Rgb<u8> = if p == *active {
            image::Rgb([255, 255, 255])
        } else if scanning.contains(&p) {
            image::Rgb([127, 127, 127])
        } else if visited.contains(&p) {
            if index % 2 == 0 {
                image::Rgb([255, 0, 0])
            } else {
                image::Rgb([0, 255, 0])
            }
        } else {
            image::Rgb([64, 64, 64])
        };

        for xd in 0..5 {
            for yd in 0..5 {
                if FONT
                    .chars()
                    .nth(index * 6 + xd + 6 * 4 * yd)
                    .unwrap()
                    .is_ascii_whitespace()
                {
                    continue;
                }

                let px = p.x as u32 * 6 + xd as u32;
                let py = p.y as u32 * 6 + yd as u32;
                image.put_pixel(px, py, color);
            }
        }
    }

    let scale = 10;
    let image = imageops::resize(
        &image,
        grid.width as u32 * 6 * scale,
        grid.height as u32 * 6 * scale,
        image::imageops::Nearest,
    );
    image.save(path).unwrap();
}

fn main() {
    std::fs::create_dir_all("output").unwrap();

    let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let grid = Grid::read(input, &|c| c);

    #[allow(unused_assignments)]
    let mut active_letter = Point::new(0, 0);

    let mut scanning_letters = vec![];
    let mut final_letters = vec![];

    let mut frame = 0;
    let frame_skip = 0;
    let mut rendered_frame = 0;

    // For each starting point
    for x in 0..grid.width {
        for y in 0..grid.height {
            active_letter = (x, y).into();
            scanning_letters.clear();

            // Ignore any that don't start with X
            if grid.get((x, y)) != Some(&'X') {
                continue;
            }

            // For each direction
            for dx in -1..=1 {
                'one_direction: for dy in -1..=1 {
                    scanning_letters.clear();

                    // But have to be moving
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    // Iterate up to the remaining 3 characters in that direction
                    let mut xi = x as isize;
                    let mut yi = y as isize;

                    for target in ['M', 'A', 'S'].iter() {
                        xi += dx;
                        yi += dy;

                        scanning_letters.push((xi, yi).into());

                        if frame_skip == 0 || frame % frame_skip == 0 {
                            render(
                                &grid,
                                &active_letter,
                                &scanning_letters,
                                &final_letters,
                                format!("output/{:08}.png", rendered_frame),
                            );
                            rendered_frame += 1;
                        }
                        frame += 1;

                        if let Some(c) = grid.get((xi, yi)) {
                            if c != target {
                                continue 'one_direction;
                            }
                        } else {
                            continue 'one_direction;
                        }
                    }

                    final_letters.push(active_letter);
                    final_letters.extend(scanning_letters.clone());
                }
            }
        }
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
        day4-part1-example.mp4";

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
