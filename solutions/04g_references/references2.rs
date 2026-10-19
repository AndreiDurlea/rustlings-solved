fn main() {
    // A mutable reference requires a mutable variable.
    let mut x = 5;

    let r = &mut x;
    // Dereference to change the value behind the reference.
    *r = 10;
    *r += 1;

    println!("x = {x}");
    assert_eq!(x, 11);
}
