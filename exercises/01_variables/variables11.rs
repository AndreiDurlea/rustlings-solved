fn main() {
    let total: i32;

    {
        // `a` and `b` only exist inside of this block.
        let a = 10;
        let b = 32;

        // TODO: Initialize `total` with the sum of `a` and `b` (exactly once
        // and without adding `mut`!) so that the code below compiles.
    }

    println!("Total: {total}");
    assert_eq!(total, 42);
}
