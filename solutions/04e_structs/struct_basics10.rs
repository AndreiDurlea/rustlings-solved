#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

// Returns the squared length of the line (we avoid square roots here).
fn squared_length(line: &Line) -> i32 {
    let dx = line.end.x - line.start.x;
    let dy = line.end.y - line.start.y;
    dx * dx + dy * dy
}

// Returns the middle point of the line (use integer division).
fn midpoint(line: &Line) -> Point {
    // Destructuring the nested structs.
    let Point { x: x1, y: y1 } = line.start;
    let Point { x: x2, y: y2 } = line.end;

    Point {
        x: (x1 + x2) / 2,
        y: (y1 + y2) / 2,
    }
}

// Returns `true` if the line is horizontal or vertical.
fn is_axis_aligned(line: &Line) -> bool {
    line.start.x == line.end.x || line.start.y == line.end.y
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    const DIAGONAL: Line = Line {
        start: Point { x: 1, y: 2 },
        end: Point { x: 4, y: 6 },
    };

    const HORIZONTAL: Line = Line {
        start: Point { x: -2, y: 3 },
        end: Point { x: 8, y: 3 },
    };

    #[test]
    fn lengths() {
        assert_eq!(squared_length(&DIAGONAL), 25);
        assert_eq!(squared_length(&HORIZONTAL), 100);
    }

    #[test]
    fn midpoints() {
        let m = midpoint(&HORIZONTAL);
        assert_eq!((m.x, m.y), (3, 3));
        let m = midpoint(&DIAGONAL);
        assert_eq!((m.x, m.y), (2, 4));
    }

    #[test]
    fn alignment() {
        assert!(!is_axis_aligned(&DIAGONAL));
        assert!(is_axis_aligned(&HORIZONTAL));
    }
}
