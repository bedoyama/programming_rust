# Chapter 3 Examples - Quick Reference

## Project Structure

```
ch03/
├── Cargo.toml
├── README.md (detailed documentation)
├── SUMMARY.md (this file)
├── src/
│   └── main.rs (comprehensive examples for all topics)
└── examples/
    ├── numeric_ops.rs (advanced numeric operations)
    ├── string_ops.rs (advanced string handling)
    └── collections.rs (arrays, vectors, slices)
```

## Running Examples

```bash
cd ch03

# Run main program (all topics)
cargo run

# Run individual examples
cargo run --example numeric_ops
cargo run --example string_ops
cargo run --example collections
```

## Quick Topic Index

### Numeric Types

- **Fixed-width integers**: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`
- **Architecture-dependent**: `isize`, `usize`
- **Floating-point**: `f32`, `f64`

### Arithmetic Methods

```rust
x.checked_add(y)    // Returns Option<T>
x.wrapping_add(y)   // Wraps on overflow
x.saturating_add(y) // Clamps to min/max
x.overflowing_add(y) // Returns (result, bool)
```

### Bool & Char

```rust
let b: bool = true;
let c: char = '😊'; // 4-byte Unicode
```

### Tuples

```rust
let tuple = (42, 3.14, 'x');
let (a, b, c) = tuple; // destructuring
```

### Pointers

```rust
let r = &x;           // immutable reference
let mr = &mut y;      // mutable reference
let b = Box::new(10); // heap allocation
```

### Collections

```rust
let arr = [1, 2, 3, 4, 5];           // array
let vec = vec![1, 2, 3];             // vector
let slice = &arr[1..4];              // slice
```

### Strings

```rust
let s: &str = "literal";             // string literal
let owned = String::from("owned");   // owned string
let bytes = b"bytes";                // byte string
```

### Type Aliases

```rust
type Kilometers = i32;
```

## Example Output Preview

### Main Program

Covers all topics with simple, clear examples showing:

- Number representations and ranges
- All arithmetic overflow modes
- Float special values (infinity, NaN)
- Character Unicode handling
- Tuple creation and destructuring
- Pointer types (references, Box, raw)
- Array, vector, and slice operations
- String types and conversions
- Type aliases

### numeric_ops.rs

Advanced numeric operations:

- Overflow behavior demonstration
- Type conversions and casting
- Bitwise operations
- Method chaining

### string_ops.rs

Advanced string handling:

- Memory layout details
- String manipulation (push, insert, replace, trim)
- Searching and pattern matching
- Unicode character handling
- String formatting

### collections.rs

Advanced collection operations:

- Array methods (sort, reverse, split, windows)
- Vector operations (push, pop, drain, extend)
- Slice operations (split, binary search)
- Type conversions between collections

## Key Learning Points

1. **Type Safety**: Rust requires explicit type conversions
2. **Overflow Handling**: Four modes for arithmetic overflow
3. **UTF-8**: Strings are UTF-8, be careful with indexing
4. **Ownership**: Vectors own data, slices borrow
5. **Stack vs Heap**: Arrays on stack, vectors on heap
6. **Size Matters**: Fixed-size arrays, dynamic vectors

## Testing Your Understanding

Try modifying the examples:

1. Change numeric types and observe compiler errors
2. Cause intentional overflows with different methods
3. Try invalid UTF-8 operations (they won't compile!)
4. Convert between collection types
5. Experiment with string slicing
