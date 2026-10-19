#[derive(Debug, PartialEq)]
enum Search {
    NotFound,
    Found(usize),
}

const N: usize = 5;

// Returns the index of the first even number in the array.
fn first_even(numbers: [i32; N]) -> Search {
    for (i, n) in numbers.into_iter().enumerate() {
        if n % 2 == 0 {
            return Search::Found(i);
        }
    }

    Search::NotFound
}

// Returns a description of a search result.
fn describe(result: Search) -> &'static str {
    match result {
        Search::Found(_) => "found",
        Search::NotFound => "not found",
    }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found() {
        assert_eq!(first_even([1, 3, 4, 5, 6]), Search::Found(2));
        assert_eq!(first_even([2, 3, 4, 5, 6]), Search::Found(0));
        assert_eq!(first_even([-1, 3, 5, 7, -8]), Search::Found(4));
    }

    #[test]
    fn not_found() {
        assert_eq!(first_even([1, 3, 5, 7, 9]), Search::NotFound);
    }

    #[test]
    fn descriptions() {
        assert_eq!(describe(Search::Found(3)), "found");
        assert_eq!(describe(Search::NotFound), "not found");
    }
}
