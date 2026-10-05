fn main() {
    // `+=`, `-=`, `*=`, `/=` etc. modify the variable, so it needs to be `mut`.
    let mut counter = 0;
    counter += 1;
    counter += 2;
    counter *= 3;

    println!("Counter: {counter}");
    assert_eq!(counter, 9);
}
