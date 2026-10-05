// Returns `true` if `n` is a prime number.
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    let mut d = 2;
    // `<=` so that `d` with `d * d == n` (like 3 for 9) is checked too.
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }

    true
}

// Counts the prime numbers in the range `from..=to`.
fn count_primes(from: u32, to: u32) -> u32 {
    let mut count = 0;

    for n in from..=to {
        if is_prime(n) {
            count += 1;
        }
    }

    count
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_primes() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7919));
    }

    #[test]
    fn not_primes() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(100));
    }

    #[test]
    fn squares_of_primes() {
        assert!(!is_prime(9));
        assert!(!is_prime(25));
        assert!(!is_prime(49));
    }

    #[test]
    fn counting() {
        assert_eq!(count_primes(1, 10), 4);
        assert_eq!(count_primes(10, 20), 4);
        assert_eq!(count_primes(1, 100), 25);
    }
}
