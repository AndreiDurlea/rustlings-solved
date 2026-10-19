fn main() {
    let mut temperature = 20;

    // TODO: Fix the compiler error by moving one line (don't remove any).
    let old = &temperature;
    temperature += 5;
    println!("Old temperature: {old}");

    println!("New temperature: {temperature}");
    assert_eq!(temperature, 25);
}
