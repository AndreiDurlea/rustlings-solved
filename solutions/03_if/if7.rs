fn grade(score: u32) -> char {
    // The most specific (highest) threshold has to be checked first.
    if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
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
    fn grade_a() {
        assert_eq!(grade(90), 'A');
        assert_eq!(grade(100), 'A');
    }

    #[test]
    fn grade_b() {
        assert_eq!(grade(80), 'B');
        assert_eq!(grade(89), 'B');
    }

    #[test]
    fn grade_c() {
        assert_eq!(grade(75), 'C');
    }

    #[test]
    fn grade_d() {
        assert_eq!(grade(60), 'D');
    }

    #[test]
    fn grade_f() {
        assert_eq!(grade(0), 'F');
        assert_eq!(grade(59), 'F');
    }
}
