// Chapter 6: Expressions
// Comprehensive examples of Rust's expression-based syntax

fn main() {
    println!("=== Chapter 6: Expressions ===\n");

    expression_language();
    precedence_and_associativity();
    blocks_and_semicolons();
    declarations();
    if_and_match();
    if_let();
    loops_demo();
    control_flow_in_loops();
    return_expressions();
    why_rust_has_loop();
    function_and_method_calls();
    fields_and_elements();
    reference_operators();
    arithmetic_bitwise_comparison_logical();
    assignment_examples();
    type_casts();
    closures_demo();
}

/// Rust is an expression language
fn expression_language() {
    println!("--- Expression Language ---");

    // Almost everything is an expression
    let x = {
        let y = 10;
        y + 5 // Expression evaluates to 15
    };
    println!("Block expression: {}", x);

    // if is an expression
    let number = 42;
    let description = if number > 0 {
        "positive"
    } else {
        "non-positive"
    };
    println!("if expression: {}", description);

    // match is an expression
    let coin_flip = true;
    let result = match coin_flip {
        true => "heads",
        false => "tails",
    };
    println!("match expression: {}", result);

    // Statements vs expressions
    let a = 5; // Statement (ends with ;)
    let b = (a + 1); // Expression in statement
    println!("Statement assigns: a={}, b={}", a, b);

    // Expression-oriented programming
    let status_code = 200;
    let message = match status_code {
        200 => "OK",
        404 => "Not Found",
        500 => "Server Error",
        _ => "Unknown",
    };
    println!("Status {}: {}", status_code, message);

    println!();
}

/// Precedence and associativity
fn precedence_and_associativity() {
    println!("--- Precedence and Associativity ---");

    // Arithmetic precedence (standard math rules)
    let result = 2 + 3 * 4; // 14, not 20
    println!("2 + 3 * 4 = {}", result);

    let result = (2 + 3) * 4; // 20, parentheses override
    println!("(2 + 3) * 4 = {}", result);

    // Left-to-right associativity for same precedence
    let result = 10 - 5 - 2; // (10 - 5) - 2 = 3
    println!("10 - 5 - 2 = {}", result);

    let result = 2 * 3 * 4; // (2 * 3) * 4 = 24
    println!("2 * 3 * 4 = {}", result);

    // Comparison operators don't chain like in Python
    // let valid = 1 < 2 < 3;  // ERROR: doesn't work
    let valid = 1 < 2 && 2 < 3; // Correct way
    println!("1 < 2 && 2 < 3: {}", valid);

    // Assignments in Rust return (), not the value
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    a = 5;
    b = a;
    c = b; // All get 5
    println!("After assigning 5 to all: a={}, b={}, c={}", a, b, c);

    // Bitwise operators
    let result = 0b1010 | 0b1100; // OR
    println!("0b1010 | 0b1100 = {:b}", result);

    let result = 0b1010 & 0b1100; // AND
    println!("0b1010 & 0b1100 = {:b}", result);

    let result = 0b1010 ^ 0b1100; // XOR
    println!("0b1010 ^ 0b1100 = {:b}", result);

    // Shift operators
    let result = 1 << 3; // Left shift (multiply by 8)
    println!("1 << 3 = {}", result);

    let result = 16 >> 2; // Right shift (divide by 4)
    println!("16 >> 2 = {}", result);

    println!();
}

/// Blocks and semicolons
fn blocks_and_semicolons() {
    println!("--- Blocks and Semicolons ---");

    // Block with final expression (no semicolon)
    let result = {
        let x = 10;
        let y = 20;
        x + y // Returns 30
    };
    println!("Block returns value: {}", result);

    // Block with semicolon returns ()
    let result = {
        let x = 10;
        let y = 20;
        x + y; // Semicolon makes it return ()
    };
    println!("Block with semicolon: {:?}", result);

    // Nested blocks
    let result = {
        let outer = 5;
        let temp = {
            let inner = 10;
            outer + inner
        };
        temp + 3 // 18
    };
    println!("Nested blocks: {}", result);

    // Empty block
    let unit = {};
    println!("Empty block: {:?}", unit);

    // Block creates scope
    let x = 1;
    {
        let x = 2; // Shadows outer x
        println!("Inner x: {}", x);
    }
    println!("Outer x: {}", x);

    // Multiple statements in block
    let area = {
        let width = 10;
        let height = 20;
        println!("Calculating area...");
        width * height // Final expression
    };
    println!("Area: {}", area);

    println!();
}

