use crate::point2d::Point2D;

pub struct Line2D {
    start: Point2D,
    end: Point2D,
}

impl Line2D {
    pub fn new(start: Point2D, end: Point2D) -> Self {
        Line2D { start, end }
    }

    pub fn intersects(&self, other: &Line2D) -> bool {
        let d1 = (self.end.x - self.start.x) * (other.start.y - self.start.y)
            - (self.end.y - self.start.y) * (other.start.x - self.start.x);
        let d2 = (self.end.x - self.start.x) * (other.end.y - self.start.y)
            - (self.end.y - self.start.y) * (other.end.x - self.start.x);
        let d3 = (other.end.x - other.start.x) * (self.start.y - other.start.y)
            - (other.end.y - other.start.y) * (self.start.x - other.start.x);
        let d4 = (other.end.x - other.start.x) * (self.end.y - other.start.y)
            - (other.end.y - other.start.y) * (self.end.x - other.start.x);

        if ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))
        {
            return true;
        }

        false
    }
}

mod test {
    #[test]
    fn test_lines_intersect() {
        let line1 = super::Line2D::new(super::Point2D::new(0, 0), super::Point2D::new(4, 4));
        let line2 = super::Line2D::new(super::Point2D::new(0, 4), super::Point2D::new(4, 0));
        assert!(line1.intersects(&line2));
    }

    #[test]
    fn test_lines_do_not_intersect() {
        let line1 = super::Line2D::new(super::Point2D::new(0, 0), super::Point2D::new(2, 2));
        let line2 = super::Line2D::new(super::Point2D::new(3, 3), super::Point2D::new(4, 4));

        assert!(!line1.intersects(&line2));
    }
}
