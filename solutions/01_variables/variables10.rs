fn main() {
    let width = 4;
    let height = 5;

    // The last expression of a block (without `;`) is its value.
    let perimeter = {
        let double_width = 2 * width;
        let double_height = 2 * height;
        double_width + double_height
    };

    println!("Perimeter: {perimeter}");
    assert_eq!(perimeter, 18);
}
