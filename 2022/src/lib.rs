use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::time::Instant;

pub fn read_lines(filename: &Path) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn iter_lines(filename: &Path) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
}

type FnPart = fn(&Path) -> String;

pub fn aoc_main(part1: FnPart, part2: FnPart) {
    let part = env::args()
        .nth(1).expect("first arg should be part (1 or 2)")
        .parse::<u32>().expect("part must be a non-negative integer");

    if part != 1 && part != 2 {
        
    }

    let filename = env::args().nth(2).expect("second arg should be input filename");
    let path = Path::new(&filename);
    if !path.exists() {
        panic!("{:?} does not exist", filename);
    }

    let now = Instant::now();
    let result = match part {
        1 => part1(path),
        2 => part2(path),
        _ => panic!("part must be 1 or 2")
    };
    let elapsed = now.elapsed();

    println!("{}", result);
    println!("took {:?}", elapsed);
}

pub fn aoc_test(day: &str, f: FnPart, expected: &str) {
    let mut filename = String::from("data/");
    filename.push_str(&day);
    filename.push_str(".txt");

    let actual = f(Path::new(filename.as_str()));

    assert_eq!(expected, actual);
}