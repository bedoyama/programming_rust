# Chapter 6: Expressions - Quick Summary

## The Big Picture

**Rust is Expression-Oriented: Almost Everything Returns a Value**

```rust
let x = if condition { 1 } else { 2 };  // if is an expression
let y = match x { 1 => "one", _ => "other" };  // match is an expression
let z = { let a = 10; a + 5 };  // blocks are expressions
```

## Expression vs Statement

| Type           | Returns Value? | Example                                   |
| -------------- | -------------- | ----------------------------------------- |
| **Expression** | ✓ Yes          | `x + 1`, `if x { 1 } else { 2 }`, `{ x }` |
| **Statement**  | ✗ No           | `let x = 5;`, `x + 1;`, `{ x; }`          |

**Key Rule:** Adding semicolon (`;`) turns expression into statement!

```rust
let x = { 5 + 3 };   // Returns 8
let y = { 5 + 3; };  // Returns ()
```

## Control Flow Quick Reference

### if Expression

```rust
// Basic if
let result = if condition { value1 } else { value2 };

// Multi-branch
let grade = if score >= 90 {
    'A'
} else if score >= 80 {
    'B'
} else {
    'C'
};

// All branches must return same type!
```

### match Expression

```rust
// Basic match
match value {
    1 => "one",
    2 => "two",
    _ => "other",  // Catch-all required
}

// With guards
match num {
    x if x < 0 => "negative",
    x if x > 0 => "positive",
    _ => "zero",
}

// Destructuring
match point {
    (0, 0) => "origin",
    (x, 0) => format!("x-axis: {}", x),
    (0, y) => format!("y-axis: {}", y),
    (x, y) => format!("point: ({}, {})", x, y),
}
```

### if let (Shorthand)

```rust
// Instead of:
match optional {
    Some(x) => println!("{}", x),
    None => {},
}

// Use:
if let Some(x) = optional {
    println!("{}", x);
}

// With else:
if let Some(x) = optional {
    println!("Some: {}", x);
} else {
    println!("None");
}
```

## Loops

### Loop Types Comparison

| Loop        | When to Use          | Can Break with Value? | Example                               |
| ----------- | -------------------- | --------------------- | ------------------------------------- |
| `loop`      | Infinite/until break | ✓ Yes                 | `loop { break 5; }`                   |
| `while`     | Condition-based      | ✗ No                  | `while x < 10 { }`                    |
| `while let` | Pattern matching     | ✗ No                  | `while let Some(x) = iter.next() { }` |
| `for`       | Iterate collection   | ✗ No                  | `for x in 0..10 { }`                  |

### loop - Infinite Loop

```rust
// Basic infinite loop
loop {
    if done() {
        break;
    }
}

// Return value with break
let result = loop {
    let x = get_value();
    if valid(x) {
        break x;  // Returns x from loop
    }
};
```

### while - Conditional Loop

```rust
let mut count = 0;
while count < 10 {
    println!("{}", count);
    count += 1;
}
```

### while let - Pattern Loop

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### for - Iterator Loop

```rust
// Range
for i in 0..5 {
    println!("{}", i);
}

// Collection
for item in &collection {
    println!("{}", item);
}

// With index
for (i, item) in collection.iter().enumerate() {
    println!("{}: {}", i, item);
}

// Reverse
for i in (0..5).rev() {
    println!("{}", i);
}
```

## Loop Control

### break and continue

```rust
// break - exit loop
for i in 0..10 {
    if i == 5 {
        break;  // Exit loop
    }
}

// break with value (loop only)
let x = loop {
    if condition {
        break 42;  // Return 42
    }
};

// continue - skip to next iteration
for i in 0..10 {
    if i % 2 == 0 {
        continue;  // Skip evens
    }
    println!("{}", i);
}
```

### Loop Labels

```rust
// Break outer loop
'outer: for x in 0..10 {
    for y in 0..10 {
        if x * y > 50 {
            break 'outer;  // Break outer
        }
    }
}

// Continue outer loop
'outer: for x in 0..3 {
    for y in 0..3 {
        if y == 1 {
            continue 'outer;  // Continue outer
        }
        println!("({}, {})", x, y);
    }
}
```

