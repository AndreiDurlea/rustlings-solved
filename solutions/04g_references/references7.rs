// Return the value itself. A reference to the local `x` would dangle.
fn create_number() -> i32 {
    42
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        assert_eq!(create_number(), 42);
    }
}
