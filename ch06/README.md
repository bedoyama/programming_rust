# Chapter 6: Expressions

Comprehensive examples demonstrating Rust's expression-based language design and control flow mechanisms.

## Overview

Rust is fundamentally an expression-oriented language. This chapter covers:

- **Expression Language**: How Rust treats most constructs as expressions
- **Precedence and Associativity**: Operator ordering rules
- **Blocks and Semicolons**: Expression blocks and statement termination
- **Declarations**: `let`, `fn`, and other declarations
- **if and match**: Conditional expressions
- **if let**: Pattern matching shorthand
- **Loops**: `while`, `for`, `loop`, and `while let`
- **Control Flow**: `break`, `continue`, and loop labels
- **return Expressions**: Early returns and explicit returns
- **Why Rust Has loop**: The infinite loop construct
- **Function and Method Calls**: Calling functions and methods
- **Fields and Elements**: Accessing struct fields and array/slice elements
- **Reference Operators**: `&`, `&mut`, and `*`
- **Operators**: Arithmetic, bitwise, comparison, logical
- **Assignment**: Simple and compound assignment
- **Type Casts**: Using `as` for type conversion
- **Closures**: Anonymous functions that capture environment

## Key Concepts

### Everything is an Expression

In Rust, almost everything evaluates to a value:

```rust
// Blocks are expressions
let x = {
    let y = 10;
    y + 5  // Evaluates to 15
};

// if is an expression
let number = if x > 10 { "large" } else { "small" };

// match is an expression
let result = match x {
    0 => "zero",
    _ => "non-zero",
};
```

**Key Difference:**

- **Expression**: Evaluates to a value
- **Statement**: Performs an action (ends with `;`)

### Blocks and Semicolons

A block without a semicolon on the last line returns a value:

```rust
let value = {
    let x = 10;
    x + 5  // No semicolon - returns 15
};

let unit = {
    let x = 10;
    x + 5;  // Semicolon - returns ()
};
```

### Pattern Matching

Rust's `match` is powerful and must be exhaustive:

```rust
match value {
    0 => println!("zero"),
    1..=10 => println!("small"),
    _ => println!("large"),  // Catch-all required
}
```

## Running the Examples

### Main Program

Run the comprehensive examples covering all topics:

```bash
cd ch06
cargo run
```

This demonstrates:

- Expression language fundamentals
- Precedence and associativity
- Blocks and semicolons
- Declarations
- if and match expressions
- if let syntax
- All loop types
- Control flow (break, continue, labels)
- return expressions
- Function and method calls
- Field and element access
- Reference operators
- All operator types
- Assignment
- Type casts
- Basic closures

### Example Programs

#### 1. Control Flow (`control_flow.rs`)

Advanced control flow patterns:

```bash
cargo run --example control_flow
```

Topics covered:

- Advanced match patterns
- Match guards
- Advanced loops
- Loop labels
- Early returns

#### 2. Closures (`closures.rs`)

Comprehensive closure examples:

```bash
cargo run --example closures
```

Topics covered:

- Closure basics
- Capturing environment
- Closure traits (Fn, FnMut, FnOnce)
- Closures as parameters
- Returning closures
- Closures with iterators

#### 3. Operators (`operators.rs`)

All operator types:

```bash
cargo run --example operators
```

Topics covered:

- Arithmetic operators
- Bitwise operators
- Comparison operators
- Logical operators
- Compound assignment
- Operator precedence
- Operator overloading

## Core Principles

### 1. Expression-Oriented

Rust favors expressions over statements:

```rust
// Good: expression-based
let result = if condition { 10 } else { 20 };

// Less idiomatic: statement-based
let result;
if condition {
    result = 10;
} else {
    result = 20;
}
```

### 2. Explicit Control Flow

All control flow is explicit and clear:

```rust
// loop for infinite loops
loop {
    if done() { break; }
}

// for for iteration
for item in collection {
    process(item);
}

// while for conditions
while !done() {
    work();
}
```

### 3. Exhaustive Pattern Matching

`match` must cover all possibilities:

```rust
match option {
    Some(value) => use(value),
    None => handle_none(),  // Must handle all cases
}
```

## Common Patterns

### Pattern 1: Expression Blocks

```rust
let result = {
    let temp = expensive_computation();
    let processed = process(temp);
    processed  // Return value
};
```

### Pattern 2: if let for Simple Patterns

```rust
// Instead of:
match optional {
    Some(value) => println!("{}", value),
    None => {},
}

// Use:
if let Some(value) = optional {
    println!("{}", value);
}
```

### Pattern 3: loop with break Value

