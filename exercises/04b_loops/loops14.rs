// Returns the shipping cost in cents for a package of the given weight in grams:
// - Packages of 0 grams are invalid and cost `u32::MAX`.
// - Up to (and including) 500 grams cost 300 cents.
// - Up to (and including) 2000 grams cost 700 cents.
// - Heavier packages cost 700 cents + 100 cents for every started 1000 grams
//   above 2000 grams.
// - Express shipping doubles the price (but not for invalid packages).
fn shipping_cost(weight: u32, express: bool) -> u32 {
    // TODO: Complete the `match` by replacing the `todo!()`s.
    // You will need a match guard for the express shipping.
    let base = match weight {
        0 => return u32::MAX,
        1..=500 => 300,
        501..=2000 => todo!(),
        _ => todo!(),
    };

    match base {
        _ => todo!(),
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
    fn invalid() {
        assert_eq!(shipping_cost(0, false), u32::MAX);
        assert_eq!(shipping_cost(0, true), u32::MAX);
    }

    #[test]
    fn small() {
        assert_eq!(shipping_cost(1, false), 300);
        assert_eq!(shipping_cost(500, false), 300);
    }

    #[test]
    fn medium() {
        assert_eq!(shipping_cost(501, false), 700);
        assert_eq!(shipping_cost(2000, false), 700);
    }

    #[test]
    fn heavy() {
        assert_eq!(shipping_cost(2001, false), 800);
        assert_eq!(shipping_cost(3000, false), 800);
        assert_eq!(shipping_cost(3001, false), 900);
        assert_eq!(shipping_cost(10000, false), 1500);
    }

    #[test]
    fn express() {
        assert_eq!(shipping_cost(100, true), 600);
        assert_eq!(shipping_cost(1500, true), 1400);
        assert_eq!(shipping_cost(3001, true), 1800);
    }
}
