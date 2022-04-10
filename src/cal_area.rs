struct Point(f64, f64);

#[allow(dead_code)]
struct Circle {
    centre: Point,
    radius: f64,
}

struct Triangle(Point, Point, Point);

/// List Point in anticlockwise
struct Square(Point, Point, Point, Point);

pub trait Area {
    fn area(&self) -> f64;
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        let Triangle(point1, point2, point3) = self;
        (((point1.0 * point2.1 - point2.0 * point1.1) + (point2.0 * point3.1 - point3.0 * point2.1) + (point3.0 * point1.1 - point1.0 * point3.1)) / 2.0).abs()
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        let Square(point1, _point2, point3, _point4) = self;
        (((point3.0 - point1.0).powf(2.0) + (point3.1 - point1.1).powf(2.0)) / 2.0).abs()
    }
}

#[allow(dead_code)]
pub fn print_area<T: Area>(item: &T) -> String {
    format!("Area size: {}", item.area())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { 
            centre: Point(0.0, 0.0), 
            radius: 1.0,
        };
        assert_eq!(print_area(&circle), "Area size: 3.14");
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle(
            Point(0.0, 0.0), 
            Point(0.0, 3.0), 
            Point(4.0, 0.0),
        );
        assert!(triangle.area() - 6.0 < f64::EPSILON);
    }

    #[test]
    fn test_square_area() {
        let square = Square(
            Point(0.0, 0.0), 
            Point(1.0, 0.0), 
            Point(1.0, 1.0),
            Point(0.0, 1.0),
        );
        assert!(square.area() - 1.0 < f64::EPSILON);
    }
}