```rust
let result = loop {
    let value = get_next();
    if is_valid(value) {
        break value;  // Return value from loop
    }
};
```

### Pattern 4: Match Guards

```rust
match point {
    (x, y) if x == y => println!("diagonal"),
    (x, y) if x > y => println!("above diagonal"),
    (x, y) => println!("below diagonal"),
}
```

### Pattern 5: Closures with Iterators

```rust
let doubled: Vec<_> = numbers
    .iter()
    .filter(|&&x| x > 0)
    .map(|x| x * 2)
    .collect();
```

## Decision Trees

### Loop Type Selection

```
What kind of loop do you need?
├─ Iterate over collection → for
│   └─ for item in collection { ... }
│
├─ Condition-based → while
│   └─ while condition { ... }
│
├─ Pattern-based iteration → while let
│   └─ while let Some(x) = stack.pop() { ... }
│
└─ Infinite/break when done → loop
    └─ loop { if done { break; } }
```

### match vs if let

```
How many patterns to match?
├─ Just one (typically Some/None) → if let
│   └─ if let Some(x) = optional { ... }
│
└─ Multiple patterns → match
    └─ match value {
          Pattern1 => ...,
          Pattern2 => ...,
        }
```

### Closure Trait Selection

```
How does closure use captured variables?
├─ Just reads (immutable) → Fn
│   └─ Can be called multiple times
│
├─ Modifies (mutable) → FnMut
│   └─ Can be called multiple times
│
└─ Consumes (takes ownership) → FnOnce
    └─ Can only be called once
```

## Best Practices

### DO ✓

1. **Prefer expressions over statements**

   ```rust
   // Good
   let value = if condition { 1 } else { 2 };

   // Less idiomatic
   let value;
   if condition { value = 1; } else { value = 2; }
   ```

2. **Use if let for simple matches**

   ```rust
   // Good: clear intent
   if let Some(value) = optional {
       println!("{}", value);
   }

   // Verbose for simple case
   match optional {
       Some(value) => println!("{}", value),
       None => {},
   }
   ```

3. **Use loop for infinite loops**

   ```rust
   // Good: explicit intent
   loop {
       if done() { break; }
   }

   // Less clear
   while true {  // or while !false
       if done() { break; }
   }
   ```

4. **Let closures infer types**

   ```rust
   // Good: concise
   numbers.iter().map(|x| x * 2)

   // Verbose
   numbers.iter().map(|x: &i32| -> i32 { x * 2 })
   ```

5. **Use match guards for complex conditions**
   ```rust
   match pair {
       (x, y) if x == y => println!("diagonal"),
       (x, y) => println!("off-diagonal"),
   }
   ```

### DON'T ✗

1. **Don't use while true**

   ```rust
   // Bad: unclear
   while true {
       if done() { break; }
   }

   // Good: explicit
   loop {
       if done() { break; }
   }
   ```

2. **Don't ignore block return values unnecessarily**

   ```rust
   // Bad: throws away value
   {
       let result = expensive();
       result
   };  // Semicolon makes it ()

   // Good: use the value
   let value = {
       let result = expensive();
       result
   };  // No semicolon
   ```

3. **Don't use match when if let suffices**

   ```rust
   // Bad: verbose
   match optional {
       Some(x) => process(x),
       None => {},
   }

   // Good: concise
   if let Some(x) = optional {
       process(x);
   }
   ```

4. **Don't nest if let excessively**

   ```rust
   // Bad: deeply nested
   if let Some(a) = opt1 {
       if let Some(b) = opt2 {
           if let Some(c) = opt3 {
               // ...
           }
       }
   }

   // Good: use match or early returns
   match (opt1, opt2, opt3) {
       (Some(a), Some(b), Some(c)) => process(a, b, c),
       _ => return,
   }
   ```

## Common Errors and Solutions

### Error: "expected expression, found statement"

**Problem:**

```rust
let x = (let y = 5);  // ERROR: let is a statement
```

**Solution:** Statements can't be used as expressions:

```rust
let x = {
    let y = 5;
    y  // Return y
};
```

### Error: "if and else have incompatible types"

**Problem:**

```rust
let x = if condition { 5 } else { "hello" };  // ERROR: different types
```

**Solution:** All branches must return the same type:

```rust
let x = if condition { "five" } else { "hello" };
```

### Error: "non-exhaustive patterns"

**Problem:**

```rust
match value {
    1 => println!("one"),
    2 => println!("two"),
    // ERROR: missing patterns
}
```

**Solution:** Add catch-all pattern:

```rust
match value {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("other"),  // Catch-all
}
```

### Error: "loop might have zero iterations"

