struct Rectangle {
    width: u32,
    height: u32,
}

// TODO: Change both functions so that they borrow the rectangle
// instead of taking ownership of it.
fn area(rect: Rectangle) -> u32 {
    rect.width * rect.height
}

fn is_square(rect: Rectangle) -> bool {
    rect.width == rect.height
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // TODO: Fix the calls too.
    let a = area(rect);
    let square = is_square(rect);

    println!("Area: {a}, square: {square}");
    assert_eq!(a, 1500);
    assert!(!square);
}
