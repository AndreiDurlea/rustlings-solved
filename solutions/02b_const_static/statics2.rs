static WEEKDAYS: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

// Data stored in a static lives for the whole program: `'static`.
fn weekday_name(index: usize) -> &'static str {
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
