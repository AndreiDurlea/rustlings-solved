// Sets every element of the slice to `value`.
fn fill(slice: &mut [i32], value: i32) {
    for n in slice {
        *n = value;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whole_array() {
        let mut numbers = [1, 2, 3];
        fill(&mut numbers, 0);
        assert_eq!(numbers, [0, 0, 0]);
    }

    #[test]
    fn parts() {
        let mut numbers = [0; 8];

        fill(&mut numbers[1..4], 7);
        fill(&mut numbers[6..], 9);

        assert_eq!(numbers, [0, 7, 7, 7, 0, 0, 9, 9]);
    }
}
