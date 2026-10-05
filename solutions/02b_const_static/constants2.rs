// `u64` can hold values up to about 1.8 * 10^19.
const SPEED_OF_LIGHT: u64 = 299_792_458; // meters per second
const SECONDS_PER_YEAR: u64 = 60 * 60 * 24 * 365;
const LIGHT_YEAR: u64 = SPEED_OF_LIGHT * SECONDS_PER_YEAR; // meters

fn main() {
    println!("A light year is {LIGHT_YEAR} meters");
    assert_eq!(LIGHT_YEAR, 9_454_254_955_488_000);
}
