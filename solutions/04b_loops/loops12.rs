fn age_group(age: u32) -> &'static str {
    match age {
        0..=12 => "child",
        13..=19 => "teenager",
        20..=64 => "adult",
        // `_` would work too.
        65.. => "senior",
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
    fn children() {
        assert_eq!(age_group(0), "child");
        assert_eq!(age_group(12), "child");
    }

    #[test]
    fn teenagers() {
        assert_eq!(age_group(13), "teenager");
        assert_eq!(age_group(19), "teenager");
    }

    #[test]
    fn adults() {
        assert_eq!(age_group(20), "adult");
        assert_eq!(age_group(64), "adult");
    }

    #[test]
    fn seniors() {
        assert_eq!(age_group(65), "senior");
        assert_eq!(age_group(101), "senior");
    }
}
