// The function receives a mutable reference to the caller's variable.
fn increase(counter: &mut u32, amount: u32) {
    *counter += amount;
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increase_once() {
        let mut points = 10;
        increase(&mut points, 5);
        assert_eq!(points, 15);
    }

    #[test]
    fn increase_twice() {
        let mut points = 0;
        increase(&mut points, 1);
        increase(&mut points, 2);
        assert_eq!(points, 3);
    }
}
