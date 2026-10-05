// This quiz tests:
// - Variables
// - Constants and statics
// - Primitive types
// - If
// - Loops and match
//
// A weather station records the temperature (in °C) once per day for a week.
// Implement the following functions:
//
// - `average`: The average temperature of the week.
// - `count_freezing`: The number of days with a temperature strictly below
//   `FREEZING_POINT`.
// - `warmest_day`: The index of the day with the highest temperature.
//   If multiple days have the same highest temperature, return the first one.
// - `classify`: Describes a temperature:
//   - below 0.0: "freezing"
//   - from 0.0 up to (but not including) 15.0: "cold"
//   - from 15.0 up to (but not including) 25.0: "mild"
//   - 25.0 and above: "hot"
// - `record_measurement`: Every call should increase the static counter
//   `MEASUREMENTS` by 1 and return the new total.

use std::sync::atomic::{AtomicU32, Ordering};

const DAYS: usize = 7;
const FREEZING_POINT: f64 = 0.0;

static MEASUREMENTS: AtomicU32 = AtomicU32::new(0);

// TODO: Implement the functions below by replacing the `todo!()`s.

fn average(temperatures: [f64; DAYS]) -> f64 {
    todo!()
}

fn count_freezing(temperatures: [f64; DAYS]) -> u32 {
    todo!()
}

fn warmest_day(temperatures: [f64; DAYS]) -> usize {
    todo!()
}

fn classify(temperature: f64) -> &'static str {
    todo!()
}

fn record_measurement() -> u32 {
    todo!()
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    const WEEK: [f64; DAYS] = [-3.5, 0.0, 4.5, 21.0, 27.5, 27.5, -1.0];

    #[test]
    fn test_average() {
        assert!((average(WEEK) - 76.0 / 7.0).abs() < 1e-9);
        assert_eq!(average([10.0; DAYS]), 10.0);
    }

    #[test]
    fn test_count_freezing() {
        assert_eq!(count_freezing(WEEK), 2);
        assert_eq!(count_freezing([1.0; DAYS]), 0);
    }

    #[test]
    fn test_warmest_day() {
        assert_eq!(warmest_day(WEEK), 4);
        assert_eq!(warmest_day([-5.0, -4.0, -3.0, -2.0, -1.0, -10.0, -20.0]), 4);
        assert_eq!(warmest_day([-1.0; DAYS]), 0);
    }

    #[test]
    fn test_classify() {
        assert_eq!(classify(-0.5), "freezing");
        assert_eq!(classify(0.0), "cold");
        assert_eq!(classify(14.9), "cold");
        assert_eq!(classify(15.0), "mild");
        assert_eq!(classify(24.9), "mild");
        assert_eq!(classify(25.0), "hot");
    }

    #[test]
    fn test_record_measurement() {
        let first = record_measurement();
        let second = record_measurement();
        assert_eq!(second, first + 1);
        assert_eq!(MEASUREMENTS.load(Ordering::Relaxed), second);
    }
}
