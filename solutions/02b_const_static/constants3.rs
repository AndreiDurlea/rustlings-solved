fn main() {
    // Array lengths must be compile-time constants of type `usize`.
    const SIZE: usize = 5;

    let board = [0; SIZE];

    println!("The board has {} cells", board.len());
    assert_eq!(board.len(), 5);
}
