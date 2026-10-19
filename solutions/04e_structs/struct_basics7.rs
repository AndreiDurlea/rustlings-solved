// `Debug` allows printing with `{:?}`, `PartialEq` allows comparing with `==`.
#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn mix(a: Color, b: Color) -> Color {
    Color {
        red: a.red / 2 + b.red / 2,
        green: a.green / 2 + b.green / 2,
        blue: a.blue / 2 + b.blue / 2,
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
    fn mix_red_and_blue() {
        let red = Color {
            red: 255,
            green: 0,
            blue: 0,
        };
        let blue = Color {
            red: 0,
            green: 0,
            blue: 255,
        };

        let purple = Color {
            red: 127,
            green: 0,
            blue: 127,
        };
        assert_eq!(mix(red, blue), purple);
    }

    #[test]
    fn not_equal() {
        let black = Color {
            red: 0,
            green: 0,
            blue: 0,
        };
        let white = Color {
            red: 255,
            green: 255,
            blue: 255,
        };
        assert_ne!(black, white);
    }
}