## Closures

### Basics

```rust
// Simple closure
let add = |a, b| a + b;
println!("{}", add(2, 3));

// With type annotations
let multiply: fn(i32, i32) -> i32 = |a, b| a * b;

// Multi-line
let complex = |x| {
    let doubled = x * 2;
    doubled * doubled
};

// No parameters
let greet = || println!("Hello!");
```

### Capturing Environment

```rust
// Capture by reference (immutable)
let x = 10;
let print_x = || println!("{}", x);  // Borrows x
print_x();
println!("{}", x);  // x still accessible

// Capture by mutable reference
let mut count = 0;
let mut increment = || count += 1;  // Mutably borrows count
increment();

// Move closure
let text = String::from("hello");
let consume = move || println!("{}", text);  // Takes ownership
consume();
// text no longer accessible
```

### Closure Traits

| Trait    | Captures  | Can Call       | Use Case         |
| -------- | --------- | -------------- | ---------------- |
| `Fn`     | Immutable | Multiple times | Reading values   |
| `FnMut`  | Mutable   | Multiple times | Modifying values |
| `FnOnce` | By value  | Once           | Consuming values |

```rust
// Fn: can call multiple times
fn call_twice<F: Fn()>(f: F) {
    f();
    f();
}

// FnMut: can call multiple times, mutates
fn call_twice_mut<F: FnMut()>(mut f: F) {
    f();
    f();
}

// FnOnce: can call only once
fn call_once<F: FnOnce()>(f: F) {
    f();
}
```

### Closures with Iterators

```rust
let numbers = vec![1, 2, 3, 4, 5];

// map - transform each element
let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();

// filter - keep matching elements
let evens: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();

// fold - accumulate
let sum = numbers.iter().fold(0, |acc, x| acc + x);

// Chain operations
let result: Vec<_> = numbers
    .iter()
    .filter(|&&x| x > 2)
    .map(|x| x * 2)
    .collect();
```

## Operators

### Arithmetic

```rust
5 + 3   // Addition: 8
5 - 3   // Subtraction: 2
5 * 3   // Multiplication: 15
5 / 3   // Division: 1 (integer)
5 % 3   // Remainder: 2
-5      // Negation: -5
```

### Comparison

```rust
5 == 5  // Equal: true
5 != 3  // Not equal: true
5 > 3   // Greater: true
5 < 3   // Less: false
5 >= 5  // Greater or equal: true
5 <= 3  // Less or equal: false
```

### Logical

```rust
true && false   // AND: false
true || false   // OR: true
!true           // NOT: false

// Short-circuit evaluation
true || expensive()   // expensive() not called
false && expensive()  // expensive() not called
```

### Bitwise

```rust
0b1100 & 0b1010  // AND: 0b1000
0b1100 | 0b1010  // OR: 0b1110
0b1100 ^ 0b1010  // XOR: 0b0110
!0b1010          // NOT: inverts all bits
1 << 3           // Left shift: 8
16 >> 2          // Right shift: 4
```

### Compound Assignment

```rust
x += 5   // x = x + 5
x -= 3   // x = x - 3
x *= 2   // x = x * 2
x /= 4   // x = x / 4
x %= 5   // x = x % 5
x &= y   // x = x & y
x |= y   // x = x | y
x ^= y   // x = x ^ y
```

## Type Casts

```rust
// Numeric conversions
let x: i32 = 42;
let y: i64 = x as i64;
let f: f64 = x as f64;

// Lossy conversions
let large: i64 = 10000;
let small: i16 = large as i16;  // May truncate

// Float to int (truncates)
let pi = 3.14;
let int_pi = pi as i32;  // 3

// Bool to int
let t = true as i32;   // 1
let f = false as i32;  // 0

// Char to int
let c = 'A' as u32;  // 65
```

## Operator Precedence (High to Low)

