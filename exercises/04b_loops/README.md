# Loops and Match

Rust has three kinds of loops:

- `loop` repeats forever until you `break` out of it. It can return a value with `break value`.
- `while condition { ... }` repeats as long as the condition is `true`.
- `for element in something { ... }` iterates over ranges (`0..10`, `0..=10`), arrays and more.

`break` exits a loop, `continue` jumps to the next iteration and loop labels (`'outer:`)
let you control nested loops.

You will also meet `match`, which compares a value against a series of patterns
(literals, ranges, `|` alternatives, guards) and must cover every possible value.

## Further information

- [Repetition with Loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops)
- [The `match` Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html)
- [Pattern Syntax](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)
