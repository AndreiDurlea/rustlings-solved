// Swaps the values of the caller's two variables.
fn swap(a: &mut i32, b: &mut i32) {
    // TODO: Implement this function.
}

// Sorts the two values so that `small <= big` afterwards.
fn sort_two(small: &mut i32, big: &mut i32) {
    // TODO: Implement this function using `swap`.
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swapping() {
        let mut x = 1;
        let mut y = 2;
        swap(&mut x, &mut y);
        assert_eq!((x, y), (2, 1));
    }

    #[test]
    fn sorting() {
        let mut x = 9;
        let mut y = 3;
        sort_two(&mut x, &mut y);
        assert_eq!((x, y), (3, 9));

        sort_two(&mut x, &mut y);
        assert_eq!((x, y), (3, 9));
    }
}
