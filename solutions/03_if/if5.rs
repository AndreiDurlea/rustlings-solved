fn describe(count: u32) -> &'static str {
    // The condition must be a `bool`.
    if count > 0 { "some items" } else { "empty" }
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
