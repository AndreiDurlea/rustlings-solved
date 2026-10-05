fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // The value after `break` is returned from the loop.
            break counter * 2;
        }
    };

    println!("The result is {result}");
    assert_eq!(result, 20);
}
