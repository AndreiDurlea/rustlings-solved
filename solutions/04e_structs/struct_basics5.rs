struct Rectangle {
    width: u32,
    height: u32,
}

// Borrowing with `&` doesn't move the struct into the function.
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn is_square(rect: &Rectangle) -> bool {
    rect.width == rect.height
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let a = area(&rect);
    let square = is_square(&rect);

    println!("Area: {a}, square: {square}");
    assert_eq!(a, 1500);
    assert!(!square);
}
