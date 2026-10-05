// The Collatz sequence starts with a positive number `n`:
// - If `n` is even, the next number is `n / 2`.
// - If `n` is odd, the next number is `3 * n + 1`.
// The sequence ends when it reaches 1.
//
// For example, starting at 6: 6 -> 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
// That's 8 steps.
//
// Return the number of steps needed to reach 1.
fn collatz_steps(mut n: u64) -> u32 {
    let mut steps = 0;

    // TODO: Write a `while` loop that computes the number of steps.

    steps
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_one() {
        assert_eq!(collatz_steps(1), 0);
    }

    #[test]
    fn small_numbers() {
        assert_eq!(collatz_steps(2), 1);
        assert_eq!(collatz_steps(6), 8);
        assert_eq!(collatz_steps(7), 16);
    }

    #[test]
    fn big_number() {
        assert_eq!(collatz_steps(27), 111);
    }
}
