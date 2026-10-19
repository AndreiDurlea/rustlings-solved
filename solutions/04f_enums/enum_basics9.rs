#[derive(Debug, PartialEq, Clone, Copy)]
enum Light {
    Red,
    Yellow,
    Green,
}

// Returns the next state of the traffic light.
fn next(light: Light) -> Light {
    match light {
        Light::Red => Light::Green,
        Light::Green => Light::Yellow,
        Light::Yellow => Light::Red,
    }
}

// Returns how many seconds the light stays in this state:
// Red: 30, Yellow: 3, Green: 25
fn duration(light: Light) -> u32 {
    match light {
        Light::Red => 30,
        Light::Yellow => 3,
        Light::Green => 25,
    }
}

// Returns the light after `seconds` seconds, starting at the beginning of `light`.
fn light_after(mut light: Light, mut seconds: u32) -> Light {
    while seconds >= duration(light) {
        seconds -= duration(light);
        light = next(light);
    }
    light
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle() {
        assert_eq!(next(Light::Red), Light::Green);
        assert_eq!(next(Light::Green), Light::Yellow);
        assert_eq!(next(Light::Yellow), Light::Red);
    }

    #[test]
    fn durations() {
        assert_eq!(duration(Light::Red), 30);
        assert_eq!(duration(Light::Yellow), 3);
        assert_eq!(duration(Light::Green), 25);
    }

    #[test]
    fn after() {
        assert_eq!(light_after(Light::Red, 0), Light::Red);
        assert_eq!(light_after(Light::Red, 29), Light::Red);
        assert_eq!(light_after(Light::Red, 30), Light::Green);
        assert_eq!(light_after(Light::Red, 56), Light::Yellow);
        assert_eq!(light_after(Light::Red, 58), Light::Red);
        assert_eq!(light_after(Light::Green, 58 * 10 + 1), Light::Green);
    }
}
