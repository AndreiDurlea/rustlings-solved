fn sign(n: i32) -> &'static str {
    // TODO: Fix the compiler error. What happens if `n` is neither positive
    // nor negative?
    if n > 0 {
        "positive"
    } else if n < 0 {
        "negative"
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
    fn positive() {
        assert_eq!(sign(42), "positive");
    }

    #[test]
    fn negative() {
        assert_eq!(sign(-7), "negative");
    }

    #[test]
    fn zero() {
        assert_eq!(sign(0), "zero");
    }
}
