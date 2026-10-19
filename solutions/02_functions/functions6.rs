// Parameters can be declared as mutable.
fn add_tax(mut price: f64) -> f64 {
    price *= 1.19;
    price
}

fn main() {
    let total = add_tax(100.0);
    println!("Price with tax: {total}");
    assert_eq!(total, 119.0);
}
