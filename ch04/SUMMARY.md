# Chapter 4 - Quick Reference

## Core Concepts

### Ownership Rules

1. Each value has exactly one owner
2. When owner goes out of scope, value is dropped
3. Ownership can be transferred (moved)

### Move Semantics

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 moved to s2, s1 now invalid
```

### Copy Types

```rust
let x = 42;
let y = x;  // x copied, both valid
```

## Quick Command Reference

```bash
# Run all examples
cargo run

# Run specific example
cargo run --example move_patterns
cargo run --example copy_trait
cargo run --example shared_ownership
```

## Common Patterns

### 1. Borrow, Don't Move

```rust
fn process(v: &Vec<i32>) { }  // Good
fn process(v: Vec<i32>) { }   // Only if consuming
```

### 2. Move Out of Collections

```rust
// Clone
let s = v[0].clone();

// Replace
let s = std::mem::replace(&mut v[0], String::new());

// Remove
let s = v.remove(0);

// Swap remove
let s = v.swap_remove(0);
```

### 3. Shared Ownership

```rust
// Single-threaded
use std::rc::Rc;
let data = Rc::new(value);

// Multi-threaded
use std::sync::Arc;
let data = Arc::new(value);
```

### 4. Interior Mutability

```rust
// Single-threaded
Rc<RefCell<T>>

// Multi-threaded
Arc<Mutex<T>>
```

## Type Cheat Sheet

| Type           | Copy? | Clone? | Use Case           |
| -------------- | ----- | ------ | ------------------ |
| `i32`, `f64`   | ✓     | ✓      | Numbers            |
| `bool`, `char` | ✓     | ✓      | Primitives         |
| `&T`           | ✓     | ✓      | References         |
| `String`       | ✗     | ✓      | Owned strings      |
| `Vec<T>`       | ✗     | ✓      | Dynamic arrays     |
| `Box<T>`       | ✗     | ✓\*    | Heap allocation    |
| `Rc<T>`        | ✗     | ✓      | Shared ownership   |
| `Arc<T>`       | ✗     | ✓      | Thread-safe shared |

\*if T: Clone

## Ownership Decision Tree

```
Need to use value after passing it?
├─ No → Move it
└─ Yes
   ├─ Need to modify?
   │  ├─ Yes → Pass &mut T
   │  └─ No → Pass &T
   └─ Multiple owners?
      ├─ Single thread → Rc<T>
      └─ Multiple threads → Arc<T>
```

## Common Errors & Solutions

### Error: Value used after move

```rust
// Problem
let s = String::from("hello");
let s2 = s;
println!("{}", s);  // ERROR

// Solution 1: Clone
let s2 = s.clone();

// Solution 2: Borrow
let s2 = &s;

// Solution 3: Use after last use of s
println!("{}", s);
let s2 = s;
```

### Error: Cannot move out of index

```rust
// Problem
let v = vec![String::from("a")];
let s = v[0];  // ERROR

// Solutions
let s = v[0].clone();           // Clone
let s = v.remove(0);            // Remove
let s = std::mem::take(&mut v[0]); // Take
```

### Error: Cannot borrow as mutable

```rust
// Problem
let v = vec![1, 2, 3];
v.push(4);  // ERROR: v not mutable

// Solution
let mut v = vec![1, 2, 3];
v.push(4);
```

## Memory Visualization

### Stack vs Heap

**Copy Type (Stack)**

```
let x = 5;     ┌───┐
let y = x;     │ 5 │  x (stack)
               ├───┤
               │ 5 │  y (stack, copied)
               └───┘
```

**Move (Heap)**

```
let s1 = String::from("hello");
               ┌────────┐
Stack:  s1 →   │ ptr    │ ───→  Heap: "hello"
               │ len: 5 │
               │ cap: 5 │
               └────────┘

let s2 = s1;   (s1 invalidated)
               ┌────────┐
Stack:  s2 →   │ ptr    │ ───→  Heap: "hello"
               │ len: 5 │
               │ cap: 5 │
               └────────┘
```

**Rc (Shared)**

```
let s = Rc::new(String::from("hello"));
               ┌────────┐
Stack:  s  →   │ ptr    │ ───→  Heap: ┌─────────┐
               └────────┘            │ count: 1│
                                     │ "hello" │
let s2 = Rc::clone(&s);              └─────────┘
               ┌────────┐                  ↑
Stack:  s2 →   │ ptr    │ ─────────────────┘
               └────────┘
```

## Performance Notes

| Operation    | Cost | Notes                  |
| ------------ | ---- | ---------------------- |
| Move         | O(1) | Just pointer update    |
| Copy (small) | O(1) | Single CPU instruction |
| Copy (large) | O(n) | Copies all bytes       |
| Clone String | O(n) | Allocates + copies     |
| Rc::clone    | O(1) | Increments counter     |
| Arc::clone   | O(1) | Atomic increment       |

## When to Use What

**Use references (&T, &mut T):**

- Don't need ownership
- Temporary access
- Most function parameters

**Use moves:**

- Transferring ownership
- Consuming values
- Builder patterns

**Use Copy:**

- Small, simple types
- No heap allocation
- Cheap to duplicate

**Use Clone:**

- Need independent copy
- Explicit cost acceptable
- Deep copying needed

**Use Rc:**

- Single-threaded
- Multiple owners
- Read-only sharing

**Use Arc:**

- Multi-threaded
- Multiple owners
- Thread-safe sharing

## Example Index

| File                  | Topics Covered                                |
| --------------------- | --------------------------------------------- |
| `main.rs`             | All core concepts                             |
| `move_patterns.rs`    | Advanced move patterns, borrowing strategies  |
| `copy_trait.rs`       | Copy implementation, Copy vs Clone            |
| `shared_ownership.rs` | Rc, Arc, Weak references, interior mutability |
