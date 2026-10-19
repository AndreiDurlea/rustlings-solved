fn change(mut numbers: [i32; 3]) -> i32 {
    numbers[0] = 99;
    numbers[0]
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies() {
        let a = [1, 2, 3];
        let mut b = a;
        b[0] = 100;

        // `b` is a copy of `a`. Changing `b` doesn't change `a`.
        assert_eq!(a, [1, 2, 3]);
        assert_eq!(b, [100, 2, 3]);

        let c = [5, 5, 5];
        let returned = change(c);
        // The function changed its own copy only.
        assert_eq!(returned, 99);
        assert_eq!(c, [5, 5, 5]);
    }
}
