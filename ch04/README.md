# Chapter 4: Ownership and Moves

Comprehensive examples demonstrating Rust's ownership system, move semantics, and shared ownership patterns.

## Project Structure

```
ch04/
├── Cargo.toml
├── README.md (this file)
├── SUMMARY.md (quick reference)
├── src/
│   └── main.rs (comprehensive examples)
└── examples/
    ├── move_patterns.rs (advanced move patterns)
    ├── copy_trait.rs (Copy trait deep dive)
    └── shared_ownership.rs (Rc and Arc patterns)
```

## Running Examples

```bash
cd ch04

# Main program (all topics)
cargo run

# Individual examples
cargo run --example move_patterns
cargo run --example copy_trait
cargo run --example shared_ownership
```

## Topics Covered

### 1. Ownership Basics

**Core Principles:**

- Each value has exactly one owner
- When owner goes out of scope, value is dropped
- Ownership prevents double-free and use-after-free

```rust
let s1 = String::from("hello");  // s1 owns the String
// s1 dropped at end of scope
```

**Key Concepts:**

- Single ownership prevents memory errors
- Automatic cleanup via RAII (Resource Acquisition Is Initialization)
- No garbage collector needed

### 2. Moves

**Move Semantics:**

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 moved to s2
// s1 is now invalid
```

**When Moves Happen:**

- Variable assignment
- Function arguments
- Function returns
- Collection insertion
- Struct/tuple construction

**Why Moves?**

- Prevents double-free
- Makes ownership transfer explicit
- Zero-cost abstraction

### 3. More Operations That Move

**Common Move Operations:**

```rust
// Assignment
let s2 = s1;  // move

// Function call
takes_ownership(s);  // move

// Return from function
let s = gives_ownership();  // move

// Collection operations
vec.push(s);  // move
vec.extend(strings);  // move

// Pattern matching
match option {
    Some(value) => { /* value moved */ }
    None => {}
}
```

### 4. Moves and Control Flow

**Control Flow Patterns:**

```rust
// If expressions
let result = if condition {
    value  // value moved
} else {
    other_value
};

// Match (moving)
match option {
    Some(s) => println!("{}", s),  // s moved
    None => {}
}

// Match (borrowing)
match &option {
    Some(s) => println!("{}", s),  // s borrowed
    None => {}
}

// For loops (moving)
for item in vec { }  // vec moved

// For loops (borrowing)
for item in &vec { }  // vec borrowed
```

### 5. Moves and Indexed Content

**Problem:** Can't move out of indexed content

```rust
let v = vec![String::from("a"), String::from("b")];
// let s = v[0];  // ERROR: cannot move out of index
```

**Solutions:**

1. **Reference** - Borrow instead of move

```rust
let s_ref = &v[0];
```

2. **Clone** - Make a copy

```rust
let s = v[0].clone();
```

3. **Replace** - Swap with another value

```rust
let s = std::mem::replace(&mut v[0], String::new());
```

4. **Remove** - Take out and shift elements

```rust
let s = v.remove(0);
```

5. **Swap Remove** - Fast removal (no shift)

```rust
let s = v.swap_remove(0);
```

6. **Take from Option** - Replace with None

```rust
let s = option.take();
```

### 6. Copy Types: The Exception to Moves

**Copy Types:**

- Primitive types: integers, floats, bool, char
- References: `&T`
- Raw pointers: `*const T`, `*mut T`
- Function pointers
- Tuples/arrays of Copy types

**Copy Behavior:**

```rust
let x = 42;
let y = x;  // x is copied, not moved
println!("{} {}", x, y);  // both valid
```

**Implementing Copy:**

```rust
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
```

**Copy Requirements:**

- All fields must be Copy
- Cannot have Drop implementation
- Cannot own heap data (String, Vec, Box, etc.)

**Copy vs Clone:**

- Copy: Implicit, always cheap bitwise copy
- Clone: Explicit, can be expensive

### 7. Rc and Arc: Shared Ownership

**Rc (Reference Counted)** - Single-threaded

```rust
use std::rc::Rc;

