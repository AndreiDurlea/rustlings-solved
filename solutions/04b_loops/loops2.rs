// Returns the sum of all decimal digits of `n`.
// For example, the digit sum of 1234 is 1 + 2 + 3 + 4 = 10.
fn digit_sum(mut n: u32) -> u32 {
    let mut sum = 0;

    // Run until all digits are removed.
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }

    sum
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(digit_sum(0), 0);
    }

    #[test]
    fn single_digit() {
        assert_eq!(digit_sum(5), 5);
    }

    #[test]
    fn multiple_digits() {
        assert_eq!(digit_sum(10), 1);
        assert_eq!(digit_sum(1234), 10);
        assert_eq!(digit_sum(99999), 45);
    }
}
