#[derive(Clone, Copy)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn is_weekend(day: Weekday) -> bool {
    hours_of_work(day) == 0
}

fn hours_of_work(day: Weekday) -> u32 {
    // Variants must be written with the enum name as prefix.
    match day {
        Weekday::Friday => 6,
        Weekday::Saturday | Weekday::Sunday => 0,
        _ => 8,
    }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    const WEEK: [Weekday; 7] = [
        Weekday::Monday,
        Weekday::Tuesday,
        Weekday::Wednesday,
        Weekday::Thursday,
        Weekday::Friday,
        Weekday::Saturday,
        Weekday::Sunday,
    ];

    #[test]
    fn weekend() {
        assert!(!is_weekend(Weekday::Monday));
        assert!(!is_weekend(Weekday::Friday));
        assert!(is_weekend(Weekday::Saturday));
        assert!(is_weekend(Weekday::Sunday));
    }

    #[test]
    fn hours() {
        let mut total = 0;
        for day in WEEK {
            total += hours_of_work(day);
        }
        assert_eq!(total, 38);
        assert_eq!(hours_of_work(Weekday::Friday), 6);
    }
}
