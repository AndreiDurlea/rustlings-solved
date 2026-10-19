const N: usize = 5;

// Returns a reference to the biggest element of the array
// (the first one in case of a tie).
fn largest(numbers: &[i32; N]) -> &i32 {
    // TODO: Fix this function.
    &numbers[0]
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biggest() {
        let numbers = [3, 9, -2, 9, 4];
        assert_eq!(*largest(&numbers), 9);
    }

    #[test]
    fn first_of_ties() {
        let numbers = [3, 9, -2, 9, 4];
        // `std::ptr::eq` checks that both references point to the same place.
        assert!(std::ptr::eq(largest(&numbers), &numbers[1]));
    }

    #[test]
    fn negative() {
        assert_eq!(*largest(&[-5, -1, -9, -3, -7]), -1);
    }
}
