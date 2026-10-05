// TODO: The compiler evaluates constants and complains that the computation
// overflows. Change the types (not the values) so that everything fits.
const SPEED_OF_LIGHT: u32 = 299_792_458; // meters per second
const SECONDS_PER_YEAR: u32 = 60 * 60 * 24 * 365;
const LIGHT_YEAR: u32 = SPEED_OF_LIGHT * SECONDS_PER_YEAR; // meters

fn main() {
    println!("A light year is {LIGHT_YEAR} meters");
    assert_eq!(LIGHT_YEAR, 9_454_254_955_488_000);
}
