fn main() {
    // TODO: Fix the compiler error.
    let scores = [0; 3];

    scores[0] = 10;
    scores[1] = 25;
    scores[2] = scores[0] + scores[1];

    println!("{scores:?}");
    assert_eq!(scores, [10, 25, 35]);
}
