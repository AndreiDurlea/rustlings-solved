enum Payment {
    Cash,
    Card { last_digits: u16 },
    Voucher(u32),
}

// Returns the fee in cents for a payment.
// Only voucher payments have a fee: 5% of the voucher's value (in cents).
fn fee(payment: Payment) -> u32 {
    if let Payment::Voucher(value) = payment {
        value * 5 / 100
    } else {
        0
    }
}

// Returns the last digits of the card, or 0 if it isn't a card payment.
fn card_digits(payment: Payment) -> u16 {
    if let Payment::Card { last_digits } = payment {
        last_digits
    } else {
        0
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
    fn fees() {
        assert_eq!(fee(Payment::Cash), 0);
        assert_eq!(fee(Payment::Card { last_digits: 1234 }), 0);
        assert_eq!(fee(Payment::Voucher(1000)), 50);
    }

    #[test]
    fn digits() {
        assert_eq!(card_digits(Payment::Card { last_digits: 4321 }), 4321);
        assert_eq!(card_digits(Payment::Cash), 0);
        assert_eq!(card_digits(Payment::Voucher(1000)), 0);
    }
}
