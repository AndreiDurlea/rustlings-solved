// Returns the position of the first occurrence of `needle` in `haystack`.
// If `needle` isn't found, the length of `haystack` is returned.
fn position_of(haystack: [char; 6], needle: char) -> usize {
    for (i, c) in haystack.into_iter().enumerate() {
        if c == needle {
            return i;
        }
    }

    haystack.len()
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    const LETTERS: [char; 6] = ['r', 'u', 's', 't', 'u', 'p'];

    #[test]
    fn found() {
        assert_eq!(position_of(LETTERS, 'r'), 0);
        assert_eq!(position_of(LETTERS, 's'), 2);
        assert_eq!(position_of(LETTERS, 'p'), 5);
    }

    #[test]
    fn first_occurrence() {
        assert_eq!(position_of(LETTERS, 'u'), 1);
    }

    #[test]
    fn not_found() {
        assert_eq!(position_of(LETTERS, 'x'), 6);
    }
}
