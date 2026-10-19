fn main() {
    let mut score = 0;

    // TODO: Fix the compiler error by reordering the lines (don't remove any).
    let first = &mut score;
    let second = &mut score;
    *first += 1;
    *second += 10;

    println!("Score: {score}");
    assert_eq!(score, 11);
}
