// Finds the first pair `(a, b)` with `1 <= a <= b <= 10` and `a * b == target`.
// "First" means the smallest `a`. Returns `(0, 0)` if there is no such pair.
fn find_factors(target: u32) -> (u32, u32) {
    let mut result = (0, 0);

    'outer: for a in 1..=10 {
        for b in a..=10 {
            if a * b == target {
                result = (a, b);
                // Exits both loops.
                break 'outer;
            }
        }
    }

    result
}

// An alternative is returning directly from the inner loop:
// `return (a, b);` and then `(0, 0)` after the loops.

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_pair() {
        assert_eq!(find_factors(49), (7, 7));
    }

    #[test]
    fn first_pair() {
        assert_eq!(find_factors(12), (2, 6));
        assert_eq!(find_factors(24), (3, 8));
        assert_eq!(find_factors(36), (4, 9));
    }

    #[test]
    fn no_pair() {
        assert_eq!(find_factors(13), (0, 0));
    }
}
