fn main() {
    // An array must be mutable to change its elements.
    let mut scores = [0; 3];

    scores[0] = 10;
    scores[1] = 25;
    scores[2] = scores[0] + scores[1];

    println!("{scores:?}");
    assert_eq!(scores, [10, 25, 35]);
}
