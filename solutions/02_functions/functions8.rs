fn print_sum(a: i32, b: i32) -> i32 {
    let sum = a + b;
    println!("{a} + {b} = {sum}");
    sum
}

fn main() {
    let result = print_sum(2, 3);
    assert_eq!(result, 5);
}
