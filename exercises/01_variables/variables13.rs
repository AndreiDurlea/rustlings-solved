fn main() {
    // TODO: Fix the compiler error so that `a` and `b` can be swapped.
    let (a, b) = (1, 2);

    // Swap the values of `a` and `b` (destructuring assignment).
    (a, b) = (b, a);

    println!("a = {a}, b = {b}");
    assert_eq!(a, 2);
    assert_eq!(b, 1);
}
