const N: usize = 4;

// Returns the name of the oldest person.
// If several people have the same age, the first one is returned.
fn oldest(people: [(&'static str, u32); N]) -> &'static str {
    // TODO: Implement this function.
    ""
}

// Returns the sum of the ages of all people.
fn total_age(people: [(&'static str, u32); N]) -> u32 {
    // TODO: Implement this function.
    0
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    const PEOPLE: [(&str, u32); N] = [("Ana", 31), ("Bogdan", 45), ("Carmen", 27), ("Dan", 45)];

    #[test]
    fn find_oldest() {
        assert_eq!(oldest(PEOPLE), "Bogdan");
        assert_eq!(oldest([("A", 1), ("B", 2), ("C", 3), ("D", 4)]), "D");
        assert_eq!(oldest([("A", 9), ("B", 2), ("C", 3), ("D", 4)]), "A");
    }

    #[test]
    fn sum_ages() {
        assert_eq!(total_age(PEOPLE), 148);
    }
}
