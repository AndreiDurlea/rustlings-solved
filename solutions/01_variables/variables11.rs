fn main() {
    let total: i32;

    {
        // `a` and `b` only exist inside of this block.
        let a = 10;
        let b = 32;

        // Deferred initialization: allowed exactly once, no `mut` needed.
        total = a + b;
    }

    println!("Total: {total}");
    assert_eq!(total, 42);
}
