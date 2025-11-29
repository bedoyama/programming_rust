# Chapter 3: Basic Types

This directory contains comprehensive examples for Chapter 3 of Programming Rust, covering all fundamental types in Rust.

## Running the Examples

### Main Example (All Topics)
```bash
cargo run
```

### Individual Example Files
```bash
cargo run --example numeric_ops
cargo run --example string_ops
cargo run --example collections
```

## Topics Covered

### 1. Fixed-Width Numeric Types
- Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`
- Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`
- Architecture-dependent: `isize`, `usize`
- Number literals: decimal, hex, octal, binary

### 2. Integer Types
- Different number representations (hex: `0xff`, binary: `0b1010`, octal: `0o77`)
- Type suffixes (`42i32`, `100u8`)
- Byte literals (`b'A'`)

### 3. Checked, Wrapping, Saturating, and Overflowing Arithmetic
- **Checked**: Returns `Option<T>`, `None` on overflow
  ```rust
  let result = x.checked_add(y);
  ```
- **Wrapping**: Wraps around on overflow
  ```rust
  let result = x.wrapping_add(y);
  ```
- **Saturating**: Clamps to min/max value
  ```rust
  let result = x.saturating_add(y);
  ```
- **Overflowing**: Returns `(result, bool)` indicating overflow
  ```rust
  let (result, overflowed) = x.overflowing_add(y);
  ```

### 4. Floating-Point Types
- `f32` and `f64`
- Special values: `INFINITY`, `NEG_INFINITY`, `NAN`
- Methods: `is_nan()`, `is_finite()`, `is_infinite()`

### 5. The bool Type
- Values: `true`, `false`
- Boolean operators: `&&`, `||`, `!`
- Can be cast to integers

### 6. Characters
- 32-bit Unicode scalar values
- Character literals: `'a'`, `'😊'`, `'\u{1F60E}'`
- Methods: `is_alphabetic()`, `is_numeric()`, `to_uppercase()`

### 7. Tuples
- Fixed-size heterogeneous collections: `(i32, f64, char)`
- Accessing elements: `tuple.0`, `tuple.1`
- Destructuring: `let (x, y, z) = tuple;`
- Unit type: `()` (empty tuple)

### 8. Pointer Types

#### References
- Immutable: `&T`
- Mutable: `&mut T`
- Always valid, never null

#### Boxes
- Heap allocation: `Box::new(value)`
- Owned pointer to heap data

#### Raw Pointers
- `*const T` and `*mut T`
- Can be null, require `unsafe` to dereference

### 9. Arrays
- Fixed size: `[T; N]`
- Stack allocated
- Examples: `[1, 2, 3, 4, 5]`, `[0; 100]`

### 10. Vectors
- Dynamic size: `Vec<T>`
- Heap allocated
- Create with: `Vec::new()`, `vec![1, 2, 3]`
- Methods: `push()`, `pop()`, `insert()`, `remove()`, `extend()`

### 11. Slices
- View into a sequence: `&[T]`
- Borrowed view of arrays or vectors
- Ranges: `&arr[2..5]`, `&arr[..3]`, `&arr[5..]`

### 12. String Types

#### String Literals (`&str`)
- Immutable, UTF-8 encoded
- Stored in binary
- Type: `&'static str`

#### String (`String`)
- Owned, growable, UTF-8 encoded
- Heap allocated
- Methods: `push()`, `push_str()`, `insert()`, `replace()`

#### Byte Strings
- `b"hello"` - type `&[u8; N]`
- Raw bytes, not necessarily UTF-8

### 13. Type Aliases
- Create alternative names for types
- Syntax: `type Kilometers = i32;`
- Does not create new types (no type safety)

## Key Concepts

### Memory Layout
- **Stack vs Heap**: Arrays on stack, Vectors on heap
- **String Memory**: `&str` (pointer + length), `String` (pointer + length + capacity)
- **Size**: Use `std::mem::size_of::<T>()` to check

### Type Conversions
- Explicit casting: `value as Type`
- Fallible conversion: `value.try_into()`
- From/Into traits for common conversions

### UTF-8 and Unicode
- Strings are UTF-8 encoded
- Be careful with slicing (use `char_indices()`)
- Iterate over `chars()` or `bytes()`

## Common Patterns

### Safe Element Access
```rust
// Vector/slice access
if let Some(value) = vec.get(index) {
    // use value
}
```

### String Building
```rust
let s = format!("{} + {} = {}", a, b, a + b);
```

### Collection Iteration
```rust
// Immutable
for item in &collection { }

// Mutable
for item in &mut collection { }

// Consuming
for item in collection { }
```

## Build and Test

```bash
# Build all examples
cargo build --examples

# Run main program
cargo run

# Run specific example
cargo run --example numeric_ops
cargo run --example string_ops
cargo run --example collections

# Check for errors
cargo check
```

## Additional Resources

- [The Rust Book - Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- [Rust by Example - Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)
- [Rust Reference - Types](https://doc.rust-lang.org/reference/types.html)
