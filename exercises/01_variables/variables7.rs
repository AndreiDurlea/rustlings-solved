fn main() {
    // TODO: Fix the compiler error without removing or reordering any lines.
    let counter = 0;
    counter += 1;
    counter += 2;
    counter *= 3;

    println!("Counter: {counter}");
    assert_eq!(counter, 9);
}
