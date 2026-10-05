fn age_group(age: u32) -> &'static str {
    // TODO: Fix the compiler error. Ages of 65 and above are "senior".
    match age {
        0..=12 => "child",
        13..=19 => "teenager",
        20..=64 => "adult",
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