/// Declarations
fn declarations() {
    println!("--- Declarations ---");

    // let declarations
    let x = 42;
    println!("let x: {}", x);

    // Type annotations
    let x: i32 = 42;
    println!("let x: i32 = {}", x);

    // Mutable bindings
    let mut count = 0;
    count += 1;
    println!("mut count: {}", count);

    // Pattern matching in let
    let (a, b) = (1, 2);
    println!("Destructuring: a={}, b={}", a, b);

    let Point { x, y } = Point { x: 10, y: 20 };
    println!("Struct destructuring: x={}, y={}", x, y);

    // let with type inference
    let numbers = vec![1, 2, 3];
    println!("Inferred Vec: {:?}", numbers);

    // Shadowing
    let x = 5;
    let x = x + 1; // Shadows previous x
    let x = x * 2; // Shadows again
    println!("Shadowed x: {}", x);

    // Different types with shadowing
    let spaces = "   ";
    let spaces = spaces.len(); // Different type OK with shadowing
    println!("Spaces count: {}", spaces);

    // fn declarations
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    println!("Function result: {}", add(5, 3));

    println!();
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

/// if and match expressions
fn if_and_match() {
    println!("--- if and match ---");

    // if expression
    let number = 7;
    let result = if number < 5 {
        "small"
    } else if number < 10 {
        "medium"
    } else {
        "large"
    };
    println!("if expression result: {}", result);

    // if without else returns ()
    let x = 5;
    if x > 0 {
        println!("x is positive");
    }

    // All branches must return same type
    let score = 85;
    let grade = if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else {
        'F'
    };
    println!("Grade: {}", grade);

    // match expression
    let coin = "heads";
    let value = match coin {
        "heads" => 1,
        "tails" => 0,
        _ => panic!("Invalid coin"),
    };
    println!("Coin value: {}", value);

    // match with multiple patterns
    let number = 3;
    match number {
        1 | 2 => println!("one or two"),
        3 | 4 | 5 => println!("three to five"),
        _ => println!("something else"),
    }

    // match with ranges
    let age = 25;
    match age {
        0..=12 => println!("child"),
        13..=19 => println!("teenager"),
        20..=64 => println!("adult"),
        _ => println!("senior"),
    }

    // match with guards
    let number = Some(4);
    match number {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("greater or equal to five: {}", x),
        None => println!("no value"),
    }

    // match must be exhaustive
    let boolean = true;
    match boolean {
        true => println!("yes"),
        false => println!("no"),
    }

    // Destructuring in match
    let point = Point { x: 0, y: 7 };
    match point {
        Point { x: 0, y } => println!("On y-axis at {}", y),
        Point { x, y: 0 } => println!("On x-axis at {}", x),
        Point { x, y } => println!("At ({}, {})", x, y),
    }

    println!();
}

/// if let
fn if_let() {
    println!("--- if let ---");

    // if let for pattern matching
    let some_value = Some(7);

    if let Some(x) = some_value {
        println!("Got value: {}", x);
    }

    // Equivalent to:
    match some_value {
        Some(x) => println!("Match got value: {}", x),
        None => {}
    }

    // if let with else
    let optional = Some(42);
    if let Some(value) = optional {
        println!("Value is: {}", value);
    } else {
        println!("No value");
    }

    // if let for cleaner code
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Max is: {}", max);
    }

    // if let with tuple
    let pair = (2, -2);
    if let (0, y) = pair {
        println!("First is zero, second is {}", y);
    } else if let (x, 0) = pair {
        println!("Second is zero, first is {}", x);
    } else {
        println!("Neither is zero: ({}, {})", pair.0, pair.1);
    }

    // Multiple if let
    let a = Some(5);
    let b = Some(10);

    if let Some(x) = a {
        if let Some(y) = b {
            println!("a={}, b={}", x, y);
        }
    }

    println!();
}

/// Loops
fn loops_demo() {
    println!("--- Loops ---");

    // while loop
    let mut count = 0;
    while count < 3 {
        println!("while: count = {}", count);
        count += 1;
    }

    // while let
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("while let: popped {}", top);
    }

    // for loop
    println!("for loop:");
    for i in 0..3 {
        println!("  i = {}", i);
    }

    // for with iterator
    let array = [10, 20, 30];
    for element in &array {
        println!("for element: {}", element);
    }

    // for with enumerate
    for (index, value) in array.iter().enumerate() {
        println!("array[{}] = {}", index, value);
    }

    // loop (infinite loop)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2; // loop can return a value
        }
    };
    println!("loop result: {}", result);

    // Nested loops
    'outer: for x in 0..3 {
        for y in 0..3 {
            if x == 1 && y == 1 {
                println!("Breaking outer at ({}, {})", x, y);
                break 'outer;
            }
        }
    }

    println!();
}

