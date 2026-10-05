// String literals are stored in the compiled binary, so they can be used to
// initialize statics.
static GREETING: &str = "Hello";
static NAME: &str = "Ferris";

fn main() {
    println!("{GREETING}, {NAME}!");
    assert_eq!(GREETING, "Hello");
    assert_eq!(NAME.len(), 6);
}
