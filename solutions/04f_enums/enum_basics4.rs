use std::f64::consts::PI;

enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Triangle { base, height } => base * height / 2.0,
    }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle() {
        assert_eq!(area(Shape::Circle(1.0)), PI);
        assert_eq!(area(Shape::Circle(2.0)), 4.0 * PI);
    }

    #[test]
    fn rectangle() {
        assert_eq!(
            area(Shape::Rectangle {
                width: 3.0,
                height: 4.5,
            }),
            13.5,
        );
    }

    #[test]
    fn triangle() {
        assert_eq!(
            area(Shape::Triangle {
                base: 6.0,
                height: 3.0,
            }),
            9.0,
        );
    }
}
