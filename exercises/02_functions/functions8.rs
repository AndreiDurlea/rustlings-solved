// TODO: Fix the function so that it prints AND returns the sum.
fn print_sum(a: i32, b: i32) {
    let sum = a + b;
    println!("{a} + {b} = {sum}");
}

fn main() {
    let result = print_sum(2, 3);
    assert_eq!(result, 5);
}
