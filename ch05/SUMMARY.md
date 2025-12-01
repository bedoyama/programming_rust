# Chapter 5: References - Quick Summary

## The Big Picture

**References = Non-Owning Pointers That Are Always Valid**

```rust
let x = 10;
let r = &x;      // Borrow x (immutable)
let m = &mut x;  // Borrow x (mutable)
```

## The Golden Rule

**You can have EITHER:**

- Multiple immutable references (`&T`), OR
- One mutable reference (`&mut T`)

**NEVER both at the same time!**

## Reference Types

| Type      | Symbol   | Allows       | Count | Example           |
| --------- | -------- | ------------ | ----- | ----------------- |
| Immutable | `&T`     | Read only    | Many  | `let r = &x;`     |
| Mutable   | `&mut T` | Read + Write | One   | `let m = &mut x;` |

## Core Syntax

```rust
// Creating references
let x = 5;
let r = &x;           // Immutable reference
let m = &mut x;       // Mutable reference (x must be mut)

// Dereferencing
println!("{}", *r);   // Get value through reference
*m = 10;              // Set value through mutable reference

// Automatic dereferencing (method calls)
let s = String::from("hello");
let r = &s;
r.len();              // Compiler adds * automatically
```

## Borrowing Rules Cheat Sheet

### ✓ Valid Patterns

```rust
// Multiple immutable borrows
let r1 = &data;
let r2 = &data;
let r3 = &data;

// Sequential mutable borrows
{
    let m1 = &mut data;
    m1.modify();
} // m1 ends
{
    let m2 = &mut data;
    m2.modify();
}

// Non-lexical lifetimes (NLL)
let r = &data;
println!("{:?}", r);
// r no longer used here
data.push(4);  // OK!
```

### ✗ Invalid Patterns

```rust
// Mutable + immutable
let r = &data;
data.push(4);        // ERROR: can't mutate while borrowed
println!("{:?}", r);

// Multiple mutable borrows
let m1 = &mut data;
let m2 = &mut data;  // ERROR: already borrowed

// Dangling reference
let r;
{
    let x = 5;
    r = &x;          // ERROR: x will be dropped
}
println!("{}", r);   // ERROR: r would dangle
```

## Lifetime Syntax

### Function Signatures

```rust
// Input and output have same lifetime
fn first<'a>(s: &'a str) -> &'a str

// Multiple lifetimes
fn select<'a, 'b>(x: &'a str, y: &'b str) -> &'a str

// Lifetime elision (compiler infers)
fn first(s: &str) -> &str  // Same as above
```

### Structs with References

```rust
struct Excerpt<'a> {
    text: &'a str,
}

// Usage
let novel = String::from("Once upon a time...");
let excerpt = Excerpt { text: &novel };
```

### The 'static Lifetime

```rust
let s: &'static str = "I live forever";
static GLOBAL: i32 = 42;
```

## Lifetime Elision Rules

The compiler automatically infers lifetimes in these cases:

1. **Each input reference gets its own lifetime**

   ```rust
   fn foo(x: &i32, y: &i32)
   // Becomes: fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
   ```

2. **If exactly one input lifetime, it's assigned to all outputs**

   ```rust
   fn foo(x: &i32) -> &i32
   // Becomes: fn foo<'a>(x: &'a i32) -> &'a i32
   ```

3. **If `&self` or `&mut self`, its lifetime is assigned to all outputs**
   ```rust
   impl Struct {
       fn method(&self) -> &i32
       // Becomes: fn method<'a>(&'a self) -> &'a i32
   }
   ```

## Common Patterns

### Pattern 1: Borrow for Reading

```rust
fn calculate(data: &Vec<i32>) -> i32 {
    data.iter().sum()
}

let v = vec![1, 2, 3];
let sum = calculate(&v);  // v still valid
```

### Pattern 2: Borrow for Mutation

```rust
fn double(nums: &mut [i32]) {
    for n in nums {
        *n *= 2;
    }
}

let mut v = vec![1, 2, 3];
double(&mut v);  // v is [2, 4, 6]
```

### Pattern 3: Returning References

```rust
// From input
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// From struct field
impl Person {
    fn name(&self) -> &str {
        &self.name
    }
}
```

### Pattern 4: Borrow Splitting

