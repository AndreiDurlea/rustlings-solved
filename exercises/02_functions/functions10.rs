fn main() {
    let factor = 3;

    // TODO: Fix the compiler error. A nested function can't use the local
    // variables of the function it is defined in.
    fn scale(x: i32) -> i32 {
        x * factor
    }

    println!("{}", scale(14));
    assert_eq!(scale(14), 42);
}
