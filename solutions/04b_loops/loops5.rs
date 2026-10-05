// Builds the number `n (n-1) ... 2 1` from its digits.
// For example, `countdown_number(5)` is 54321.
fn countdown_number(n: u64) -> u64 {
    let mut result = 0;

    for digit in (1..=n).rev() {
        result = result * 10 + digit;
    }

    result
}

// Returns the sum of 0, 3, 6, 9, ... up to and including `limit`.
fn sum_of_multiples_of_3(limit: u32) -> u32 {
    let mut sum = 0;

    for n in (0..=limit).step_by(3) {
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
    fn countdown() {
        assert_eq!(countdown_number(1), 1);
        assert_eq!(countdown_number(5), 54321);
        assert_eq!(countdown_number(9), 987654321);
    }

    #[test]
    fn multiples_of_3() {
        assert_eq!(sum_of_multiples_of_3(2), 0);
        assert_eq!(sum_of_multiples_of_3(3), 3);
        assert_eq!(sum_of_multiples_of_3(10), 18);
        assert_eq!(sum_of_multiples_of_3(12), 30);
    }
}
