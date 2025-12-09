use aoc2025::line2d::Line2D;
use aoc2025::point2d::Point2D;
use aoc2025::polygon::Polygon;

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
    let mut best_rectangle: Vec<Point2D> = vec![
        points[0],
        points[0],
        points[0],
        points[0],
    ];

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
                    svg(&[(points.as_slice(), "lightgray"),
                        (&best_rectangle, "lightgreen"),
                        (&vertices, "red")])
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
                    svg(&[(points.as_slice(), "lightgray"),
                        (&best_rectangle, "lightgreen"),
                        (&vertices, "red")])
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
                    svg(&[(points.as_slice(), "lightgray"),
                        (&best_rectangle, "lightgreen"),
                        (&vertices, "green")]),
                    force = true
                );

                max_area = area;
                best_rectangle = vertices.clone();
            } else {
                aoc::render_svg_frame!(
                    800,
                    800,
                    svg(&[(points.as_slice(), "lightgray"),
                        (&best_rectangle, "lightgreen"),
                        (&vertices, "yellow")]),
                    force = true
                );
            }
        }
    }
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
