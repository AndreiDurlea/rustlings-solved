fn main() {
    // `parse` can produce many types, so we have to choose one.
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("Your guess plus one: {}", guess + 1);
    assert_eq!(guess + 1, 43);
}