/// Control flow in loops
fn control_flow_in_loops() {
    println!("--- Control Flow in Loops ---");

    // break
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("Breaking at {}", count);
            break;
        }
        println!("Count: {}", count);
    }

    // continue
    println!("Even numbers only:");
    for i in 0..5 {
        if i % 2 != 0 {
            continue;
        }
        println!("  {}", i);
    }

    // break with value
    let mut sum = 0;
    let result = loop {
        sum += 1;
        if sum > 10 {
            break sum;
        }
    };
    println!("Loop returned: {}", result);

    // Loop labels
    let mut count = 0;
    'counting: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("  remaining = {}", remaining);
            if remaining == 9 {
                break; // Breaks inner loop
            }
            if count == 2 {
                break 'counting; // Breaks outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }

    // continue with labels
    'outer: for x in 0..3 {
        for y in 0..3 {
            if y == 1 {
                continue 'outer; // Continue outer loop
            }
            println!("({}, {})", x, y);
        }
    }

    println!();
}

/// return expressions
fn return_expressions() {
    println!("--- return Expressions ---");

    // Early return
    fn check_positive(n: i32) -> &'static str {
        if n <= 0 {
            return "not positive";
        }
        "positive"
    }

    println!("check_positive(5): {}", check_positive(5));
    println!("check_positive(-1): {}", check_positive(-1));

    // Multiple returns
    fn describe_number(n: i32) -> &'static str {
        if n < 0 {
            return "negative";
        }
        if n == 0 {
            return "zero";
        }
        "positive"
    }

    println!("describe_number(0): {}", describe_number(0));

    // return in loop
    fn find_even(numbers: &[i32]) -> Option<i32> {
        for &num in numbers {
            if num % 2 == 0 {
                return Some(num);
            }
        }
        None
    }

    let nums = [1, 3, 4, 7];
    println!("First even: {:?}", find_even(&nums));

    // Explicit return vs implicit
    fn add_explicit(a: i32, b: i32) -> i32 {
        return a + b;
    }

    fn add_implicit(a: i32, b: i32) -> i32 {
        a + b // No return keyword, no semicolon
    }

    println!(
        "Explicit: {}, Implicit: {}",
        add_explicit(2, 3),
        add_implicit(2, 3)
    );

    println!();
}

/// Why Rust has loop
fn why_rust_has_loop() {
    println!("--- Why Rust Has loop ---");

    // loop is more explicit than while true
    let mut attempts = 0;
    let result = loop {
        attempts += 1;
        if attempts > 3 {
            break attempts;
        }
    };
    println!("Took {} attempts", result);

    // loop is known to diverge (never returns normally)
    // The compiler knows this for control flow analysis
    let x = 5;
    let y = if x > 3 {
        loop {
            break 10;
        }
    } else {
        20
    };
    println!("y = {}", y);

    // loop with break value is more idiomatic
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop with break value: {}", result);

    // Infinite loops are explicit
    fn server_loop_example() {
        // In a real server:
        // loop {
        //     let request = receive_request();
        //     handle_request(request);
        // }
        println!("Server would loop forever");
    }
    server_loop_example();

    println!();
}

/// Function and method calls
fn function_and_method_calls() {
    println!("--- Function and Method Calls ---");

    // Function calls
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let sum = add(5, 3);
    println!("Function call: add(5, 3) = {}", sum);

    // Method calls
    let text = String::from("hello");
    let upper = text.to_uppercase();
    println!("Method call: {}.to_uppercase() = {}", text, upper);

    // Chained method calls
    let result = "  hello world  "
        .trim()
        .to_uppercase()
        .replace("WORLD", "RUST");
    println!("Chained methods: {}", result);

    // Methods with self
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("rect1 area: {}", rect1.area());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));

    // Associated functions (no self)
    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    let square = Rectangle::square(20);
    println!("square area: {}", square.area());

    // Generic function calls
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    println!();
}

