fn main() {
    // TODO: Fix the compiler errors.
    let x = 5;

    let r = &mut x;
    r = 10;
    r += 1;

    println!("x = {x}");
    assert_eq!(x, 11);
}
