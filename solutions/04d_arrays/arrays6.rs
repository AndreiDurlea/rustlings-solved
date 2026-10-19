const N: usize = 8;

// Returns `true` if `target` is in the array.
fn contains(letters: [char; N], target: char) -> bool {
    for c in letters {
        if c == target {
            return true;
        }
    }
    false
}

// Returns how many times `target` appears in the array.
fn count(letters: [char; N], target: char) -> u32 {
    let mut n = 0;
    for c in letters {
        if c == target {
            n += 1;
        }
    }
    n
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    const WORD: [char; N] = ['b', 'a', 'n', 'a', 'n', 'a', 'z', 'z'];

    #[test]
    fn contains_letters() {
        assert!(contains(WORD, 'b'));
        assert!(contains(WORD, 'z'));
        assert!(!contains(WORD, 'x'));
    }

    #[test]
    fn count_letters() {
        assert_eq!(count(WORD, 'a'), 3);
        assert_eq!(count(WORD, 'n'), 2);
        assert_eq!(count(WORD, 'b'), 1);
        assert_eq!(count(WORD, 'x'), 0);
    }
}
