fn add_one(n: &mut u32) {
    *n += 1;
}

fn add_three(counter: &mut u32) {
    // TODO: Fix the compiler errors.
    add_one(&mut counter);
    add_one(&mut counter);
    add_one(&mut counter);
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
