const TOLERANCE: f64 = 1e-9;

// Returns `true` if `a` and `b` are (almost) equal.
fn is_close(a: f64, b: f64) -> bool {
    (a - b).abs() < TOLERANCE
}

fn main() {
    println!("0.1 + 0.2 = {}", 0.1 + 0.2);
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exactly_equal() {
        assert!(is_close(1.5, 1.5));
    }

    #[test]
    fn rounding_errors() {
        assert!(is_close(0.1 + 0.2, 0.3));
        assert!(is_close(0.3, 0.1 + 0.2));
    }

    #[test]
    fn different_numbers() {
        assert!(!is_close(1.0, 1.1));
        assert!(!is_close(1.1, 1.0));
    }
}
