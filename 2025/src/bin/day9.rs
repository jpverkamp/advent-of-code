use aoc2025::grid::Grid;
use aoc2025::line2d::Line2D;
use aoc2025::point2d::Point2D;
use aoc2025::polygon::Polygon;
use itertools::Itertools;

aoc::main!(day9);

#[aoc::register]
fn part1(input: &str) -> impl Into<String> {
    let points = input.lines().map(Point2D::from).collect::<Vec<_>>();

    let mut max_area = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let xd = (points[i].x - points[j].x).abs() + 1;
            let yd = (points[i].y - points[j].y).abs() + 1;
            let area = xd * yd;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area.to_string()
}

fn svg(layers: &[(&[Point2D], &str)]) -> String {
    let bounds = layers.iter().flat_map(|(points, _)| points.iter()).fold(
        (
            layers[0].0[0].x,
            layers[0].0[0].y,
            layers[0].0[0].x,
            layers[0].0[0].y,
        ),
        |(min_x, min_y, max_x, max_y), p| {
            (
                min_x.min(p.x),
                min_y.min(p.y),
                max_x.max(p.x),
                max_y.max(p.y),
            )
        },
    );

    let width = (bounds.2 - bounds.0).abs() + 10;
    let height = (bounds.3 - bounds.1).abs() + 10;

    let mut svg_data = String::new();
    svg_data.push_str(&format!(
        "<svg width=\"800\" height=\"800\" viewBox=\"{} {} {} {}\" xmlns=\"http://www.w3.org/2000/svg\">\n",
        bounds.0 - 5,
        bounds.1 - 5,
        width,
        height
    ));

    // Render each layer
    for (points, overlay_color) in layers {
        // Background
        svg_data.push_str("<polygon points=\"");

        for vertex in *points {
            svg_data.push_str(&format!("{},{} ", vertex.x, vertex.y));
        }
        svg_data.push_str(&format!(
            "\" style=\"fill:{overlay_color};stroke:black;stroke-width:1\" />\n"
        ));
    }

    svg_data.push_str("</svg>\n");

    svg_data
}

#[aoc::register_render]
fn part1_svg(input: &str) {
    let points = input.lines().map(Point2D::from).collect::<Vec<_>>();

    aoc::render_svg!(polygon, svg(&[(points.as_slice(), "lightgray")]));
}

#[aoc::register]
fn part2(input: &str) -> impl Into<String> {
    let points = input.lines().map(Point2D::from).collect::<Vec<_>>();
    let polygon = Polygon::new(points.clone());

    let mut max_area = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            // All 4 vertices must be within the polygon
            // Because the polygons are on a grid of points, this means we can ignore the edge checks
            // This will skip rectangles with width/height = 1, but those won't be max area (we assume)
            let vertices = [
                Point2D::new(
                    points[i].x.min(points[j].x) + 1,
                    points[i].y.min(points[j].y) + 1,
                ),
                Point2D::new(
                    points[i].x.min(points[j].x) + 1,
                    points[i].y.max(points[j].y) - 1,
                ),
                Point2D::new(
                    points[i].x.max(points[j].x) - 1,
                    points[i].y.min(points[j].y) + 1,
                ),
                Point2D::new(
                    points[i].x.max(points[j].x) - 1,
                    points[i].y.max(points[j].y) - 1,
                ),
            ];
            if !vertices.iter().all(|v| polygon.contains(v)) {
                continue;
            }

            // No edge of the rectangle can intersect with any edge of the polygon
            let rectangle_edges = vec![
                Line2D::new(vertices[0], vertices[1]),
                Line2D::new(vertices[1], vertices[3]),
                Line2D::new(vertices[3], vertices[2]),
                Line2D::new(vertices[2], vertices[0]),
            ];

            let mut intersects = false;
            'outer: for rect_edge in &rectangle_edges {
                for i in 0..points.len() {
                    let poly_edge = Line2D::new(points[i], points[(i + 1) % points.len()]);
                    if rect_edge.intersects(&poly_edge) {
                        intersects = true;
                        break 'outer;
                    }
                }
            }
            if intersects {
                continue;
            }

            let xd = (points[i].x - points[j].x).abs() + 1;
            let yd = (points[i].y - points[j].y).abs() + 1;
            let area = xd * yd;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area.to_string()
}

