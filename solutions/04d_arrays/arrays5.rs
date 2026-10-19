const N: usize = 6;

// Returns the sum of all elements.
fn sum(numbers: [i32; N]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}

// Returns the average of all elements.
fn average(numbers: [i32; N]) -> f64 {
    sum(numbers) as f64 / N as f64
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
        assert_eq!(sum([1, 2, 3, 4, 5, 6]), 21);
        assert_eq!(sum([-3, 3, -3, 3, 0, 0]), 0);
    }

    #[test]
    fn averages() {
        assert_eq!(average([1, 2, 3, 4, 5, 6]), 3.5);
        assert_eq!(average([6, 6, 6, 6, 6, 6]), 6.0);
        assert_eq!(average([-1, -2, -3, 0, 0, 0]), -1.0);
    }
}
