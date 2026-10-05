// TODO: Fix the compiler errors. The value of a static has to be known at
// compile time. Change the type and the value, but keep the text.
static GREETING: String = String::from("Hello");
static NAME: String = String::from("Ferris");

fn main() {
    println!("{GREETING}, {NAME}!");
    assert_eq!(GREETING, "Hello");
    assert_eq!(NAME.len(), 6);
}
