fn main() {
    let mut temperature = 20;

    let old = &temperature;
    println!("Old temperature: {old}"); // Last use of `old`.
    temperature += 5; // Now the value can be changed again.

    println!("New temperature: {temperature}");
    assert_eq!(temperature, 25);
}
