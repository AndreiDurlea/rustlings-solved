fn change(mut numbers: [i32; 3]) -> i32 {
    numbers[0] = 99;
    numbers[0]
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies() {
        let a = [1, 2, 3];
        let mut b = a;
        b[0] = 100;

        // TODO: Replace the `???` with the correct values.
        assert_eq!(a, ???);
        assert_eq!(b, ???);

        let c = [5, 5, 5];
        let returned = change(c);
        assert_eq!(returned, ???);
        assert_eq!(c, ???);
    }
}
