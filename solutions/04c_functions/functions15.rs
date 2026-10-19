// A mutable reference lets the function modify the caller's array.
fn double_all(numbers: &mut [i32; 4]) {
    // Iterating over `&mut [i32; 4]` gives `&mut i32` elements.
    for n in numbers {
        *n *= 2;
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
        double_all(&mut numbers);
        assert_eq!(numbers, [2, 4, 6, 8]);
    }

    #[test]
    fn doubles_twice() {
        let mut numbers = [-1, 0, 5, 10];
        double_all(&mut numbers);
        double_all(&mut numbers);
        assert_eq!(numbers, [-4, 0, 20, 40]);
    }
}
