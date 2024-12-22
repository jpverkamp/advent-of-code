use aoc2024::day22::{self};

fn main() {
    std::fs::create_dir_all("output").unwrap();

    let mut rng = day22::SuperSecretPseudoRandomNumberGenerator::new(123);
    let mut image = image::ImageBuffer::new(400, 400);

    for x in 0..400 {
        for y in 0..20 {
            let v = rng.next().unwrap();

            for (yd, bit) in (0..20).enumerate() {
                let color = if v & (1 << bit) != 0 {
                    image::Rgb([0_u8, 0, 0])
                } else {
                    image::Rgb([255, 255, 255])
                };
                image.put_pixel(x as u32, (y * 20 + yd) as u32, color);
            }
        }
    }

    let path = "day22-part1-bits.png";
    image.save(path).unwrap();
}
