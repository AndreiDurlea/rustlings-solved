fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * (9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freezing() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
    }

    #[test]
    fn boiling() {
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    }

    #[test]
    fn same_value() {
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn round_trip() {
        assert_eq!(fahrenheit_to_celsius(celsius_to_fahrenheit(25.0)), 25.0);
    }
}
