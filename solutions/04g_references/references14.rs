enum Shape {
    Square(f64),
    Rectangle(f64, f64),
}

fn longest_side(shape: &Shape) -> f64 {
    // Matching on `&Shape` binds `side`, `a` and `b` as `&f64`.
    match shape {
        Shape::Square(side) => *side,
        Shape::Rectangle(a, b) => {
            if a > b {
                *a
            } else {
                *b
            }
        }
    }
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Square(side) => side * side,
        Shape::Rectangle(a, b) => a * b,
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
    fn square() {
        let s = Shape::Square(3.0);
        assert_eq!(longest_side(&s), 3.0);
        assert_eq!(area(&s), 9.0);
    }

    #[test]
    fn rectangle() {
        let r = Shape::Rectangle(2.0, 5.0);
        assert_eq!(longest_side(&r), 5.0);
        assert_eq!(area(&r), 10.0);
        assert_eq!(longest_side(&Shape::Rectangle(6.0, 1.5)), 6.0);
    }
}
