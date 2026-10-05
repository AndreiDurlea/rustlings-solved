fn main() {
    // Each binding in the pattern can be marked as `mut` individually.
    let (mut a, mut b) = (1, 2);

    (a, b) = (b, a);

    println!("a = {a}, b = {b}");
    assert_eq!(a, 2);
    assert_eq!(b, 1);
}
