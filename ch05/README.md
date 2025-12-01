# Chapter 5: References

Comprehensive examples demonstrating Rust's reference system, borrowing rules, and lifetime management.

## Overview

References are Rust's way of allowing multiple parts of code to access data without taking ownership. This chapter covers:

- **References to Values**: Creating and using references
- **Working with References**: Dereferencing and comparison
- **Reference Safety**: Rust's guarantees about references
- **Borrowing**: Rules for borrowing data
- **Lifetimes**: Ensuring references remain valid
- **Sharing vs Mutation**: The fundamental trade-off

## Key Concepts

### References Basics

A reference is a pointer that borrows a value without taking ownership:

```rust
let x = 10;
let r = &x;  // r is a reference to x
println!("{}", *r);  // Dereference to get value
```

**Key Properties:**

- References never own the data they point to
- References are always valid (never null)
- Multiple immutable references are allowed
- Only one mutable reference at a time

### Borrowing Rules

Rust enforces these rules at compile time:

1. **At any time**, you can have EITHER:

   - One mutable reference, OR
   - Any number of immutable references

2. **References must always be valid** (no dangling references)

```rust
let mut data = vec![1, 2, 3];

// Multiple immutable borrows OK
let r1 = &data;
let r2 = &data;
println!("{:?} {:?}", r1, r2);

// One mutable borrow
let m = &mut data;
m.push(4);
```

### Lifetimes

Lifetimes ensure references don't outlive the data they point to:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The `'a` lifetime parameter says: "The returned reference will be valid as long as both input references are valid."

## Running the Examples

### Main Program

Run the comprehensive examples covering all topics:

```bash
cd ch05
cargo run
```

This demonstrates:

- References to values
- Working with references (dereferencing, comparison)
- Rust vs C++ references
- Assigning and comparing references
- Borrowing rules
- Reference safety
- Lifetimes
- Sharing vs mutation

### Example Programs

#### 1. Lifetimes (`lifetimes.rs`)

Advanced lifetime examples:

```bash
cargo run --example lifetimes
```

Topics covered:

- Explicit lifetime annotations
- Lifetime elision rules
- Lifetime constraints and bounds
- Struct lifetimes
- The `'static` lifetime

#### 2. Borrowing Rules (`borrowing_rules.rs`)

Borrowing patterns and rules:

```bash
cargo run --example borrowing_rules
```

Topics covered:

- Immutable borrowing patterns
- Mutable borrowing patterns
- Borrow splitting
- Non-lexical lifetimes (NLL)
- Reborrowing
- Common borrowing patterns

#### 3. Reference Safety (`reference_safety.rs`)

Safety guarantees and pitfalls:

```bash
cargo run --example reference_safety
```

Topics covered:

- Dangling reference prevention
- Common lifetime errors
- Borrowing conflicts
- Interior mutability (Cell, RefCell)
- Unsafe reference operations

## Core Principles

### 1. Shared XOR Mutable

You can have multiple readers OR one writer, never both:

```rust
let mut data = vec![1, 2, 3];

// Multiple readers
let r1 = &data;
let r2 = &data;  // OK

// One writer
let m = &mut data;  // OK (but r1, r2 must be gone)
```

This prevents data races at compile time!

### 2. References Must Be Valid

Rust ensures references never outlive their data:

```rust
// This won't compile:
// let r;
// {
//     let x = 5;
//     r = &x;  // ERROR: x will be dropped
// }
// println!("{}", r);  // ERROR: r would be dangling
```

### 3. Lifetime Elision

The compiler can often infer lifetimes:

```rust
// These are equivalent:
fn first_word(s: &str) -> &str { ... }
fn first_word<'a>(s: &'a str) -> &'a str { ... }
```

Three elision rules let you omit lifetime annotations in common cases.

## Common Patterns

### Pattern 1: Borrowing for Read

