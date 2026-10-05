fn main() {
    let x = 5;

    {
        // TODO: Fix the compiler error. `x` should be 12 inside this block,
        // but still 5 after the block ends. Don't add `mut`!
        x = 12;
        println!("Inner x: {x}");
        assert_eq!(x, 12);
    }

    println!("Outer x: {x}");
    assert_eq!(x, 5);
}
