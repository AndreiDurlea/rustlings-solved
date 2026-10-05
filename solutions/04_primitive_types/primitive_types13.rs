// The length is part of the array type.
fn sum4(numbers: [i32; 4]) -> i32 {
    numbers[0] + numbers[1] + numbers[2] + numbers[3]
}

// Returns the element at `row` and `column` of a 2D grid (an array of arrays).
fn get_cell(grid: [[char; 3]; 2], row: usize, column: usize) -> char {
    // First select the row (an inner array), then the column inside of it.
    grid[row][column]
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_four() {
        assert_eq!(sum4([1, 2, 3, 4]), 10);
        assert_eq!(sum4([-5, 5, 0, 7]), 7);
    }

    #[test]
    fn grid_cells() {
        let grid = [['a', 'b', 'c'], ['d', 'e', 'f']];
        assert_eq!(get_cell(grid, 0, 0), 'a');
        assert_eq!(get_cell(grid, 0, 2), 'c');
        assert_eq!(get_cell(grid, 1, 0), 'd');
        assert_eq!(get_cell(grid, 1, 2), 'f');
    }
}
