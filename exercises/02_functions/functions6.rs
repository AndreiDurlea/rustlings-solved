// TODO: Fix the compiler error in this function. Don't change its body!
fn add_tax(price: f64) -> f64 {
    price *= 1.19;
    price
}

fn main() {
    let total = add_tax(100.0);
    println!("Price with tax: {total}");
    assert_eq!(total, 119.0);
}
