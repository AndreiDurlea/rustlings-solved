// Sums up the numbers in the array with the following rules:
// - Negative numbers are skipped.
// - A `0` marks the end of the data: it and all numbers after it are ignored.
fn sum_until_zero(numbers: [i32; 8]) -> i32 {
    let mut sum = 0;

    for n in numbers {
        if n < 0 {
            // TODO: Negative numbers should only be skipped.
            break;
        }

        // TODO: Stop the loop when `n` is zero.

        sum += n;
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
    fn only_positive() {
        assert_eq!(sum_until_zero([1, 2, 3, 4, 5, 6, 7, 8]), 36);
    }

    #[test]
    fn skip_negative() {
        assert_eq!(sum_until_zero([1, -2, 3, -4, 5, -6, 7, -8]), 16);
    }

    #[test]
    fn stop_at_zero() {
        assert_eq!(sum_until_zero([1, 2, 3, 0, 5, 6, 7, 8]), 6);
        assert_eq!(sum_until_zero([0, 1, 1, 1, 1, 1, 1, 1]), 0);
    }

    #[test]
    fn skip_and_stop() {
        assert_eq!(sum_until_zero([-1, 10, -1, 20, 0, 30, -1, 40]), 30);
    }
}
