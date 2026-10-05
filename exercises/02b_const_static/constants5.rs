fn main() {
    const GREETING: &str = "Hello";

    println!("{GREETING} has {} letters", greeting_len());
    assert_eq!(greeting_len(), 5);
}

fn greeting_len() -> usize {
    // TODO: `GREETING` isn't visible here. Move its declaration so that
    // both functions can use it.
    GREETING.len()
}
