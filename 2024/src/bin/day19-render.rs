use aoc2024::day19::Puzzle;
use hashbrown::HashMap;
use image::imageops;

const SCALE: usize = 8;
const FRAME_SKIP: usize = 1;
static mut FRAME_COUNT: usize = 0;
static mut MAX_CHARS: usize = 0;
const MAX_FRAMES: usize = 300;

const FONT: &str = "\
RRRR   GGG  BBBB  U   U W   W 
R   R G     B   B U   U W   W 
RRRR  G  GG BBBB  U   U W W W 
R  R  G   G B   B U   U WW WW 
R   R  GGG  BBBB   UUU  W   W 
";

#[allow(clippy::modulo_one)]
fn render(input: &str, highlights: &[(usize, usize)], force: bool) {
    let path = unsafe {
        FRAME_COUNT += 1;
        &format!("output/{:0>8}.png", FRAME_COUNT / FRAME_SKIP)
    };
    if !force && unsafe { FRAME_COUNT } % FRAME_SKIP != 0 {
        return;
    }

    println!("Rendering frame: {path}...");

    let size_in_chars = (unsafe { MAX_CHARS } as f64).sqrt().ceil() as usize;

    let mut image = image::ImageBuffer::new(size_in_chars as u32 * 6, size_in_chars as u32 * 6);

    for (i, c) in input.chars().enumerate() {
        let which_char = match c {
            'R' | 'r' => 0,
            'G' | 'g' => 1,
            'B' | 'b' => 2,
            'U' | 'u' => 3,
            'W' | 'w' => 4,
            _ => panic!("Unknown character: {}", c),
        };

        let mut color: image::Rgb<u8> = image::Rgb([64, 64, 64]);

        for (highlight_index, (start, len)) in highlights.iter().enumerate() {
            if i >= *start && i < *start + *len {
                let hue = 693.7 * (highlight_index as f64 / 10.0_f64) % 360.0;
                let rgb = hsv::hsv_to_rgb(hue, 1.0, 1.0);
                color = image::Rgb([rgb.0, rgb.1, rgb.2]);
            }
        }

        if !highlights.is_empty()
            && i == highlights.last().unwrap().0 + highlights.last().unwrap().1
        {
            color = image::Rgb([255, 255, 255]);
        }

        for xd in 0..5 {
            for yd in 0..5 {
                if FONT
                    .lines()
                    .nth(yd)
                    .unwrap()
                    .chars()
                    .nth(which_char * 6 + xd)
                    .unwrap()
                    .is_ascii_whitespace()
                {
                    continue;
                }

                let px = (i % size_in_chars) as u32 * 6 + xd as u32;
                let py = (i / size_in_chars) as u32 * 6 + yd as u32;
                image.put_pixel(px, py, color);
            }
        }
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
    let input = include_str!("../../input/2024/day19.txt");
    let puzzle: Puzzle = input.into();

    let mut segments = vec![];

    unsafe {
        MAX_CHARS = puzzle.targets.iter().map(|s| s.len()).max().unwrap();
        println!("MAX_CHARS: {}", MAX_CHARS);
    }

    fn recur<'input>(
        cache: &mut HashMap<&'input str, usize>,
        towels: &[&str],
        target: &'input str,

        segments: &mut Vec<(usize, usize)>,
        original_target: &'input str,
        index: usize,
        use_cache: bool,
    ) -> usize {
        render(original_target, segments, false);
        unsafe {
            if FRAME_COUNT >= MAX_FRAMES {
                return 0;
            }
        }

        // Base case: empty tests are makeable exactly 1 way
        if target.is_empty() {
            return 1;
        }

        // If we've already calculated this target, return the cached value
        // Memoization yo
        if let Some(&count) = cache.get(target) {
            return count;
        }

        // Try each towel and recur on the first occurrence of that towel in the target
        let mut count = 0;

        for towel in towels {
            if let Some(rest) = target.strip_prefix(towel) {
                segments.push((index, towel.len()));

                count += recur(
                    cache,
                    towels,
                    rest,
                    segments,
                    original_target,
                    index + towel.len(),
                    use_cache,
                );

                segments.pop();
            }
        }

        if use_cache {
            cache.insert(target, count);
        }
        count
    }

    let mut cache = HashMap::new();

    for use_cache in [false, true] {
        unsafe {
            std::fs::create_dir_all("output").unwrap();
            FRAME_COUNT = 0;
        }

        for target in &puzzle.targets {
            println!("Target: {}", target);
            recur(
                &mut cache,
                &puzzle.towels,
                target,
                &mut segments,
                target,
                0,
                use_cache,
            );

            unsafe {
                if FRAME_COUNT >= MAX_FRAMES {
                    break;
                }
            }
        }

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
            day19-{}.mp4",
            if use_cache { "memo" } else { "no_memo" }
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
}