#[aoc::register]
fn part2_area_first(input: &str) -> impl Into<String> {
    let points = input.lines().map(Point2D::from).collect::<Vec<_>>();
    let polygon = Polygon::new(points.clone());

    let mut max_area = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let xd = (points[i].x - points[j].x).abs() + 1;
            let yd = (points[i].y - points[j].y).abs() + 1;
            let area = xd * yd;

            if area <= max_area {
                continue;
            }

            // All 4 vertices must be within the polygon
            // Because the polygons are on a grid of points, this means we can ignore the edge checks
            // This will skip rectangles with width/height = 1, but those won't be max area (we assume)
            let vertices = [
                Point2D::new(
                    points[i].x.min(points[j].x) + 1,
                    points[i].y.min(points[j].y) + 1,
                ),
                Point2D::new(
                    points[i].x.min(points[j].x) + 1,
                    points[i].y.max(points[j].y) - 1,
                ),
                Point2D::new(
                    points[i].x.max(points[j].x) - 1,
                    points[i].y.min(points[j].y) + 1,
                ),
                Point2D::new(
                    points[i].x.max(points[j].x) - 1,
                    points[i].y.max(points[j].y) - 1,
                ),
            ];
            if !vertices.iter().all(|v| polygon.contains(v)) {
                continue;
            }

            // No edge of the rectangle can intersect with any edge of the polygon
            let rectangle_edges = vec![
                Line2D::new(vertices[0], vertices[1]),
                Line2D::new(vertices[1], vertices[3]),
                Line2D::new(vertices[3], vertices[2]),
                Line2D::new(vertices[2], vertices[0]),
            ];

            let mut intersects = false;
            'outer: for rect_edge in &rectangle_edges {
                for i in 0..points.len() {
                    let poly_edge = Line2D::new(points[i], points[(i + 1) % points.len()]);
                    if rect_edge.intersects(&poly_edge) {
                        intersects = true;
                        break 'outer;
                    }
                }
            }
            if intersects {
                continue;
            }

            max_area = area;
        }
    }

    max_area.to_string()
}

#[aoc::register_render(fps = 60, sample = 1000)]
fn part2_svg(input: &str) {
    let points = input.lines().map(Point2D::from).collect::<Vec<_>>();
    let polygon = Polygon::new(points.clone());

    let mut max_area = 0;
    let mut best_rectangle: Vec<Point2D> = vec![points[0], points[0], points[0], points[0]];

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            // All 4 vertices must be within the polygon
            // Because the polygons are on a grid of points, this means we can ignore the edge checks
            // This will skip rectangles with width/height = 1, but those won't be max area (we assume)
            let vertices = vec![
                Point2D::new(
                    points[i].x.min(points[j].x) + 1,
                    points[i].y.min(points[j].y) + 1,
                ),
                Point2D::new(
                    points[i].x.min(points[j].x) + 1,
                    points[i].y.max(points[j].y) - 1,
                ),
                Point2D::new(
                    points[i].x.max(points[j].x) - 1,
                    points[i].y.max(points[j].y) - 1,
                ),
                Point2D::new(
                    points[i].x.max(points[j].x) - 1,
                    points[i].y.min(points[j].y) + 1,
                ),
            ];
            if !vertices.iter().all(|v| polygon.contains(v)) {
                aoc::render_svg_frame!(
                    800,
                    800,
                    svg(&[
                        (points.as_slice(), "lightgray"),
                        (&best_rectangle, "lightgreen"),
                        (&vertices, "red")
                    ])
                );
                continue;
            }

            // No edge of the rectangle can intersect with any edge of the polygon
            let rectangle_edges = vec![
                Line2D::new(vertices[0], vertices[1]),
                Line2D::new(vertices[1], vertices[3]),
                Line2D::new(vertices[3], vertices[2]),
                Line2D::new(vertices[2], vertices[0]),
            ];

            let mut intersects = false;
            'outer: for rect_edge in &rectangle_edges {
                for i in 0..points.len() {
                    let poly_edge = Line2D::new(points[i], points[(i + 1) % points.len()]);
                    if rect_edge.intersects(&poly_edge) {
                        intersects = true;
                        break 'outer;
                    }
                }
            }
            if intersects {
                aoc::render_svg_frame!(
                    800,
                    800,
                    svg(&[
                        (points.as_slice(), "lightgray"),
                        (&best_rectangle, "lightgreen"),
                        (&vertices, "red")
                    ])
                );
                continue;
            }

            let xd = (points[i].x - points[j].x).abs() + 1;
            let yd = (points[i].y - points[j].y).abs() + 1;
            let area = xd * yd;

            if area > max_area {
                aoc::render_svg_frame!(
                    800,
                    800,
                    svg(&[
                        (points.as_slice(), "lightgray"),
                        (&best_rectangle, "lightgreen"),
                        (&vertices, "green")
                    ]),
                    force = true
                );

                max_area = area;
                best_rectangle = vertices.clone();
            } else {
                aoc::render_svg_frame!(
                    800,
                    800,
                    svg(&[
                        (points.as_slice(), "lightgray"),
                        (&best_rectangle, "lightgreen"),
                        (&vertices, "yellow")
                    ]),
                    force = true
                );
            }
        }
    }
}