/// Fields and elements
fn fields_and_elements() {
    println!("--- Fields and Elements ---");

    // Struct fields
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("person.name: {}", person.name);
    println!("person.age: {}", person.age);

    // Tuple fields
    let pair = (42, "hello");
    println!("pair.0: {}", pair.0);
    println!("pair.1: {}", pair.1);

    // Array elements
    let array = [1, 2, 3, 4, 5];
    println!("array[0]: {}", array[0]);
    println!("array[4]: {}", array[4]);

    // Vector elements
    let vec = vec![10, 20, 30];
    println!("vec[0]: {}", vec[0]);
    println!("vec[2]: {}", vec[2]);

    // Slice elements
    let slice = &array[1..4];
    println!("slice[0]: {}", slice[0]);
    println!("slice[1]: {}", slice[1]);

    // get method for safe access
    match vec.get(10) {
        Some(value) => println!("vec[10]: {}", value),
        None => println!("vec[10]: out of bounds"),
    }

    // Nested access
    struct Point {
        x: i32,
        y: i32,
    }

    struct Line {
        start: Point,
        end: Point,
    }

    let line = Line {
        start: Point { x: 0, y: 0 },
        end: Point { x: 10, y: 10 },
    };

    println!("line.start.x: {}", line.start.x);
    println!("line.end.y: {}", line.end.y);

    println!();
}

/// Reference operators
fn reference_operators() {
    println!("--- Reference Operators ---");

    // & creates a reference
    let x = 42;
    let r = &x;
    println!("x = {}, r = &x = {}", x, r);

    // &mut creates a mutable reference
    let mut y = 10;
    let m = &mut y;
    *m += 5;
    println!("After *m += 5, y = {}", y);

    // * dereferences
    let x = 5;
    let r = &x;
    let value = *r;
    println!("*r = {}", value);

    // Automatic dereferencing in method calls
    let s = String::from("hello");
    let r = &s;
    println!("r.len() = {} (auto-deref)", r.len());

    // Reference to reference
    let x = 42;
    let r1 = &x;
    let r2 = &r1;
    println!("**r2 = {}", **r2);

    // References in patterns
    let reference = &4;
    match reference {
        &val => println!("Got value {} via destructuring", val),
    }

    match *reference {
        val => println!("Got value {} via dereferencing", val),
    }

    // ref in patterns
    let value = 5;
    match value {
        ref r => println!("Got reference via ref: {}", r),
    }

    println!();
}

/// Arithmetic, bitwise, comparison, and logical operators
fn arithmetic_bitwise_comparison_logical() {
    println!("--- Operators ---");

    // Arithmetic
    println!("Arithmetic:");
    println!("  10 + 5 = {}", 10 + 5);
    println!("  10 - 5 = {}", 10 - 5);
    println!("  10 * 5 = {}", 10 * 5);
    println!("  10 / 5 = {}", 10 / 5);
    println!("  10 % 3 = {}", 10 % 3);
    println!("  -5 = {}", -5);

    // Bitwise
    println!("\nBitwise:");
    println!("  0b1010 & 0b1100 = {:b}", 0b1010 & 0b1100);
    println!("  0b1010 | 0b1100 = {:b}", 0b1010 | 0b1100);
    println!("  0b1010 ^ 0b1100 = {:b}", 0b1010 ^ 0b1100);
    println!("  !0b1010 (u8) = {:b}", !0b1010u8);
    println!("  1 << 3 = {}", 1 << 3);
    println!("  16 >> 2 = {}", 16 >> 2);

    // Comparison
    println!("\nComparison:");
    println!("  5 == 5: {}", 5 == 5);
    println!("  5 != 3: {}", 5 != 3);
    println!("  5 > 3: {}", 5 > 3);
    println!("  5 < 3: {}", 5 < 3);
    println!("  5 >= 5: {}", 5 >= 5);
    println!("  5 <= 3: {}", 5 <= 3);

    // Logical
    println!("\nLogical:");
    println!("  true && true: {}", true && true);
    println!("  true && false: {}", true && false);
    println!("  true || false: {}", true || false);
    println!("  false || false: {}", false || false);
    println!("  !true: {}", !true);

    // Short-circuit evaluation
    println!("\nShort-circuit:");
    fn expensive_true() -> bool {
        println!("  (expensive_true called)");
        true
    }

    fn expensive_false() -> bool {
        println!("  (expensive_false called)");
        false
    }

    println!("  true || expensive_true():");
    let _ = true || expensive_true(); // expensive_true not called

    println!("  false && expensive_false():");
    let _ = false && expensive_false(); // expensive_false not called

    // Compound assignment
    println!("\nCompound assignment:");
    let mut x = 10;
    x += 5;
    println!("  After x += 5: {}", x);
    x -= 3;
    println!("  After x -= 3: {}", x);
    x *= 2;
    println!("  After x *= 2: {}", x);
    x /= 4;
    println!("  After x /= 4: {}", x);
    x %= 5;
    println!("  After x %= 5: {}", x);

    println!();
}

