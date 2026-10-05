fn main() {
    let world_population: u64 = 8_100_000_000; // Too big for `u32`.
    let temperature: i8 = -12; // Negative, so it needs a signed type.
    let year: u16 = 2026; // `u16` goes up to 65_535 (`i16` would work too).

    println!("{world_population} people, {temperature}°C, year {year}");

    // You can find out the limits of each type like this:
    println!("u8 range: {} to {}", u8::MIN, u8::MAX);
    println!("i8 range: {} to {}", i8::MIN, i8::MAX);
}
