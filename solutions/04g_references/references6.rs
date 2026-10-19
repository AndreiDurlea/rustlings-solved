fn add_one(n: &mut u32) {
    *n += 1;
}

fn add_three(counter: &mut u32) {
    // `counter` is already a mutable reference, pass it on directly.
    add_one(counter);
    add_one(counter);
    add_one(counter);
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three() {
        let mut points = 0;
        add_three(&mut points);
        assert_eq!(points, 3);
        add_three(&mut points);
        assert_eq!(points, 6);
    }
}