#[aoc::register_render]
fn part2_compress_render(input: &str) {
    let points = input.lines().map(Point2D::from).collect::<Vec<_>>();

    let x_points = points
        .iter()
        .map(|p| p.x)
        .sorted()
        .unique()
        .collect::<Vec<_>>();
    let y_points = points
        .iter()
        .map(|p| p.y)
        .sorted()
        .unique()
        .collect::<Vec<_>>();

    let compressed_points = points
        .iter()
        .map(|p| {
            Point2D::new(
                x_points.iter().position(|&x| x == p.x).unwrap() as isize,
                y_points.iter().position(|&y| y == p.y).unwrap() as isize,
            )
        })
        .collect::<Vec<_>>();

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Cell {
        Unknown,
        Outside,
        Inside,
        Wall,
    }

    impl Cell {
        fn color(&self) -> (u8, u8, u8) {
            match self {
                Cell::Unknown => (0, 0, 0),
                Cell::Outside => (255, 255, 255),
                Cell::Inside => (200, 200, 200),
                Cell::Wall => (0, 127, 0),
            }
        }
    }

    let mut grid = Grid::new(x_points.len() + 2, y_points.len() + 2, Cell::Unknown);

    // Each point is a corner as are all points bewtween them
    // This does assume that either x or y does not change
    for (p1, p2) in compressed_points.iter().tuple_combinations() {
        if p1.x == p2.x {
            let x = p1.x + 1;
            let y_start = p1.y.min(p2.y) + 1;
            let y_end = p1.y.max(p2.y) + 1;
            for y in y_start..=y_end {
                grid.set(x, y, Cell::Wall);
            }
        } else if p1.y == p2.y {
            let y = p1.y + 1;
            let x_start = p1.x.min(p2.x) + 1;
            let x_end = p1.x.max(p2.x) + 1;
            for x in x_start..=x_end {
                grid.set(x, y, Cell::Wall);
            }
        }
    }

    // Flood fill from (0,0) to mark outside
    let mut stack = vec![(0isize, 0isize)];
    while let Some((x, y)) = stack.pop() {
        if grid.get(x, y) != Some(Cell::Unknown) {
            continue;
        }
        grid.set(x, y, Cell::Outside);

        for nx in (x - 1)..=(x + 1) {
            for ny in (y - 1)..=(y + 1) {
                if (nx == x || ny == y) && grid.get(nx, ny) == Some(Cell::Unknown) {
                    stack.push((nx, ny));
                }
            }
        }
    }

    // Now any points that are still Unknown are Inside
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if grid.get(x, y) == Some(Cell::Unknown) {
                grid.set(x, y, Cell::Inside);
            }
        }
    }

    aoc::render_image!(inside, grid.width(), grid.height(), |x, y| {
        grid.get(x, y).unwrap().color()
    });
}

