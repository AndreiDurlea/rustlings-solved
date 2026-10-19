# References

A reference lets you use a value without taking ownership of it. This is called *borrowing*.

- `&x` creates a shared reference: you can read the value, but not change it.
- `&mut x` creates a mutable reference: you can change the value (`*r = 5`).
- `*r` dereferences a reference to get to the value behind it.

The borrow checker enforces two rules:

1. At any time, you can have *either* one mutable reference *or* any number of shared references to a value.
2. References must always be valid: a reference can't outlive the value it points to.

A reference only counts as "alive" until its last use.

## Further information

- [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [Dereference operator (Reference)](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-dereference-operator)
