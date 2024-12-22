use aoc2024::day22::{self};

fn main() {
    std::fs::create_dir_all("output").unwrap();

    let mut rng = day22::SuperSecretPseudoRandomNumberGenerator::new(123);
    let mut image = image::ImageBuffer::new(400, 400);

    for x in 0..400 {
        for y in 0..400 {
            let v = rng.next().unwrap();
            let rgb = hsv::hsv_to_rgb(120.0, 1.0, (v % 10) as f64 / 10.0);

            image.put_pixel(x as u32, y as u32, image::Rgb::<u8>([rgb.0, rgb.1, rgb.2]));
        }
    }

    let path = "day22-part1-ones.png";
    image.save(path).unwrap();
}
