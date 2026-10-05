// TODO: Fix the compiler error by changing the function declaration so that
// it can be called while computing a constant.
fn square(x: u32) -> u32 {
    x * x
}

const AREA: u32 = square(12);

fn main() {
    println!("The area is {AREA}");
    assert_eq!(AREA, 144);
    // A `const fn` can still be called at runtime too.
    assert_eq!(square(3), 9);
}
