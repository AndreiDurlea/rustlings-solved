const SECONDS_PER_MINUTE: u32 = 60;

fn main() {
    let minutes_per_hour = 60;

    // TODO: Fix the compiler error. A constant can only be computed from
    // literals and other constants.
    const SECONDS_PER_HOUR: u32 = SECONDS_PER_MINUTE * minutes_per_hour;

    println!("One hour has {SECONDS_PER_HOUR} seconds");
    assert_eq!(SECONDS_PER_HOUR, 3600);
}
