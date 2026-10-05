// Returns:
// - "FizzBuzz" if `n` is divisible by both 3 and 5,
// - "Fizz" if `n` is only divisible by 3,
// - "Buzz" if `n` is only divisible by 5,
// - "Number" otherwise.
fn fizzbuzz(n: u32) -> &'static str {
    // The most specific pattern has to come first.
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz",
        (0, _) => "Fizz",
        (_, 0) => "Buzz",
        _ => "Number",
    }
}

// Returns:
// - "vowel" for the lowercase vowels 'a', 'e', 'i', 'o' and 'u',
// - "consonant" for all other lowercase letters from 'a' to 'z',
// - "other" for everything else.
fn letter_kind(c: char) -> &'static str {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
        // The vowels were already matched above.
        'a'..='z' => "consonant",
        _ => "other",
    }
}

fn main() {
    for n in 1..=15 {
        println!("{n}: {}", fizzbuzz(n));
    }
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizz() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(9), "Fizz");
    }

    #[test]
    fn buzz() {
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(20), "Buzz");
    }

    #[test]
    fn fizzbuzz_both() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(0), "FizzBuzz");
    }

    #[test]
    fn number() {
        assert_eq!(fizzbuzz(1), "Number");
        assert_eq!(fizzbuzz(7), "Number");
    }

    #[test]
    fn letters() {
        for c in ['a', 'e', 'i', 'o', 'u'] {
            assert_eq!(letter_kind(c), "vowel");
        }

        for c in ['b', 'm', 'y', 'z'] {
            assert_eq!(letter_kind(c), "consonant");
        }

        for c in ['A', 'E', '1', ' ', '?'] {
            assert_eq!(letter_kind(c), "other");
        }
    }
}
