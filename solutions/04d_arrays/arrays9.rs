const ROWS: usize = 3;
const COLUMNS: usize = 4;

// Returns the sum of all cells.
fn total(grid: [[u32; COLUMNS]; ROWS]) -> u32 {
    let mut sum = 0;
    for row in grid {
        for cell in row {
            sum += cell;
        }
    }
    sum
}

// Returns an array with the sum of each row.
fn row_sums(grid: [[u32; COLUMNS]; ROWS]) -> [u32; ROWS] {
    let mut sums = [0; ROWS];
    for (r, row) in grid.into_iter().enumerate() {
        for cell in row {
            sums[r] += cell;
        }
    }
    sums
}

// Returns an array with the sum of each column.
fn column_sums(grid: [[u32; COLUMNS]; ROWS]) -> [u32; COLUMNS] {
    let mut sums = [0; COLUMNS];
    for row in grid {
        for (c, cell) in row.into_iter().enumerate() {
            sums[c] += cell;
        }
    }
    sums
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    const GRID: [[u32; COLUMNS]; ROWS] = [
        [1, 2, 3, 4], //
        [5, 6, 7, 8],
        [9, 10, 11, 12],
    ];

    #[test]
    fn test_total() {
        assert_eq!(total(GRID), 78);
    }

    #[test]
    fn test_rows() {
        assert_eq!(row_sums(GRID), [10, 26, 42]);
    }

    #[test]
    fn test_columns() {
        assert_eq!(column_sums(GRID), [15, 18, 21, 24]);
    }
}
