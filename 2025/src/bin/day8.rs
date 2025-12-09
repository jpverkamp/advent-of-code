use std::collections::BinaryHeap;

use aoc2025::point3d::Point3D;

use itertools::sorted;

aoc::main!(day8);

#[aoc::register]
fn part1(input: &str) -> impl Into<String> {
    let mut lines = input.lines().peekable();

    // The test cases is shorter, so if there's a # on the first line, use that
    let join_count = if lines.peek().unwrap().starts_with('#') {
        lines.next().unwrap()[1..].trim().parse::<usize>().unwrap()
    } else {
        1000
    };

    // Parse points
    let points = lines.map(Point3D::from).collect::<Vec<_>>();
    let points_len = points.len();

    // Initialize each point to be its own region
    let mut regions = (0..points_len).collect::<Vec<_>>();

    // Pre-calculate and sort all distances
    // This seems excessive, but I'm not sure you can avoid it...
    let mut distances = vec![];
    for i in 0..points_len {
        for j in i + 1..points.len() {
            distances.push((points[i].distance_squared(&points[j]), (i, j)));
        }
    }
    distances.sort_by_key(|(d, _)| *d);

    // For the first n distances, join them
    let mut distances = distances.iter();
    for _i in 0..join_count {
        let (_, (i, j)) = distances.next().unwrap();

        let region_to_keep = regions[*i];
        let region_to_replace = regions[*j];
        for region in regions.iter_mut() {
            if *region == region_to_replace {
                *region = region_to_keep;
            }
        }
    }

    // Calculate the size of each region, the answer is the product of the largest 3
    sorted((0..points_len).map(|region_id| regions.iter().filter(|r| **r == region_id).count()))
        .rev()
        .take(3)
        .product::<usize>()
        .to_string()
}

#[aoc::register]
fn part1_heap(input: &str) -> impl Into<String> {
    let mut lines = input.lines().peekable();

    // The test cases is shorter, so if there's a # on the first line, use that
    let join_count = if lines.peek().unwrap().starts_with('#') {
        lines.next().unwrap()[1..].trim().parse::<usize>().unwrap()
    } else {
        1000
    };

    // Parse points
    let points = lines.map(Point3D::from).collect::<Vec<_>>();
    let points_len = points.len();

    // Initialize each point to be its own region
    let mut regions = (0..points_len).collect::<Vec<_>>();

    // Pre-calculate and sort all distances
    // This seems excessive, but I'm not sure you can avoid it...
    let mut distances = BinaryHeap::new();
    for i in 0..points_len {
        for j in i + 1..points.len() {
            distances.push((-points[i].distance_squared(&points[j]), (i, j)));
        }
    }

    // For the first n distances, join them
    for _i in 0..join_count {
        let (_, (i, j)) = distances.pop().unwrap();

        let region_to_keep = regions[i];
        let region_to_replace = regions[j];
        for region in regions.iter_mut() {
            if *region == region_to_replace {
                *region = region_to_keep;
            }
        }
    }

    // Calculate the size of each region, the answer is the product of the largest 3
    sorted((0..points_len).map(|region_id| regions.iter().filter(|r| **r == region_id).count()))
        .rev()
        .take(3)
        .product::<usize>()
        .to_string()
}

#[aoc::register]
fn part2(input: &str) -> impl Into<String> {
    let mut lines = input.lines().peekable();

    // We don't care about iteration count
    if lines.peek().unwrap().starts_with('#') {
        lines.next();
    }

    // Parse points
    let points = lines.map(Point3D::from).collect::<Vec<_>>();
    let points_len = points.len();

    // Initialize each point to be its own region
    let mut regions = (0..points_len).collect::<Vec<_>>();

    // Pre-calculate and sort all distances
    // This seems excessive, but I'm not sure you can avoid it...
    let mut distances = vec![];
    for i in 0..points_len {
        for j in i + 1..points.len() {
            distances.push((points[i].distance_squared(&points[j]), (i, j)));
        }
    }
    distances.sort_by_key(|(d, _)| *d);

    // For the first n distances, join them
    let mut distances = distances.iter();
    loop {
        let (_, (i, j)) = distances.next().unwrap();
        if regions[*i] == regions[*j] {
            continue;
        }

        let region_to_keep = regions[*i];
        let region_to_replace = regions[*j];
        for region in regions.iter_mut() {
            if *region == region_to_replace {
                *region = region_to_keep;
            }
        }

        // If there's only one region, we're done
        // The answer is the product of the two points' x
        if regions.iter().all(|r| *r == region_to_keep) {
            return (points[*i].x * points[*j].x).to_string();
        }
    }
}

aoc::test!(
    text = "\
#10
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
", 
    [part1] => "40",
    [part2] => "25272"
);

aoc::test!(
    file = "input/2025/day8.txt",
    [part1] => "90036",
    [part2] => "6083499488"
);
