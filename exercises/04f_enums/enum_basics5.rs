// TODO: Give each variant the right HTTP status code (see the tests).
#[derive(Clone, Copy)]
enum HttpStatus {
    Ok,
    NotFound,
    InternalServerError,
}

fn code(status: HttpStatus) -> u16 {
    // TODO: Convert the status into its number.
    0
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes() {
        assert_eq!(code(HttpStatus::Ok), 200);
        assert_eq!(code(HttpStatus::NotFound), 404);
        assert_eq!(code(HttpStatus::InternalServerError), 500);
    }
}