1. **Field/Method/Index**: `.`, `()`, `[]`
2. **Unary**: `-`, `!`, `*`, `&`, `&mut`
3. **Cast**: `as`
4. **Multiply/Divide**: `*`, `/`, `%`
5. **Add/Subtract**: `+`, `-`
6. **Shift**: `<<`, `>>`
7. **Bitwise AND**: `&`
8. **Bitwise XOR**: `^`
9. **Bitwise OR**: `|`
10. **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
11. **Logical AND**: `&&`
12. **Logical OR**: `||`
13. **Range**: `..`, `..=`
14. **Assignment**: `=`, `+=`, etc.

## Common Patterns

### Pattern 1: Expression Blocks

```rust
let result = {
    let intermediate = expensive_computation();
    let processed = process(intermediate);
    processed  // Return value (no semicolon)
};
```

### Pattern 2: loop with break Value

```rust
let result = loop {
    let value = try_get_value();
    if is_valid(value) {
        break value;  // Return valid value
    }
};
```

### Pattern 3: Early Return Guards

```rust
fn process(value: Option<i32>) -> i32 {
    let val = match value {
        Some(v) => v,
        None => return 0,  // Early return
    };

    if val < 0 {
        return -1;  // Guard clause
    }

    val * 2
}
```

### Pattern 4: if let Chain

```rust
if let Some(x) = opt1 {
    if let Some(y) = opt2 {
        return x + y;
    }
}

// Better: use match
match (opt1, opt2) {
    (Some(x), Some(y)) => x + y,
    _ => 0,
}
```

### Pattern 5: Closure Capture

```rust
let threshold = 10;
let above_threshold: Vec<_> = numbers
    .iter()
    .filter(|&&x| x > threshold)  // Captures threshold
    .collect();
```

## Decision Guide

### Choose Loop Type

```
What kind of iteration?
├─ Infinite/break when done → loop
│   └─ loop { if done { break; } }
│
├─ Condition to continue → while
│   └─ while condition { }
│
├─ Pattern matching iteration → while let
│   └─ while let Some(x) = iter.next() { }
│
└─ Iterate over collection → for
    └─ for item in collection { }
```

### match vs if let

```
How many patterns?
├─ One pattern (usually Some/None) → if let
│   └─ if let Some(x) = opt { }
│
└─ Multiple patterns → match
    └─ match opt {
          Some(x) => ...,
          None => ...,
        }
```

### Closure Trait

```
How does closure use captures?
├─ Just reads → Fn
│   └─ || println!("{}", x)
│
├─ Modifies → FnMut
│   └─ || { x += 1; }
│
└─ Consumes → FnOnce
    └─ move || { consume(x); }
```

## Common Mistakes

### ✗ Forgetting Semicolon vs No Semicolon

```rust
// Wrong: wanted to return value
let x = {
    5 + 3;  // Semicolon makes this ()
};
println!("{}", x);  // Error: x is ()

// Right: no semicolon to return value
let x = {
    5 + 3  // No semicolon, returns 8
};
```

### ✗ Mismatched Branch Types

```rust
// Wrong: different types
let x = if condition { 5 } else { "hello" };

// Right: same types
let x = if condition { "five" } else { "hello" };
```

### ✗ Non-Exhaustive match

```rust
// Wrong: missing None case
match optional {
    Some(x) => println!("{}", x),
    // Error: non-exhaustive
}

// Right: handle all cases
match optional {
    Some(x) => println!("{}", x),
    None => println!("none"),
}
```

### ✗ Using while true

```rust
// Wrong: less clear
while true {
    if done() { break; }
}

// Right: explicit infinite loop
loop {
    if done() { break; }
}
```

## Key Takeaways

1. **Expression-oriented**: Most things return values
2. **Semicolons matter**: Change expressions to statements
3. **loop for infinite**: More explicit than `while true`
4. **match is exhaustive**: Must handle all cases
5. **if let for simplicity**: When you only care about one pattern
6. **Closures capture**: Can use variables from surrounding scope
7. **Type consistency**: All branches must return same type

---

## Examples to Run

```bash
# All topics
cargo run

# Specific examples
cargo run --example control_flow
cargo run --example closures
cargo run --example operators
```

---

_Everything is an expression!_ 🦀
