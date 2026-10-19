#[derive(Clone, Copy)]
struct Student {
    name: &'static str,
    grade: f64,
}

const N: usize = 5;

// TODO: Implement the following functions.

// Returns the average grade.
fn average_grade(students: [Student; N]) -> f64 {
    0.0
}

// Returns the name of the student with the highest grade
// (the first one in case of a tie).
fn best_student(students: [Student; N]) -> &'static str {
    ""
}

// Returns how many students passed (grade of at least 5.0).
fn passed(students: [Student; N]) -> u32 {
    0
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    const CLASS: [Student; N] = [
        Student {
            name: "Ana",
            grade: 9.5,
        },
        Student {
            name: "Bogdan",
            grade: 4.0,
        },
        Student {
            name: "Carmen",
            grade: 10.0,
        },
        Student {
            name: "Dan",
            grade: 5.0,
        },
        Student {
            name: "Elena",
            grade: 10.0,
        },
    ];

    #[test]
    fn average() {
        assert_eq!(average_grade(CLASS), 7.7);
    }

    #[test]
    fn best() {
        assert_eq!(best_student(CLASS), "Carmen");
    }

    #[test]
    fn passing() {
        assert_eq!(passed(CLASS), 4);
    }
}
