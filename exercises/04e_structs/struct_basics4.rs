struct Rectangle {
    width: u32,
    height: u32,
}

// TODO: Implement the following functions.

fn area(rect: Rectangle) -> u32 {
    0
}

fn perimeter(rect: Rectangle) -> u32 {
    0
}

// Returns a new rectangle where both sides are multiplied by `factor`.
fn scale(rect: Rectangle, factor: u32) -> Rectangle {
    rect
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        assert_eq!(area(Rectangle { width: 3, height: 4 }), 12);
    }

    #[test]
    fn test_perimeter() {
        assert_eq!(perimeter(Rectangle { width: 3, height: 4 }), 14);
    }

    #[test]
    fn test_scale() {
        let big = scale(Rectangle { width: 3, height: 4 }, 10);
        assert_eq!(big.width, 30);
        assert_eq!(big.height, 40);
    }
}
