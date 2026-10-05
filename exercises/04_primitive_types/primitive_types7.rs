fn main() {
    // TODO: Fix the compiler errors by choosing integer types that can hold
    // the values. Try to pick the smallest type that fits!
    let world_population: u8 = 8_100_000_000;
    let temperature: u8 = -12;
    let year: i8 = 2026;

    println!("{world_population} people, {temperature}°C, year {year}");

    // You can find out the limits of each type like this:
    println!("u8 range: {} to {}", u8::MIN, u8::MAX);
    println!("i8 range: {} to {}", i8::MIN, i8::MAX);
}
