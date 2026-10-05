// Classify a triangle by the lengths of its sides `a`, `b` and `c`:
// - "invalid" if the sides can't form a triangle (see the hint),
// - "equilateral" if all three sides are equal,
// - "isosceles" if exactly two sides are equal,
// - "scalene" if all sides are different.
fn triangle_kind(a: u32, b: u32, c: u32) -> &'static str {
    // Early return for invalid triangles.
    if a == 0 || b == 0 || c == 0 || a + b <= c || a + c <= b || b + c <= a {
        return "invalid";
    }

    if a == b && b == c {
        "equilateral"
    } else if a == b || b == c || a == c {
        "isosceles"
    } else {
        "scalene"
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
    fn equilateral() {
        assert_eq!(triangle_kind(3, 3, 3), "equilateral");
    }

    #[test]
    fn isosceles() {
        assert_eq!(triangle_kind(5, 5, 8), "isosceles");
        assert_eq!(triangle_kind(5, 8, 5), "isosceles");
        assert_eq!(triangle_kind(8, 5, 5), "isosceles");
    }

    #[test]
    fn scalene() {
        assert_eq!(triangle_kind(3, 4, 5), "scalene");
    }

    #[test]
    fn zero_side() {
        assert_eq!(triangle_kind(0, 0, 0), "invalid");
        assert_eq!(triangle_kind(0, 4, 5), "invalid");
    }

    #[test]
    fn impossible_triangle() {
        assert_eq!(triangle_kind(1, 2, 10), "invalid");
        assert_eq!(triangle_kind(1, 1, 2), "invalid");
        assert_eq!(triangle_kind(10, 2, 2), "invalid");
    }
}
