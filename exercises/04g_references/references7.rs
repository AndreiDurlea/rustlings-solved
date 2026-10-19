// TODO: Fix the compiler error. Change the return type and the body.
fn create_number() -> &i32 {
    let x = 42;
    &x
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
