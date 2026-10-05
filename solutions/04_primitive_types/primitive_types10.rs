// The health of a player is between 0 and 255.
// Drinking a potion increases it, but it can't go above 255.
fn drink_potion(health: u8, potion: u8) -> u8 {
    health.saturating_add(potion)
}

// Taking damage reduces the health, but it can't go below 0.
fn take_damage(health: u8, damage: u8) -> u8 {
    health.saturating_sub(damage)
}

// An 8-bit counter that starts again at 0 after 255.
fn tick(counter: u8) -> u8 {
    counter.wrapping_add(1)
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn potion() {
        assert_eq!(drink_potion(100, 50), 150);
        assert_eq!(drink_potion(250, 10), 255);
        assert_eq!(drink_potion(255, 255), 255);
    }

    #[test]
    fn damage() {
        assert_eq!(take_damage(100, 30), 70);
        assert_eq!(take_damage(20, 30), 0);
    }

    #[test]
    fn counter() {
        assert_eq!(tick(0), 1);
        assert_eq!(tick(254), 255);
        assert_eq!(tick(255), 0);
    }
}
