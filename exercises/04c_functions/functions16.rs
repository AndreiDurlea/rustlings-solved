fn add_one(x: i32) -> i32 {
    x + 1
}

fn double(x: i32) -> i32 {
    x * 2
}

// TODO: Fix the compiler error by adding the type of the parameter `f`.
// `f` is a function that takes an `i32` and returns an `i32`.
fn apply_twice(f, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twice_add_one() {
        assert_eq!(apply_twice(add_one, 5), 7);
    }

    #[test]
    fn twice_double() {
        assert_eq!(apply_twice(double, 5), 20);
    }

    #[test]
    fn function_in_a_variable() {
        let f: fn(i32) -> i32 = double;
        assert_eq!(apply_twice(f, 1), 4);
    }
}
