fn main() {
    // TODO: Fix the compiler error by turning `size` into a constant.
    let size = 5;

    let board = [0; size];

    println!("The board has {} cells", board.len());
    assert_eq!(board.len(), 5);
}
