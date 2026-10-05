// A tuple lets a function return multiple values.
fn div_rem(a: u32, b: u32) -> (u32, u32) {
    (a / b, a % b)
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide() {
        let (quotient, remainder) = div_rem(17, 5);
        assert_eq!(quotient, 3);
        assert_eq!(remainder, 2);
    }

    #[test]
    fn no_remainder() {
        assert_eq!(div_rem(12, 4), (3, 0));
    }

    #[test]
    fn smaller_dividend() {
        assert_eq!(div_rem(3, 7), (0, 3));
    }
}
