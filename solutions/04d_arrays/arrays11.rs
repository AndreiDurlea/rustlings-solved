const N: usize = 7;

// Sorts the array in ascending order (smallest first).
// Don't use any sorting function from the standard library!
fn bubble_sort(numbers: &mut [i32; N]) {
    for pass in 0..N - 1 {
        // After each pass, the last `pass + 1` elements are in place.
        for j in 0..N - 1 - pass {
            if numbers[j] > numbers[j + 1] {
                (numbers[j], numbers[j + 1]) = (numbers[j + 1], numbers[j]);
            }
        }
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
    fn unsorted() {
        let mut numbers = [5, 2, 9, 1, 5, 6, -3];
        bubble_sort(&mut numbers);
        assert_eq!(numbers, [-3, 1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn reversed() {
        let mut numbers = [7, 6, 5, 4, 3, 2, 1];
        bubble_sort(&mut numbers);
        assert_eq!(numbers, [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn already_sorted() {
        let mut numbers = [1, 2, 3, 4, 5, 6, 7];
        bubble_sort(&mut numbers);
        assert_eq!(numbers, [1, 2, 3, 4, 5, 6, 7]);
    }
}
