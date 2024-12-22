use aoc2024::day22::{self};

fn main() {
    std::fs::create_dir_all("output").unwrap();

    let mut rng = day22::SuperSecretPseudoRandomNumberGenerator::new(123);

    let mut ones_counts = [0; 10];
    let mut deltas = [0; 100];
    let mut previous_ones = 0;

    for _ in 0..1_000_000 {
        let v = rng.next().unwrap();

        let ones = (v % 10) as usize;
        ones_counts[ones] += 1;
        deltas[previous_ones * 10 + ones] += 1;
        previous_ones = ones;
    }

    #[allow(clippy::needless_range_loop)]
    for i in 0..10 {
        let percent = ones_counts[i] as f64 / 1_000_000.0 * 100.0;
        println!("{:2} {:6} {:5.2}%", i, ones_counts[i], percent);
    }
    println!();

    for i in 0..10 {
        for j in 0..10 {
            print!("{:6}", deltas[i * 10 + j]);
        }
        println!();
    }
    println!();

    for i in 0..10 {
        for j in 0..10 {
            let percent = deltas[i * 10 + j] as f64 / 1_000_000.0 * 100.0;
            print!("{:5.02}", percent);
        }
        println!();
    }
    println!();
}
