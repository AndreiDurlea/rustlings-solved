#[derive(Clone, Copy)]
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}

fn code(status: HttpStatus) -> u16 {
    status as u16
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