```rust
fn calculate_length(s: &String) -> usize {
    s.len()  // Borrow, don't take ownership
}

let text = String::from("hello");
let len = calculate_length(&text);
println!("{} is {} chars", text, len);  // text still valid
```

### Pattern 2: Borrowing for Mutation

```rust
fn append_world(s: &mut String) {
    s.push_str(" world");
}

let mut text = String::from("hello");
append_world(&mut text);
println!("{}", text);  // "hello world"
```

### Pattern 3: Returning References

```rust
// Return reference to input
fn first_element<T>(slice: &[T]) -> Option<&T> {
    slice.first()
}

// Return reference to struct field
struct Person { name: String }
impl Person {
    fn name(&self) -> &str {
        &self.name
    }
}
```

### Pattern 4: Structs with Lifetimes

```rust
struct Excerpt<'a> {
    text: &'a str,
}

let novel = String::from("Call me Ishmael...");
let first_sentence = novel.split('.').next().unwrap();
let excerpt = Excerpt { text: first_sentence };
```

### Pattern 5: Multiple Lifetimes

```rust
struct Context<'s, 't> {
    source: &'s str,
    target: &'t str,
}

fn process<'s, 't>(ctx: &Context<'s, 't>) -> &'s str {
    ctx.source  // Return has 's lifetime
}
```

## Decision Trees

### Should I Use a Reference?

```
Do you need to modify the data?
├─ Yes → Use &mut T
│   └─ Can you have exclusive access?
│       ├─ Yes → &mut T is perfect
│       └─ No → Consider RefCell<T> or Mutex<T>
│
└─ No → Use &T
    └─ Do multiple parts need access?
        ├─ Yes → &T allows multiple readers
        └─ No → Consider taking ownership (T)
```

### Reference vs Ownership?

```
Does the function need to own the data?
├─ Yes (needs to store it, move it, or drop it)
│   └─ Take by value: fn foo(data: T)
│
├─ No, just needs to read it
│   └─ Borrow immutably: fn foo(data: &T)
│
└─ No, but needs to modify it
    └─ Borrow mutably: fn foo(data: &mut T)
```

### When Do I Need Lifetime Annotations?

```
Are you returning a reference?
├─ Yes
│   ├─ From a function parameter?
│   │   └─ Usually need lifetime annotation
│   ├─ To static data?
│   │   └─ Use 'static
│   └─ To self in a method?
│       └─ Usually elided automatically
│
└─ No
    └─ Probably don't need annotations (elision)
```

## Best Practices

### DO ✓

1. **Prefer borrowing over ownership** when possible

   ```rust
   fn process(data: &[i32]) { ... }  // Good: borrows
   ```

2. **Use slices for flexibility**

   ```rust
   fn print_items(items: &[String]) { ... }  // Works with Vec, arrays, etc.
   ```

3. **Let NLL do its job**

   ```rust
   let r = &data;
   println!("{:?}", r);
   // r's lifetime ends here
   data.push(4);  // OK!
   ```

4. **Use lifetime elision**

   ```rust
   // Write this:
   fn first(s: &str) -> &str { ... }

   // Not this:
   fn first<'a>(s: &'a str) -> &'a str { ... }
   ```

5. **Split borrows when possible**
   ```rust
   let (left, right) = data.split_at_mut(mid);
   process_left(left);
   process_right(right);
   ```

### DON'T ✗

1. **Don't fight the borrow checker**

   ```rust
   // Bad: trying to have mutable and immutable at once
   // let r = &data;
   // data.push(4);  // Won't compile
   // println!("{:?}", r);

   // Good: separate borrows
   {
       let r = &data;
       println!("{:?}", r);
   }
   data.push(4);
   ```

2. **Don't return references to local variables**

   ```rust
   // Bad:
   // fn bad() -> &String {
   //     let s = String::from("hello");
   //     &s  // ERROR: s will be dropped
   // }

   // Good:
   fn good() -> String {
       String::from("hello")  // Return owned value
   }
   ```

