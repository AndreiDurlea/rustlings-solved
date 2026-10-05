const SECONDS_PER_MINUTE: u32 = 60;

fn main() {
    // Constants can only be computed from other constants and literals.
    const MINUTES_PER_HOUR: u32 = 60;
    const SECONDS_PER_HOUR: u32 = SECONDS_PER_MINUTE * MINUTES_PER_HOUR;

    println!("One hour has {SECONDS_PER_HOUR} seconds");
    assert_eq!(SECONDS_PER_HOUR, 3600);
}
