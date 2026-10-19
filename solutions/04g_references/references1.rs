fn is_positive(n: &i32) -> bool {
    // `*n` is the value behind the reference.
    *n > 0
}

fn add_ten(n: &i32) -> i32 {
    let ten: i32 = 10;
    let result: i32 = *n;
    result + ten
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive() {
        let x = 5;
        assert!(is_positive(&x));
        assert!(!is_positive(&-3));
        assert!(!is_positive(&0));
    }

    #[test]
    fn plus_ten() {
        let x = 32;
        assert_eq!(add_ten(&x), 42);
        // `x` can still be used because it was only borrowed.
        assert_eq!(x, 32);
    }
}