**Problem:** Trying to break with value from loop that might not run:

```rust
// This is OK:
let x = loop { break 5; };

// This would be problematic:
// while condition { break 5; }  // Might not run
```

**Solution:** Use `loop` when you need to break with a value:

```rust
let x = loop {
    if condition {
        break 42;
    }
};
```

## Performance Notes

### Zero-Cost Closures

Closures that don't capture are as fast as function pointers:

```rust
let add = |a, b| a + b;  // No capture, no overhead
```

### Inlining

Small functions and closures are usually inlined:

```rust
// Often inlined by compiler
numbers.iter().map(|x| x * 2)
```

### Pattern Matching Optimization

The compiler optimizes match into efficient code (jump tables, etc.):

```rust
// Compiled to efficient machine code
match value {
    1 => ...,
    2 => ...,
    3 => ...,
    _ => ...,
}
```

## Expression vs Statement Reference

| Construct         | Type       | Returns Value?   | Example                 |
| ----------------- | ---------- | ---------------- | ----------------------- |
| Block without `;` | Expression | Yes              | `{ x + 1 }`             |
| Block with `;`    | Statement  | No               | `{ x + 1; }`            |
| `if`              | Expression | Yes              | `if x { 1 } else { 2 }` |
| `match`           | Expression | Yes              | `match x { ... }`       |
| `loop`            | Expression | Yes (with break) | `loop { break 5; }`     |
| `for`             | Statement  | No               | `for x in 0..10 { }`    |
| `while`           | Statement  | No               | `while x < 10 { }`      |
| `let`             | Statement  | No               | `let x = 5;`            |
| `fn`              | Statement  | No               | `fn foo() { }`          |

## Advanced Topics

### Match Ergonomics

Automatic dereferencing in patterns:

```rust
let reference = &Some(5);
match reference {
    Some(value) => println!("{}", value),  // Auto-deref
    None => println!("none"),
}
```

### Closure Captures

Three ways closures capture:

```rust
let x = 5;

// By reference (immutable)
let f1 = || println!("{}", x);

// By mutable reference
let f2 = || { x += 1; };  // x must be mut

// By value (move)
let f3 = move || println!("{}", x);
```

### Loop Labels

Break or continue outer loops:

```rust
'outer: for x in 0..10 {
    for y in 0..10 {
        if x * y > 50 {
            break 'outer;  // Break outer loop
        }
    }
}
```

## Quick Reference

### Syntax Summary

| Syntax                     | Purpose              | Example                               |
| -------------------------- | -------------------- | ------------------------------------- |
| `{ expr }`                 | Block expression     | `{ let x = 5; x + 1 }`                |
| `if cond { } else { }`     | Conditional          | `if x > 0 { 1 } else { 0 }`           |
| `if let Pat = expr { }`    | Pattern match        | `if let Some(x) = opt { }`            |
| `match expr { ... }`       | Pattern matching     | `match x { 0 => ..., _ => ... }`      |
| `loop { }`                 | Infinite loop        | `loop { break; }`                     |
| `while cond { }`           | Conditional loop     | `while x < 10 { }`                    |
| `while let Pat = expr { }` | Pattern loop         | `while let Some(x) = iter.next() { }` |
| `for pat in expr { }`      | Iterator loop        | `for x in 0..10 { }`                  |
| `break`                    | Exit loop            | `break;`                              |
| `break expr`               | Exit with value      | `break 42;`                           |
| `continue`                 | Next iteration       | `continue;`                           |
| `return expr`              | Return from function | `return 5;`                           |
| `\|args\| expr`            | Closure              | `\|x\| x * 2`                         |

### Operator Precedence (Highest to Lowest)

1. Field access, method calls, indexing: `.`, `()`, `[]`
2. Unary: `-`, `!`, `*`, `&`, `&mut`
3. `as` casts
4. Multiplication, division: `*`, `/`, `%`
5. Addition, subtraction: `+`, `-`
6. Bit shifts: `<<`, `>>`
7. Bitwise AND: `&`
8. Bitwise XOR: `^`
9. Bitwise OR: `|`
10. Comparisons: `==`, `!=`, `<`, `>`, `<=`, `>=`
11. Logical AND: `&&`
12. Logical OR: `||`
13. Range: `..`, `..=`
14. Assignment: `=`, `+=`, etc.

## Further Reading

- The Rust Book: Chapter 3 (Control Flow) and Chapter 13 (Closures)
- Rust Reference: Expressions and Statements
- Rust by Example: Flow Control and Closures
- Understanding Ownership: How closures capture variables

---

_Programming Rust, Chapter 6_
