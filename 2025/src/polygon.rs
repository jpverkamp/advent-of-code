use crate::point2d::Point2D;

#[derive(Debug, Clone)]
pub struct Polygon {
    vertices: Vec<Point2D>,
}

impl Polygon {
    pub fn new(vertices: Vec<Point2D>) -> Self {
        Polygon { vertices }
    }

    // Check if a point is inside the polygon using the ray-casting algorithm
    // Source: https://www.xjavascript.com/blog/check-if-polygon-is-inside-a-polygon/
    pub fn contains(&self, point: &Point2D) -> bool {
        let mut inside = false;
        let n = self.vertices.len();

        for i in 0..n {
            let j = (i + n - 1) % n;

            let pi = &self.vertices[i];
            let pj = &self.vertices[j];

            // TODO: On edge check

            if (pi.y > point.y) != (pj.y > point.y) {
                let x_intersect = (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x;
                if point.x < x_intersect {
                    inside = !inside;
                }
            }
        }

        inside
    }
}

mod test {
    #[test]
    fn test_polygon_contains() {
        use crate::point2d::Point2D;

        let polygon = super::Polygon::new(vec![
            Point2D::new(0, 0),
            Point2D::new(5, 0),
            Point2D::new(5, 5),
            Point2D::new(0, 5),
        ]);

        let inside_point = Point2D::new(3, 3);
        let outside_point = Point2D::new(6, 3);
        let edge_point = Point2D::new(5, 3);

        assert!(polygon.contains(&inside_point));
        assert!(!polygon.contains(&outside_point));
        assert!(!polygon.contains(&edge_point)); // Edge case
    }
}
