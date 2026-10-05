// Declared at the module level, so all functions in this file can use it.
const GREETING: &str = "Hello";

fn main() {
    println!("{GREETING} has {} letters", greeting_len());
    assert_eq!(greeting_len(), 5);
}

fn greeting_len() -> usize {
    GREETING.len()
}