/// Assignment
fn assignment_examples() {
    println!("--- Assignment ---");

    // Simple assignment
    let mut x = 5;
    println!("x = {}", x);

    x = 10;
    println!("After x = 10: {}", x);

    // Assignment returns ()
    let result = (x = 15);
    println!("Assignment returns: {:?}", result);
    println!("x is now: {}", x);

    // Destructuring assignment
    let mut a = 0;
    let mut b = 0;
    (a, b) = (1, 2);
    println!("After (a, b) = (1, 2): a={}, b={}", a, b);

    // Compound assignment
    let mut count = 0;
    count += 1;
    count += 1;
    println!("count after += twice: {}", count);

    // Assignment is not an expression that returns a value
    // This won't work: if (x = 5) { ... }
    // Must use: x = 5; if x == 5 { ... }

    // Multiple assignments (must be separate)
    let mut x = 1;
    let mut y = 2;
    let mut z = 3;

    x = 10;
    y = 10;
    z = 10;
    println!("After setting all to 10: x={}, y={}, z={}", x, y, z);

    println!();
}

/// Type casts
fn type_casts() {
    println!("--- Type Casts ---");

    // as for primitive types
    let x: i32 = 42;
    let y: i64 = x as i64;
    println!("i32 to i64: {} as i64 = {}", x, y);

    let f: f64 = x as f64;
    println!("i32 to f64: {} as f64 = {}", x, f);

    // Lossy conversion
    let large: i64 = 10000;
    let small: i16 = large as i16;
    println!("i64 to i16 (lossy): {} as i16 = {}", large, small);

    // Float to int (truncates)
    let pi = 3.14159;
    let int_pi = pi as i32;
    println!("f64 to i32: {} as i32 = {}", pi, int_pi);

    // Bool to int
    let t = true as i32;
    let f = false as i32;
    println!("true as i32 = {}, false as i32 = {}", t, f);

    // Char to int
    let c = 'A';
    let code = c as u32;
    println!("'A' as u32 = {}", code);

    // Int to char (unsafe if invalid)
    let code: u32 = 65;
    let ch = code as u8 as char;
    println!("{} as char = '{}'", code, ch);

    // Pointer casts (unsafe)
    let x = 42;
    let ptr = &x as *const i32;
    println!("Reference to pointer: {:?}", ptr);

    // Cast between numeric types
    let a: u8 = 255;
    let b: i8 = a as i8; // Wraps around
    println!("u8 255 as i8 = {}", b);

    // No automatic coercion
    let x: i32 = 100;
    // let y: i64 = x;  // ERROR: must use 'as'
    let y: i64 = x as i64; // Correct
    println!("Explicit cast required: {}", y);

    println!();
}

/// Closures
fn closures_demo() {
    println!("--- Closures ---");

    // Basic closure
    let add = |a, b| a + b;
    println!("Closure: add(2, 3) = {}", add(2, 3));

    // Closure with type annotations
    let multiply: fn(i32, i32) -> i32 = |a, b| a * b;
    println!("Typed closure: multiply(4, 5) = {}", multiply(4, 5));

    // Closure capturing environment
    let x = 10;
    let add_x = |a| a + x;
    println!("Capturing closure: add_x(5) = {}", add_x(5));

    // Closure with multiple lines
    let complex = |n: i32| {
        let doubled = n * 2;
        let squared = doubled * doubled;
        squared
    };
    println!("Multi-line closure: complex(3) = {}", complex(3));

    // Closure without parameters
    let greet = || println!("Hello from closure!");
    greet();

    // Mutable closure
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        counter
    };
    println!("increment(): {}", increment());
    println!("increment(): {}", increment());
    println!("counter is now: {}", counter);

    // Moving closure
    let text = String::from("hello");
    let consume = move || {
        println!("Moved: {}", text);
    };
    consume();
    // text is no longer available here

    // Closures as function parameters
    fn apply<F>(f: F, value: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(value)
    }

    let result = apply(|x| x * 2, 10);
    println!("apply with closure: {}", result);

    // Returning closures
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }

    let add_five = make_adder(5);
    println!("add_five(10) = {}", add_five(10));

    // Closure with iterator
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();
    println!("Sum of evens: {}", sum);

    println!();
}
