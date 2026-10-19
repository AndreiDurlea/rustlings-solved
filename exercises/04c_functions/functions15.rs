// TODO: Doubling has no effect on the caller's array. Fix the function so
// that the array of the caller is modified (you need to change the parameter
// type, the body and the calls in the tests).
fn double_all(mut numbers: [i32; 4]) {
    for i in 0..4 {
        numbers[i] *= 2;
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles() {
        let mut numbers = [1, 2, 3, 4];
        double_all(numbers);
        assert_eq!(numbers, [2, 4, 6, 8]);
    }

    #[test]
    fn doubles_twice() {
        let mut numbers = [-1, 0, 5, 10];
        double_all(numbers);
        double_all(numbers);
        assert_eq!(numbers, [-4, 0, 20, 40]);
    }
}
