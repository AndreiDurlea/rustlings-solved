// Returns the sum of all elements in the slice.
fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slices() {
        let small = [1, 2, 3];
        let big = [10, 20, 30, 40, 50, 60];

        assert_eq!(sum(&small), 6); // all of `small`
        assert_eq!(sum(&big), 210); // all of `big`
        assert_eq!(sum(&big[1..4]), 90); // 20 + 30 + 40
        assert_eq!(sum(&big[..2]), 30); // the first two elements of `big`
        assert_eq!(sum(&big[4..]), 110); // the last two elements of `big`
    }
}
