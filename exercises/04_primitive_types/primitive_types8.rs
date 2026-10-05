// Returns the exact average of three numbers as a floating point number.
fn average(a: u32, b: u32, c: u32) -> f64 {
    // TODO: Fix the compiler error. You'll need to convert the integers
    // into `f64` using `as`.
    (a + b + c) / 3
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whole_average() {
        assert_eq!(average(1, 2, 3), 2.0);
    }

    #[test]
    fn fractional_average() {
        assert_eq!(average(1, 2, 2), 5.0 / 3.0);
        assert_eq!(average(0, 0, 1), 1.0 / 3.0);
    }
}