3. **Don't use RefCell unless necessary**

   ```rust
   // Prefer compile-time checking:
   fn process(data: &mut Vec<i32>) { ... }

   // Only use RefCell when you must:
   struct Node {
       children: RefCell<Vec<Node>>,  // For graph structures
   }
   ```

4. **Don't clone unnecessarily**

   ```rust
   // Bad:
   fn process(s: String) -> usize {
       s.len()
   }
   let len = process(text.clone());  // Unnecessary clone

   // Good:
   fn process(s: &str) -> usize {
       s.len()
   }
   let len = process(&text);  // Just borrow
   ```

## Common Errors and Solutions

### Error: "cannot borrow as mutable because it is also borrowed as immutable"

**Problem:**

```rust
let r = &data;
data.push(4);  // ERROR
println!("{:?}", r);
```

**Solution:** Ensure immutable borrow ends before mutation:

```rust
let r = &data;
println!("{:?}", r);  // Last use of r
// r's lifetime ends here
data.push(4);  // OK now
```

### Error: "cannot return reference to local variable"

**Problem:**

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // ERROR: s will be dropped
}
```

**Solution:** Return owned value:

```rust
fn not_dangle() -> String {
    String::from("hello")  // Ownership moves out
}
```

### Error: "missing lifetime specifier"

**Problem:**

```rust
fn longest(x: &str, y: &str) -> &str {  // ERROR: which lifetime?
    if x.len() > y.len() { x } else { y }
}
```

**Solution:** Add explicit lifetime:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

## Performance Notes

### References Are Zero-Cost

References compile to simple pointers - no runtime overhead:

```rust
let x = 42;
let r = &x;  // Just a pointer, zero cost
```

### Avoid Unnecessary Copies

Borrowing prevents expensive copies:

```rust
// Bad: copies entire vector
fn sum_bad(v: Vec<i32>) -> i32 {
    v.iter().sum()
}

// Good: just borrows
fn sum_good(v: &[i32]) -> i32 {
    v.iter().sum()
}
```

### RefCell Has Runtime Cost

`RefCell` checks borrows at runtime:

```rust
let data = RefCell::new(vec![1, 2, 3]);
let r = data.borrow();  // Runtime check
```

Use regular references when possible for zero-cost compile-time checks.

## Advanced Topics

### Lifetime Bounds

```rust
fn print_ref<'a, T>(t: &'a T)
where
    T: std::fmt::Debug + 'a
{
    println!("{:?}", t);
}
```

### Static Lifetime

```rust
let s: &'static str = "I live forever";

static GLOBAL: &str = "Global data";
```

### Higher-Rank Trait Bounds (HRTBs)

```rust
fn call_with_one<F>(f: F) -> usize
where
    F: for<'a> Fn(&'a i32) -> usize
{
    f(&1)
}
```

## Quick Reference

### Syntax Summary

| Syntax    | Meaning                             |
| --------- | ----------------------------------- |
| `&T`      | Immutable reference to T            |
| `&mut T`  | Mutable reference to T              |
| `*r`      | Dereference r                       |
| `&x`      | Borrow x                            |
| `&mut x`  | Borrow x mutably                    |
| `'a`      | Lifetime parameter                  |
| `<'a>`    | Generic lifetime                    |
| `&'a T`   | Reference with lifetime 'a          |
| `'static` | Special lifetime for entire program |

### Key Rules

1. References must always be valid
2. Multiple `&T` OR one `&mut T`, never both
3. References cannot outlive their data
4. Mutable references have exclusive access
5. Immutable references can be shared

## Further Reading

- The Rust Book: Chapter 4 (Ownership) and Chapter 10 (Lifetimes)
- Rust Reference: Lifetimes and References
- Too Many Lists: Learning Rust through linked lists
- Rustonomicon: Advanced unsafe Rust and lifetimes

---

_Programming Rust, Chapter 5_
