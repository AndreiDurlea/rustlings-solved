fn describe(count: u32) -> &'static str {
    // TODO: Fix the compiler error. In Rust, a number is not a `bool`.
    if count {
        "some items"
    } else {
        "empty"
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
    fn no_items() {
        assert_eq!(describe(0), "empty");
    }

    #[test]
    fn some_items() {
        assert_eq!(describe(1), "some items");
        assert_eq!(describe(99), "some items");
    }
}
