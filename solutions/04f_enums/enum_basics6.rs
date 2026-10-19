#[derive(Clone, Copy)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, Copy)]
struct Card {
    suit: Suit,
    rank: u8, // 1 (ace) to 13 (king)
}

fn is_red(card: Card) -> bool {
    match card.suit {
        Suit::Diamonds | Suit::Hearts => true,
        Suit::Clubs | Suit::Spades => false,
    }
}

// Returns the Blackjack value of the card (see the hint).
fn value(card: Card) -> u32 {
    match card.rank {
        1 => 11,
        11..=13 => 10,
        n => n as u32,
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
    fn colors() {
        assert!(is_red(Card {
            suit: Suit::Hearts,
            rank: 5,
        }));
        assert!(is_red(Card {
            suit: Suit::Diamonds,
            rank: 12,
        }));
        assert!(!is_red(Card {
            suit: Suit::Spades,
            rank: 1,
        }));
        assert!(!is_red(Card {
            suit: Suit::Clubs,
            rank: 7,
        }));
    }

    #[test]
    fn values() {
        let ace = Card {
            suit: Suit::Spades,
            rank: 1,
        };
        let seven = Card {
            suit: Suit::Hearts,
            rank: 7,
        };
        let king = Card {
            suit: Suit::Clubs,
            rank: 13,
        };
        assert_eq!(value(ace), 11);
        assert_eq!(value(seven), 7);
        assert_eq!(value(king), 10);
        assert_eq!(value(ace) + value(king), 21);
    }
}
