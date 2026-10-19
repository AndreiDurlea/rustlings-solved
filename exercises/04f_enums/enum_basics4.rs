use std::f64::consts::PI;

// TODO: Add the variant `Triangle` that has two fields `base` and `height`
// (both `f64`), like the `Rectangle` variant.
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
}

fn area(shape: Shape) -> f64 {
    // TODO: Return the area of the shape.
    match shape {
        Shape::Circle(radius) => 0.0,
        _ => 0.0,
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
