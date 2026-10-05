// A year is a leap year if:
// - it is divisible by 4,
// - except if it is divisible by 100,
// - unless it is also divisible by 400.
//
// So 1996, 2000 and 2024 are leap years, but 1900 and 2023 aren't.
fn is_leap_year(year: u32) -> bool {
    // TODO: Fix the condition using the logical operators `&&`, `||` and `!`.
    year % 4 == 0
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisible_by_4() {
        assert!(is_leap_year(1996));
        assert!(is_leap_year(2024));
    }

    #[test]
    fn not_divisible_by_4() {
        assert!(!is_leap_year(2023));
        assert!(!is_leap_year(2025));
    }

    #[test]
    fn divisible_by_100() {
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2100));
    }

    #[test]
    fn divisible_by_400() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2400));
    }
}
