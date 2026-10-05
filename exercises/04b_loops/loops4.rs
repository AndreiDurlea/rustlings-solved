// Returns the biggest number in the array.
fn max_of(numbers: [i32; 5]) -> i32 {
    // TODO: Fix the compiler error.
    let max = numbers[0];

    for n in numbers {
        if n > max {
            max = n;
        }
    }

    max
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_at_the_end() {
        assert_eq!(max_of([1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn max_at_the_start() {
        assert_eq!(max_of([9, 2, 3, 4, 5]), 9);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(max_of([-8, -3, -12, -5, -4]), -3);
    }
}
