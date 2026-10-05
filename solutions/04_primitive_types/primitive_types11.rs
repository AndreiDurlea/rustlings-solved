// Converts a digit character like '7' into the number 7.
fn digit_value(c: char) -> u32 {
    // Alternative: `c.to_digit(10).unwrap()`
    c as u32 - '0' as u32
}

// Shifts a lowercase ASCII letter by one position in the alphabet.
// 'a' becomes 'b', 'b' becomes 'c', ..., and 'z' becomes 'a' again.
fn shift_letter(c: char) -> char {
    let position = c as u8 - b'a'; // 0 to 25
    ((position + 1) % 26 + b'a') as char
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits() {
        assert_eq!(digit_value('0'), 0);
        assert_eq!(digit_value('7'), 7);
        assert_eq!(digit_value('9'), 9);
    }

    #[test]
    fn shift() {
        assert_eq!(shift_letter('a'), 'b');
        assert_eq!(shift_letter('m'), 'n');
        assert_eq!(shift_letter('y'), 'z');
    }

    #[test]
    fn shift_wraps_around() {
        assert_eq!(shift_letter('z'), 'a');
    }
}
