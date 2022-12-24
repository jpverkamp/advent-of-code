use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Add, Index, IndexMut, Sub};
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

    buf.lines().map(|l| l.expect("Could not parse line"))
}

type FnPart = fn(&Path) -> String;

pub fn aoc_main(part1: FnPart, part2: FnPart) {
    let part = env::args()
        .nth(1)
        .expect("first arg should be part (1 or 2)")
        .parse::<u32>()
        .expect("part must be a non-negative integer");

    if part != 1 && part != 2 {}

    let filename = env::args()
        .nth(2)
        .expect("second arg should be input filename");
    let path = Path::new(&filename);
    if !path.exists() {
        panic!("{:?} does not exist", filename);
    }

    let now = Instant::now();
    let result = match part {
        1 => part1(path),
        2 => part2(path),
        _ => panic!("part must be 1 or 2"),
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

/* ----- A 2D matrix of any kind of value ----- */
#[derive(Clone, Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T>
where
    T: Clone + Default + core::fmt::Debug,
{
    pub fn new(width: usize, height: usize) -> Self {
        Matrix::<T> {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn index(&self, x: usize, y: usize) -> &T {
        &self.data[x * self.width + y]
    }

    pub fn index_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[x * self.width + y]
    }

    pub fn at(&self, p: &Point) -> &T {
        &self[[p.x as usize, p.y as usize]]
    }
}

impl<T> Index<[usize; 2]> for Matrix<T>
where
    T: core::fmt::Debug,
{
    type Output = T;

    fn index(&self, [x, y]: [usize; 2]) -> &Self::Output {
        &self.data[y * self.width + x]
    }
}

impl<T> IndexMut<[usize; 2]> for Matrix<T>
where
    T: core::fmt::Debug,
{
    fn index_mut(&mut self, [x, y]: [usize; 2]) -> &mut Self::Output {
        &mut self.data[y * self.width + x]
    }
}

/* ----- Represent a generic point over signed values ----- */
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    pub fn manhattan_distance(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn adjacent_to(&self, other: &Point) -> bool {
        self.manhattan_distance(other) == 1
            || ((self.x - other.x).abs() == 1 && (self.y - other.y).abs() == 1)
    }
}

impl Point {
    pub const ORIGIN: Point = Point { x: 0, y: 0 };
    pub const UP: Point = Point { x: 0, y: -1 };
    pub const DOWN: Point = Point { x: 0, y: 1 };
    pub const LEFT: Point = Point { x: -1, y: 0 };
    pub const RIGHT: Point = Point { x: 1, y: 0 };
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

/* ----- A 3D version of the point ----- */
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Point3D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Self) -> Self::Output {
        Point3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Point3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Point3D {
    pub const UNITS: [Point3D; 6] = [
        Point3D { x: -1, y: 0, z: 0 },
        Point3D { x: 1, y: 0, z: 0 },
        Point3D { x: 0, y: -1, z: 0 },
        Point3D { x: 0, y: 1, z: 0 },
        Point3D { x: 0, y: 0, z: -1 },
        Point3D { x: 0, y: 0, z: 1 },
    ];

    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Point3D { x, y, z }
    }

    pub fn adjacent_to(self, other: Point3D) -> bool {
        let delta = self - other;

        delta.x == 0 && delta.y == 0 && delta.z.abs() == 1
            || delta.x == 0 && delta.y.abs() == 1 && delta.z == 0
            || delta.x.abs() == 1 && delta.y == 0 && delta.z == 0
    }
}

/* ---- Render a series of PNGs into an mp4 ----- */
pub fn make_mp4(fps: usize, name: String) {
    println!("Rendering mp4");

    use std::process::Command;

    let commands = vec![
        format!("ffmpeg -y -framerate {fps} -i %08d.png -vf scale=iw*4:ih*4:flags=neighbor -c:v libx264 -r 30 {name}.raw.mp4"),
        format!("find . -name '*.png' | xargs rm"),
        format!("ffmpeg -y -i {name}.raw.mp4 -c:v libx264 -preset slow -crf 20 -vf format=yuv420p -movflags +faststart {name}.mp4"),
        format!("rm {name}.raw.mp4"),
    ];

    for cmd in commands.into_iter() {
        println!("$ {}", cmd);
        let mut child = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .spawn()
            .expect("command failed");
        child.wait().expect("process didn't finish");
    }
}