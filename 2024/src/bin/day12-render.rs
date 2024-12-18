use aoc2024::{day12::get_regions, Direction, Grid, Point};
use hashbrown::HashMap;
use image::{imageops, ImageBuffer};
use rand::seq::SliceRandom;

fn render_regions(width: usize, height: usize, regions: &[(&char, Vec<Point>)], path: &str) {
    println!("Rendering frame: {path}...");

    let mut image = ImageBuffer::new(width as u32, height as u32);
    let mut regions = regions.to_vec();
    regions.shuffle(&mut rand::thread_rng());

    // Color each region individually
    for (i, (_, points)) in regions.iter().enumerate() {
        let hue = i as f64 / regions.len() as f64;
        let color = hsv::hsv_to_rgb(hue * 360.0, 1.0, 1.0);
        for p in points {
            image.put_pixel(
                p.x as u32,
                p.y as u32,
                image::Rgb([color.0, color.1, color.2]),
            );
        }
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
    let input = include_str!("../../input/2024/day12.txt");
    let grid = Grid::read(input, &|c| c);

    let regions = get_regions(&grid);

    render_regions(grid.width, grid.height, &regions, "day12.png");

    let mut edges = HashMap::new();
    Direction::all().iter().for_each(|&d| {
        edges.insert(d, Grid::new(grid.width, grid.height));
    });

    regions.iter().for_each(|(&c, region)| {
        Direction::all().iter().for_each(|&direction| {
            region.iter().for_each(|p| {
                if grid.get(*p + direction).is_none_or(|&v| v != c) {
                    edges.get_mut(&direction).unwrap().set(*p, true);
                }
            });
        });
    });

    Direction::all().iter().for_each(|&d| {
        let image = Grid::render(&edges[&d], &|&v| {
            if v {
                [255, 255, 255]
            } else {
                [0, 0, 0]
            }
        });

        let image = imageops::resize(
            &image,
            grid.width as u32 * 4,
            grid.height as u32 * 4,
            image::imageops::Nearest,
        );
        image.save(format!("day12-edges-{:?}.png", d)).unwrap();
    });
}
