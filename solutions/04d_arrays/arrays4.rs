const N: usize = 5;

// Returns the element at `index`, or 0 if the index is out of bounds.
fn get_or_zero(numbers: [i32; N], index: usize) -> i32 {
    if index < N { numbers[index] } else { 0 }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    const NUMBERS: [i32; N] = [10, 20, 30, 40, 50];

    #[test]
    fn valid_index() {
        assert_eq!(get_or_zero(NUMBERS, 0), 10);
        assert_eq!(get_or_zero(NUMBERS, 4), 50);
    }

    #[test]
    fn invalid_index() {
        assert_eq!(get_or_zero(NUMBERS, 5), 0);
        assert_eq!(get_or_zero(NUMBERS, 1000), 0);
    }
}
