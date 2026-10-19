#[derive(Clone, Copy)]
enum Op {
    Add(i32),
    Subtract(i32),
    Multiply(i32),
    Divide(i32),
    Reset,
}

const N: usize = 6;

// Starts with the value 0 and applies all operations in order.
// - `Reset` sets the value back to 0.
// - Dividing by 0 is ignored (the value stays the same).
fn run(ops: [Op; N]) -> i32 {
    let mut value = 0;

    for op in ops {
        value = match op {
            Op::Add(n) => value + n,
            Op::Subtract(n) => value - n,
            Op::Multiply(n) => value * n,
            Op::Divide(0) => value,
            Op::Divide(n) => value / n,
            Op::Reset => 0,
        };
    }

    value
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let ops = [
            Op::Add(5),
            Op::Multiply(4),
            Op::Subtract(2),
            Op::Divide(3),
            Op::Add(1),
            Op::Multiply(-1),
        ];
        assert_eq!(run(ops), -7);
    }

    #[test]
    fn reset() {
        let ops = [
            Op::Add(100),
            Op::Reset,
            Op::Add(1),
            Op::Add(1),
            Op::Multiply(21),
            Op::Subtract(0),
        ];
        assert_eq!(run(ops), 42);
    }

    #[test]
    fn divide_by_zero() {
        let ops = [
            Op::Add(10),
            Op::Divide(0),
            Op::Divide(2),
            Op::Divide(0),
            Op::Subtract(1),
            Op::Divide(0),
        ];
        assert_eq!(run(ops), 4);
    }
}
