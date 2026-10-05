fn main() {
    let width = 4;
    let height = 5;

    // TODO: The block should evaluate to the perimeter. Fix it so that the
    // assertion compiles and passes.
    let perimeter = {
        let double_width = 2 * width;
        let double_height = 2 * height;
        double_width + double_height;
    };

    println!("Perimeter: {perimeter}");
    assert_eq!(perimeter, 18);
}
