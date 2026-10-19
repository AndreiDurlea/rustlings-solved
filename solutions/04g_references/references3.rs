fn main() {
    let mut score = 0;

    let first = &mut score;
    *first += 1; // Last use of `first`.

    let second = &mut score; // Now a new mutable reference is allowed.
    *second += 10;

    println!("Score: {score}");
    assert_eq!(score, 11);
}
