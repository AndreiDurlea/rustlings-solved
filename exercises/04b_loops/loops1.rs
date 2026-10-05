fn main() {
    let mut counter = 0;

    // TODO: Make the loop return `counter * 2` when `counter` reaches 10.
    let result = loop {
        counter += 1;

        if counter == 10 {
            break;
        }
    };

    println!("The result is {result}");
    assert_eq!(result, 20);
}
