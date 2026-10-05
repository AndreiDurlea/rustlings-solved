fn main() {
    // TODO: Fix the compiler error by adding a type annotation for `guess`.
    // The number should be an unsigned 32-bit integer.
    let guess = "42".parse().expect("Not a number!");

    println!("Your guess plus one: {}", guess + 1);
    assert_eq!(guess + 1, 43);
}