#[aoc::register]
fn part2_compress(input: &str) -> impl Into<String> {
    let points = input.lines().map(Point2D::from).collect::<Vec<_>>();

    let x_points = points
        .iter()
        .map(|p| p.x)
        .sorted()
        .unique()
        .collect::<Vec<_>>();
    let y_points = points
        .iter()
        .map(|p| p.y)
        .sorted()
        .unique()
        .collect::<Vec<_>>();

    let compressed_points = points
        .iter()
        .map(|p| {
            Point2D::new(
                x_points.iter().position(|&x| x == p.x).unwrap() as isize,
                y_points.iter().position(|&y| y == p.y).unwrap() as isize,
            )
        })
        .collect::<Vec<_>>();

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Cell {
        Unknown,
        Outside,
        Inside,
        Wall,
    }

    let mut grid = Grid::new(x_points.len() + 2, y_points.len() + 2, Cell::Unknown);

    // Each point is a corner as are all points bewtween them
    // This does assume that either x or y does not change
    for (p1, p2) in compressed_points.iter().tuple_combinations() {
        if p1.x == p2.x {
            let x = p1.x + 1;
            let y_start = p1.y.min(p2.y) + 1;
            let y_end = p1.y.max(p2.y) + 1;
            for y in y_start..=y_end {
                grid.set(x, y, Cell::Wall);
            }
        } else if p1.y == p2.y {
            let y = p1.y + 1;
            let x_start = p1.x.min(p2.x) + 1;
            let x_end = p1.x.max(p2.x) + 1;
            for x in x_start..=x_end {
                grid.set(x, y, Cell::Wall);
            }
        }
    }

    // Flood fill from (0,0) to mark outside
    let mut stack = vec![(0isize, 0isize)];
    while let Some((x, y)) = stack.pop() {
        if grid.get(x, y) != Some(Cell::Unknown) {
            continue;
        }
        grid.set(x, y, Cell::Outside);

        for nx in (x - 1)..=(x + 1) {
            for ny in (y - 1)..=(y + 1) {
                if (nx == x || ny == y) && grid.get(nx, ny) == Some(Cell::Unknown) {
                    stack.push((nx, ny));
                }
            }
        }
    }

    // Now any points that are still Unknown are Inside
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if grid.get(x, y) == Some(Cell::Unknown) {
                grid.set(x, y, Cell::Inside);
            }
        }
    }

    // For each pair of points, verify that all points in the rectangle are Inside or Wall
    // Then calculate their area (uncompressed) and track the max
    let mut max_area = 0;
    for i in 0..compressed_points.len() {
        for j in i + 1..compressed_points.len() {
            let xd = (points[i].x - points[j].x).abs() + 1;
            let yd = (points[i].y - points[j].y).abs() + 1;
            let area = xd * yd;

            if area <= max_area {
                continue;
            }

            let x_start = compressed_points[i].x.min(compressed_points[j].x) + 1;
            let x_end = compressed_points[i].x.max(compressed_points[j].x) + 1;
            let y_start = compressed_points[i].y.min(compressed_points[j].y) + 1;
            let y_end = compressed_points[i].y.max(compressed_points[j].y) + 1;

            let mut valid = true;
            'invalidate: for x in x_start..=x_end {
                for y in y_start..=y_end {
                    match grid.get(x, y) {
                        Some(Cell::Inside) | Some(Cell::Wall) => {}
                        _ => {
                            valid = false;
                            break 'invalidate;
                        }
                    }
                }
            }
            if !valid {
                continue;
            }

            max_area = area;
        }
    }

    max_area.to_string()
}

aoc::test!(
    text = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
", 
    [part1] => "50",
    [part2, part2_area_first] => "24"
);

aoc::test!(
    file = "input/2025/day9.txt",
    [part1] => "4749929916",
    [part2, part2_area_first] => "1572047142"
);
