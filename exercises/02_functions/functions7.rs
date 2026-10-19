// Splits a duration given in seconds into (hours, minutes, seconds).
// For example, 3725 seconds are 1 hour, 2 minutes and 5 seconds.
// TODO: Add the return type and implement the function.
fn split_duration(total_seconds: u32) {}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_seconds() {
        assert_eq!(split_duration(42), (0, 0, 42));
    }

    #[test]
    fn minutes_and_seconds() {
        assert_eq!(split_duration(125), (0, 2, 5));
    }

    #[test]
    fn everything() {
        let (hours, minutes, seconds) = split_duration(3725);
        assert_eq!(hours, 1);
        assert_eq!(minutes, 2);
        assert_eq!(seconds, 5);
    }

    #[test]
    fn full_day() {
        assert_eq!(split_duration(86399), (23, 59, 59));
    }
}
