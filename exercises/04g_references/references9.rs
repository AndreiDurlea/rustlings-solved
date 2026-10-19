fn double(n: &mut i32) {
    *n *= 2;
}

fn reset(n: &mut i32) {
    *n = 0;
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elements() {
        let mut scores = [10, 20, 30, 40];

        // TODO: Call `double` and `reset` with references to single elements
        // of `scores` so that the assertion passes.

        assert_eq!(scores, [10, 40, 0, 40]);
    }
}
