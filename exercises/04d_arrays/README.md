# Arrays

An array is a fixed-size list of values of the same type, stored next to each other in memory.
Its type contains the element type and the length: `[i32; 5]`.

- Arrays are created with `[1, 2, 3]` or `[0; 100]` (100 zeros).
- Elements are accessed by index starting at 0: `numbers[0]`. An out-of-bounds index makes the program panic.
- Arrays of `Copy` types (like numbers) are copied when assigned or passed to a function.
  Use a mutable reference (`&mut [i32; 5]`) to let a function change the caller's array.
- A slice (`&[i32]`) is a view into (a part of) an array. Its length is not part of the type.

## Further information

- [The Array Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)
- [Other Slices](https://doc.rust-lang.org/book/ch04-03-slices.html#other-slices)
- [Arrays and Slices (Rust by Example)](https://doc.rust-lang.org/rust-by-example/primitives/array.html)