let data = Rc::new(String::from("shared"));
let data2 = Rc::clone(&data);  // increment count
let data3 = Rc::clone(&data);

println!("Ref count: {}", Rc::strong_count(&data));
```

**Arc (Atomic Reference Counted)** - Thread-safe

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let data_clone = Arc::clone(&data);

thread::spawn(move || {
    println!("{:?}", data_clone);
});
```

**Weak References** - Break cycles

```rust
use std::rc::{Rc, Weak};

let strong = Rc::new(String::from("data"));
let weak: Weak<_> = Rc::downgrade(&strong);

// Upgrade weak to strong
if let Some(s) = weak.upgrade() {
    println!("{}", s);
}
```

**Interior Mutability:**

- `Rc<RefCell<T>>` - Single-threaded mutation
- `Arc<Mutex<T>>` - Multi-threaded mutation

## Key Patterns

### Pattern 1: Borrow Instead of Move

```rust
fn process(v: &Vec<i32>) {  // borrow
    // use v without taking ownership
}
```

### Pattern 2: Return Ownership

```rust
fn process(v: Vec<i32>) -> Vec<i32> {
    // process v
    v  // return ownership
}
```

### Pattern 3: Clone When Needed

```rust
let original = data.clone();
expensive_operation(data);  // moves data
// still have original
```

### Pattern 4: Use Rc/Arc for Shared Ownership

```rust
let shared = Rc::new(data);
let ref1 = Rc::clone(&shared);
let ref2 = Rc::clone(&shared);
// multiple owners
```

### Pattern 5: Move Closures

```rust
let data = vec![1, 2, 3];
let closure = move || {
    println!("{:?}", data);  // takes ownership
};
```

## Common Pitfalls

### 1. Use After Move

```rust
let s = String::from("hello");
let s2 = s;
// println!("{}", s);  // ERROR: s was moved
```

### 2. Moving Out of Indexed Content

```rust
let v = vec![String::from("a")];
// let s = v[0];  // ERROR
let s = v[0].clone();  // OK
```

### 3. Partial Moves

```rust
struct Person { name: String, age: u32 }
let p = Person { name: String::from("Alice"), age: 30 };
let name = p.name;  // partial move
// println!("{:?}", p);  // ERROR: partially moved
```

### 4. Moving in Loops

```rust
let v = vec![String::from("a")];
for s in v {  // v moved
    println!("{}", s);
}
// println!("{:?}", v);  // ERROR: v was moved
```

## Performance Considerations

### Move vs Copy

- **Move**: Transfer ownership (pointer update)
- **Copy**: Duplicate data (bitwise copy)
- Both are cheap for small types
- Move is always O(1), copy depends on size

### Rc vs Arc

- **Rc**: Faster, no atomic operations
- **Arc**: Thread-safe, atomic ref counting
- Choose Rc for single-threaded, Arc for multi-threaded

### Clone Costs

- Cheap: Integers, bools, Copy types
- Moderate: Short strings, small vecs
- Expensive: Large strings, large vecs, nested structures

## Best Practices

1. **Prefer borrowing** over moving when you don't need ownership
2. **Use references in collections** to avoid moving elements
3. **Implement Copy** for small, simple types
4. **Use Rc/Arc** sparingly - ownership is clearer
5. **Make moves explicit** with comments when non-obvious
6. **Avoid unnecessary clones** - they have runtime cost
7. **Use move closures** when transferring to threads
8. **Leverage the borrow checker** - it prevents bugs

## Additional Resources

- [The Rust Book - Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust by Example - Ownership](https://doc.rust-lang.org/rust-by-example/scope/move.html)
- [The Rustonomicon - Ownership](https://doc.rust-lang.org/nomicon/ownership.html)
