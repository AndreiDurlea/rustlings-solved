// Counts how many of the three arguments are `true`.
fn count_true(a: bool, b: bool, c: bool) -> u32 {
    // TODO: Fix the compiler error.
    a + b + c
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none() {
        assert_eq!(count_true(false, false, false), 0);
    }

    #[test]
    fn some() {
        assert_eq!(count_true(true, false, false), 1);
        assert_eq!(count_true(false, true, true), 2);
    }

    #[test]
    fn all() {
        assert_eq!(count_true(true, true, true), 3);
    }
}
