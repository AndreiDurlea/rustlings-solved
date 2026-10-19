// Returns the n-th Fibonacci number: 0, 1, 1, 2, 3, 5, 8, 13, 21, ...
fn fibonacci(n: u32) -> u64 {
    if n < 2 {
        // Base cases: fib(0) = 0 and fib(1) = 1
        n as u64
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
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
    fn base_cases() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn small() {
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(7), 13);
    }

    #[test]
    fn bigger() {
        assert_eq!(fibonacci(20), 6765);
    }
}