```rust
let mut arr = [1, 2, 3, 4, 5];
let (left, right) = arr.split_at_mut(2);

left[0] = 10;   // Modify first half
right[0] = 30;  // Modify second half
```

### Pattern 5: Builder Pattern

```rust
impl Builder {
    fn add(&mut self, val: i32) -> &mut Self {
        self.data.push(val);
        self  // Return mut ref for chaining
    }
}

builder.add(1).add(2).add(3);
```

## Decision Guide

### Reference vs Ownership?

```
Need to own the data? → Take by value (T)
Just need to read? → &T
Need to modify? → &mut T
```

### When to Clone vs Borrow?

```rust
// DON'T clone if borrowing works
fn process(s: &str) -> usize { s.len() }
process(&my_string);  // Not process(my_string.clone())

// DO clone when you need ownership
let owned = borrowed.clone();
spawn(move || use(owned));  // Move into thread
```

### Lifetime Annotations Needed?

```
Returning a reference?
├─ From one input → Compiler infers ✓
├─ From multiple inputs → Need annotation
├─ From &self/&mut self → Compiler infers ✓
└─ Static data → Use 'static
```

## Common Errors

### "cannot borrow as mutable..."

```rust
// Problem
let r = &data;
data.push(4);  // ERROR

// Solution: Separate scopes
{
    let r = &data;
    use(r);
}
data.push(4);  // OK
```

### "does not live long enough"

```rust
// Problem
let r;
{
    let x = 5;
    r = &x;  // ERROR: x dropped
}

// Solution: Extend x's lifetime
let x = 5;
let r = &x;  // OK
```

### "missing lifetime specifier"

```rust
// Problem
fn longest(x: &str, y: &str) -> &str  // ERROR

// Solution: Add lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

## Interior Mutability

When you need mutation through shared reference:

```rust
use std::cell::{Cell, RefCell};

// Cell: For Copy types
let x = Cell::new(5);
let r = &x;
r.set(10);  // Mutate through immutable ref

// RefCell: For any type (runtime checks)
let data = RefCell::new(vec![1, 2, 3]);
data.borrow_mut().push(4);  // Temporary mut borrow
```

⚠️ **Use sparingly!** Regular references are safer (compile-time checks).

## Performance Tips

### Zero-Cost Abstractions

```rust
// References are just pointers - no overhead
let r = &data;  // Same cost as C pointer
```

### Avoid Clones

```rust
// Bad: Copies entire vector
fn sum(v: Vec<i32>) -> i32 { v.iter().sum() }
let total = sum(data.clone());

// Good: Just borrows
fn sum(v: &[i32]) -> i32 { v.iter().sum() }
let total = sum(&data);
```

### Prefer Slices

```rust
// Less flexible
fn process(v: &Vec<String>) { ... }

// More flexible (works with Vec, array, etc.)
fn process(v: &[String]) { ... }
```

## Quick Checks

### Valid Code?

```rust
// ✓ Multiple readers
let r1 = &data;
let r2 = &data;

// ✓ One writer
let m = &mut data;

// ✗ Reader + writer
let r = &data;
let m = &mut data;  // ERROR

// ✓ NLL allows this
let r = &data;
println!("{:?}", r);
data.push(4);  // OK after r's last use
```

### Safe Lifetime?

```rust
// ✓ Reference outlives struct
let s = String::from("hello");
let excerpt = Excerpt { text: &s };

// ✗ Struct outlives reference
let excerpt;
{
    let s = String::from("hello");
    excerpt = Excerpt { text: &s };  // ERROR
}
```

## The Mental Model

Think of references as **temporary access passes**:

- **`&T`**: Read-only pass (can have many)
- **`&mut T`**: Read-write pass (exclusive access)
- **Lifetime**: How long the pass is valid

Rust enforces:

1. Passes must be returned before data is modified/destroyed
2. Only one write pass OR multiple read passes
3. Passes never outlive the data

## Key Takeaways

1. **References never own** - they just borrow
2. **Multiple readers XOR one writer** - prevents data races
3. **Always valid** - no null or dangling references
4. **Lifetimes** track how long references are valid
5. **Compiler infers** most lifetimes (elision)
6. **NLL** makes borrows end at last use (flexible)

---

## Examples to Run

```bash
# All topics
cargo run

# Specific examples
cargo run --example lifetimes
cargo run --example borrowing_rules
cargo run --example reference_safety
```

---

_Master references, master Rust!_ 🦀
