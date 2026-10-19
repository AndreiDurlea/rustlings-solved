fn main() {
    // A constant is an item like a function, so nested functions can use it.
    const FACTOR: i32 = 3;

    fn scale(x: i32) -> i32 {
        x * FACTOR
    }

    println!("{}", scale(14));
    assert_eq!(scale(14), 42);
}
