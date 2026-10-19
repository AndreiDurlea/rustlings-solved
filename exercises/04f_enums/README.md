# Enum Basics

An enum is a type that can be exactly one of several *variants*.
Variants can be simple names (`Direction::North`), carry a number (`Ok = 200`)
or carry data (`Circle(f64)`, `Rectangle { width: f64, height: f64 }`).

You work with enums mostly through `match` (which must cover every variant) and `if let`.

In this section, enums are only used with free functions.
You will learn how to attach methods to them in the `enums` section later.

## Further information

- [Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [The `match` Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html)
- [Concise Control Flow with `if let`](https://doc.rust-lang.org/book/ch06-03-if-let.html)
