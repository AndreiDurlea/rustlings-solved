#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants() {
        let all = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        assert_eq!(all[2], Direction::South);
        assert_ne!(all[0], all[1]);
    }
}
