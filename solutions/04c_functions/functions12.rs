// Returns the greatest common divisor of `a` and `b` (Euclidean algorithm).
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// Returns the least common multiple of `a` and `b`.
// Hint: lcm(a, b) = a * b / gcd(a, b)
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_works() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(18, 12), 6);
        assert_eq!(gcd(17, 5), 1);
        assert_eq!(gcd(7, 0), 7);
        assert_eq!(gcd(1071, 462), 21);
    }

    #[test]
    fn lcm_works() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(5, 7), 35);
        assert_eq!(lcm(21, 6), 42);
    }
}
