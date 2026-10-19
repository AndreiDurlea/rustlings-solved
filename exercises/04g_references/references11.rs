const N: usize = 6;

// Counts the numbers that are bigger than `limit`.
fn count_above(numbers: &[i32; N], limit: i32) -> u32 {
    let mut count = 0;

    for n in numbers {
        // TODO: Fix the compiler error.
        if n > limit {
            count += 1;
        }
    }

    count
}

// Returns the sum of all numbers.
fn sum(numbers: &[i32; N]) -> i32 {
    let mut total: i32 = 0;

    // TODO: Fix the compiler error by changing only the loop pattern `n`.
    for n in numbers {
        let value: i32 = n;
        total += value;
    }

    total
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    const NUMBERS: [i32; N] = [5, -2, 10, 7, 0, 3];

    #[test]
    fn above() {
        assert_eq!(count_above(&NUMBERS, 4), 3);
        assert_eq!(count_above(&NUMBERS, 100), 0);
        assert_eq!(count_above(&NUMBERS, -10), 6);
    }

    #[test]
    fn total() {
        assert_eq!(sum(&NUMBERS), 23);
    }
}
