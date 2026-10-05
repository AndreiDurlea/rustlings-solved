// Returns 1 + 2 + ... + n
fn sum_to(n: u32) -> u32 {
    let mut total = 0;

    // `..=` includes the end.
    for i in 1..=n {
        total += i;
    }

    total
}

// Returns 1 * 2 * ... * n (and 1 for n = 0)
fn factorial(n: u64) -> u64 {
    let mut product = 1;

    // Starting at 0 would make the product always 0.
    for i in 1..=n {
        product *= i;
    }

    product
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums() {
        assert_eq!(sum_to(0), 0);
        assert_eq!(sum_to(1), 1);
        assert_eq!(sum_to(5), 15);
        assert_eq!(sum_to(100), 5050);
    }

    #[test]
    fn factorials() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(20), 2_432_902_008_176_640_000);
    }
}
