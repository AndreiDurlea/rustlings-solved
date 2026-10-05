# Constants and Statics

Besides variables, Rust has two kinds of global values:

- **Constants** (`const`) are values that are computed at compile time. They are always immutable,
  must have an explicit type annotation and are written in `SCREAMING_SNAKE_CASE`.
  The compiler inserts their value at every place where they are used (they have no fixed memory address).
- **Statics** (`static`) also need an explicit type, but they have exactly one fixed location in memory
  for the whole duration of the program (the `'static` lifetime).
  Statics are immutable. A `static mut` exists, but every access to it is `unsafe` (forbidden in Rustlings).
  To change a global value safely, use types with interior mutability like atomics instead.

As a rule of thumb: use `const` unless you really need a single shared memory location.

## Further information

- [Constants](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#constants)
- [Accessing or Modifying a Mutable Static Variable](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#accessing-or-modifying-a-mutable-static-variable)
- [Constant items (Reference)](https://doc.rust-lang.org/reference/items/constant-items.html)
- [Static items (Reference)](https://doc.rust-lang.org/reference/items/static-items.html)
- [Atomic types](https://doc.rust-lang.org/std/sync/atomic/index.html)
