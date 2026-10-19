const N: usize = 6;

// Returns a new array with the elements in reverse order.
fn reversed(numbers: [i32; N]) -> [i32; N] {
    let mut result = [0; N];

    for (i, n) in numbers.into_iter().enumerate() {
        result[N - 1 - i] = n;
    }

    result
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse() {
        assert_eq!(reversed([1, 2, 3, 4, 5, 6]), [6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn reverse_twice() {
        let numbers = [7, -3, 0, 12, 5, 5];
        assert_eq!(reversed(reversed(numbers)), numbers);
    }
}
