static WEEKDAYS: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

// TODO: Fix the compiler error by adding the right lifetime to the return type.
// Hint: The returned string lives as long as the whole program.
fn weekday_name(index: usize) -> &str {
    WEEKDAYS[index]
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_day() {
        assert_eq!(weekday_name(0), "Monday");
    }

    #[test]
    fn last_day() {
        assert_eq!(weekday_name(6), "Sunday");
    }
}
