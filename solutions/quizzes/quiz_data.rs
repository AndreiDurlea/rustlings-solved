// This quiz tests:
// - Functions
// - Arrays
// - Structs
// - Enums
//
// A small library keeps track of its books. Implement the following
// functions (without adding methods or using collections like `Vec`):
//
// - `count_by_genre`: How many books of the given genre the library has.
// - `available_pages`: The total number of pages of all available books.
// - `longest_book`: The title of the book with the most pages
//   (the first one in case of a tie).
// - `borrow_book`: Tries to borrow the book with the given title:
//   - If no book has this title, return `Borrow::NotFound`.
//   - If the book is already borrowed (not available), return
//     `Borrow::AlreadyBorrowed`.
//   - Otherwise, mark the book as not available and return `Borrow::Success`.

#[derive(Debug, PartialEq, Clone, Copy)]
enum Genre {
    Fiction,
    Science,
    History,
}

#[derive(Clone, Copy)]
struct Book {
    title: &'static str,
    genre: Genre,
    pages: u32,
    available: bool,
}

#[derive(Debug, PartialEq)]
enum Borrow {
    Success,
    AlreadyBorrowed,
    NotFound,
}

const N: usize = 5;

fn count_by_genre(books: &[Book; N], genre: Genre) -> u32 {
    let mut count = 0;
    for book in books {
        // `==` works because `Genre` derives `PartialEq`.
        if book.genre == genre {
            count += 1;
        }
    }
    count
}

fn available_pages(books: &[Book; N]) -> u32 {
    let mut total = 0;
    for book in books {
        if book.available {
            total += book.pages;
        }
    }
    total
}

fn longest_book(books: &[Book; N]) -> &'static str {
    let mut longest = books[0];
    for book in books {
        if book.pages > longest.pages {
            longest = *book;
        }
    }
    longest.title
}

fn borrow_book(books: &mut [Book; N], title: &str) -> Borrow {
    // Iterating over `&mut [Book; N]` gives us `&mut Book`.
    for book in books {
        if book.title == title {
            if !book.available {
                return Borrow::AlreadyBorrowed;
            }
            book.available = false;
            return Borrow::Success;
        }
    }

    Borrow::NotFound
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    fn library() -> [Book; N] {
        [
            Book {
                title: "Dune",
                genre: Genre::Fiction,
                pages: 412,
                available: true,
            },
            Book {
                title: "Cosmos",
                genre: Genre::Science,
                pages: 365,
                available: false,
            },
            Book {
                title: "SPQR",
                genre: Genre::History,
                pages: 608,
                available: true,
            },
            Book {
                title: "Brief History of Time",
                genre: Genre::Science,
                pages: 256,
                available: true,
            },
            Book {
                title: "The Hobbit",
                genre: Genre::Fiction,
                pages: 608,
                available: true,
            },
        ]
    }

    #[test]
    fn genres() {
        let books = library();
        assert_eq!(count_by_genre(&books, Genre::Fiction), 2);
        assert_eq!(count_by_genre(&books, Genre::Science), 2);
        assert_eq!(count_by_genre(&books, Genre::History), 1);
    }

    #[test]
    fn pages() {
        assert_eq!(available_pages(&library()), 412 + 608 + 256 + 608);
    }

    #[test]
    fn longest() {
        assert_eq!(longest_book(&library()), "SPQR");
    }

    #[test]
    fn borrowing() {
        let mut books = library();
        assert_eq!(borrow_book(&mut books, "Dune"), Borrow::Success);
        assert!(!books[0].available);
        assert_eq!(borrow_book(&mut books, "Dune"), Borrow::AlreadyBorrowed);
        assert_eq!(borrow_book(&mut books, "Cosmos"), Borrow::AlreadyBorrowed);
        assert_eq!(borrow_book(&mut books, "Rustonomicon"), Borrow::NotFound);
        assert_eq!(available_pages(&books), 608 + 256 + 608);
    }
}
