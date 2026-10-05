const SIZE: usize = 5;

// Returns a multiplication table where `table[i][j] == (i + 1) * (j + 1)`.
//
// 1  2  3  4  5
// 2  4  6  8 10
// 3  6  9 12 15
// 4  8 12 16 20
// 5 10 15 20 25
fn multiplication_table() -> [[usize; SIZE]; SIZE] {
    let mut table = [[0; SIZE]; SIZE];

    for row in 1..=SIZE {
        for column in 1..=SIZE {
            // The indices start at 0, the numbers in the table at 1.
            table[row - 1][column - 1] = row * column;
        }
    }

    table
}

fn main() {
    let table = multiplication_table();

    for row in table {
        for value in row {
            print!("{value:3}");
        }
        println!();
    }
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_row_and_column() {
        let table = multiplication_table();
        assert_eq!(table[0], [1, 2, 3, 4, 5]);
        assert_eq!(table[1][0], 2);
        assert_eq!(table[4][0], 5);
    }

    #[test]
    fn diagonal() {
        let table = multiplication_table();
        assert_eq!(table[2][2], 9);
        assert_eq!(table[4][4], 25);
    }

    #[test]
    fn whole_table() {
        assert_eq!(
            multiplication_table(),
            [
                [1, 2, 3, 4, 5],
                [2, 4, 6, 8, 10],
                [3, 6, 9, 12, 15],
                [4, 8, 12, 16, 20],
                [5, 10, 15, 20, 25],
            ],
        );
    }
}
