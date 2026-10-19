// Returns the sum of all elements in the slice.
fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slices() {
        let small = [1, 2, 3];
        let big = [10, 20, 30, 40, 50, 60];

        // TODO: Replace each `???` with a slice of `small` or `big`
        // so that the assertions pass.
        assert_eq!(sum(???), 6); // all of `small`
        assert_eq!(sum(???), 210); // all of `big`
        assert_eq!(sum(???), 90); // 20 + 30 + 40
        assert_eq!(sum(???), 30); // the first two elements of `big`
        assert_eq!(sum(???), 110); // the last two elements of `big`
    }
}
