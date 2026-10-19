struct Meters(f64);
struct Feet(f64);

fn feet_to_meters(feet: Feet) -> Meters {
    Meters(feet.0 * 0.3048)
}

// TODO: Fix this function. It should return the total in meters.
fn total_distance(a: Meters, b: Feet) -> Meters {
    Meters(a.0 + b)
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert() {
        assert_eq!(feet_to_meters(Feet(10.0)).0, 3.048);
    }

    #[test]
    fn total() {
        let total = total_distance(Meters(100.0), Feet(1000.0));
        assert_eq!(total.0, 404.8);
    }
}
