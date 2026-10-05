fn main() {
    let x = 5;

    {
        // Shadowing: a new `x` that only lives until the end of this block.
        let x = 12;
        println!("Inner x: {x}");
        assert_eq!(x, 12);
    }

    println!("Outer x: {x}");
    assert_eq!(x, 5);
}
