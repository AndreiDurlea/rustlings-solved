const N: usize = 6;

// Returns the index of the first negative number in the array,
// or `N` if there is no negative number.
fn first_negative(numbers: [i32; N]) -> usize {
    let mut i = 0;

    while i < N {
        if numbers[i] < 0 {
            // TODO: Fix the compiler error. We want to leave the function here.
            i
        }
        i += 1;
    }

    N
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_start() {
        assert_eq!(first_negative([-1, 2, 3, 4, 5, 6]), 0);
    }

    #[test]
    fn in_the_middle() {
        assert_eq!(first_negative([1, 2, -3, 4, -5, 6]), 2);
    }

    #[test]
    fn none() {
        assert_eq!(first_negative([1, 2, 3, 4, 5, 6]), N);
    }
}
