use aoc2024::day22::{self};

fn main() {
    std::fs::create_dir_all("output").unwrap();

    let mut rng = day22::SuperSecretPseudoRandomNumberGenerator::new(123);
    let mut image = image::ImageBuffer::new(400, 400);

    for x in 0..400 {
        for y in 0..400 {
            // 20 bits, so we'll use 6 for each and drop 2
            let v = rng.next().unwrap();

            let r = (v & 0b111111) as f64 / 64.0;
            let g = ((v >> 6) & 0b111111) as f64 / 64.0;
            let b = ((v >> 12) & 0b111111) as f64 / 64.0;

            let r = (r * 255.0) as u8;
            let g = (g * 255.0) as u8;
            let b = (b * 255.0) as u8;

            image.put_pixel(x as u32, y as u32, image::Rgb::<u8>([r, g, b]));
        }
    }

    let path = "day22-part1-rgb.png";
    image.save(path).unwrap();
}
