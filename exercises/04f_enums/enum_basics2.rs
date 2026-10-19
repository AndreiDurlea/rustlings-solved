#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

// TODO: Fix the compiler error.
fn opposite(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::South => Direction::North,
    }
}

// TODO: Implement this function using a `match`.
// Returns the direction after turning 90 degrees clockwise.
fn turn_right(direction: Direction) -> Direction {
    direction
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposites() {
        assert_eq!(opposite(Direction::North), Direction::South);
        assert_eq!(opposite(Direction::East), Direction::West);
        assert_eq!(opposite(Direction::South), Direction::North);
        assert_eq!(opposite(Direction::West), Direction::East);
    }

    #[test]
    fn turns() {
        assert_eq!(turn_right(Direction::North), Direction::East);
        assert_eq!(turn_right(Direction::East), Direction::South);
        assert_eq!(turn_right(Direction::South), Direction::West);
        assert_eq!(turn_right(Direction::West), Direction::North);
    }
}
