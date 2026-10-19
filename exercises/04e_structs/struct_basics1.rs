// TODO: Define the struct `Point` with two fields `x` and `y` of type `i32`.

fn main() {
    let p = Point { x: 3, y: -7 };

    println!("({}, {})", p.x, p.y);
    assert_eq!(p.x, 3);
    assert_eq!(p.y, -7);
}
