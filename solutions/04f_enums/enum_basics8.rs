enum Reading {
    Celsius(f64),
    Fahrenheit(f64),
    Broken,
}

// Describes a temperature reading:
// - "broken" for a broken sensor
// - "hot" for 30 °C and above
// - "cold" for below 10 °C
// - "pleasant" otherwise
fn describe(reading: Reading) -> &'static str {
    match reading {
        Reading::Broken => "broken",
        // Convert Fahrenheit to Celsius and describe that.
        Reading::Fahrenheit(f) => describe(Reading::Celsius((f - 32.0) * 5.0 / 9.0)),
        Reading::Celsius(t) if t >= 30.0 => "hot",
        Reading::Celsius(t) if t < 10.0 => "cold",
        Reading::Celsius(_) => "pleasant",
    }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn broken() {
        assert_eq!(describe(Reading::Broken), "broken");
    }

    #[test]
    fn celsius() {
        assert_eq!(describe(Reading::Celsius(35.0)), "hot");
        assert_eq!(describe(Reading::Celsius(30.0)), "hot");
        assert_eq!(describe(Reading::Celsius(20.0)), "pleasant");
        assert_eq!(describe(Reading::Celsius(10.0)), "pleasant");
        assert_eq!(describe(Reading::Celsius(-5.0)), "cold");
    }

    #[test]
    fn fahrenheit() {
        assert_eq!(describe(Reading::Fahrenheit(100.0)), "hot");
        assert_eq!(describe(Reading::Fahrenheit(68.0)), "pleasant");
        assert_eq!(describe(Reading::Fahrenheit(32.0)), "cold");
    }
}
