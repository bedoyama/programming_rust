// Advanced control flow patterns and expressions

fn main() {
    println!("=== Advanced Control Flow ===\n");

    match_patterns();
    match_guards();
    advanced_loops();
    loop_labels();
    early_returns();
}

/// Advanced match patterns
fn match_patterns() {
    println!("--- Match Patterns ---");

    // Matching literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    // Matching multiple patterns
    let x = 5;
    match x {
        1 | 2 | 3 => println!("small"),
        4 | 5 | 6 => println!("medium"),
        _ => println!("large"),
    }

    // Matching ranges
    let x = 15;
    match x {
        0..=10 => println!("0-10"),
        11..=20 => println!("11-20"),
        21..=30 => println!("21-30"),
        _ => println!("above 30"),
    }

    // Matching tuples
    let pair = (2, -2);
    match pair {
        (0, 0) => println!("origin"),
        (x, 0) => println!("x-axis at {}", x),
        (0, y) => println!("y-axis at {}", y),
        (x, y) if x == y => println!("diagonal at {}", x),
        (x, y) if x == -y => println!("anti-diagonal ({}, {})", x, y),
        (x, y) => println!("arbitrary point ({}, {})", x, y),
    }

    // Destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };
    match point {
        Point { x: 0, y: 0 } => println!("origin"),
        Point { x: 0, y } => println!("on y-axis at {}", y),
        Point { x, y: 0 } => println!("on x-axis at {}", x),
        Point { x, y } => println!("at ({}, {})", x, y),
    }

    // Matching enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
    }

    // Ignoring values with _
    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, _, _, _, last) => {
            println!("First: {}, Last: {}", first, last);
        }
    }

    // Ignoring remaining parts with ..
    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, .., last) => {
            println!("First: {}, Last: {}", first, last);
        }
    }

    // @ bindings
    let msg = Some(42);
    match msg {
        Some(n @ 0..=10) => println!("Small number: {}", n),
        Some(n @ 11..=20) => println!("Medium number: {}", n),
        Some(n) => println!("Large number: {}", n),
        None => println!("No number"),
    }

    println!();
}

/// Match guards
fn match_guards() {
    println!("--- Match Guards ---");

    // Simple guard
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("Less than 5: {}", x),
        Some(x) => println!("Greater or equal to 5: {}", x),
        None => println!("None"),
    }

    // Multiple conditions
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("Same: {}", x),
        (x, y) if x + y == 0 => println!("Opposite: {} and {}", x, y),
        (x, y) if x > y => println!("{} > {}", x, y),
        (x, y) => println!("{} <= {}", x, y),
    }

    // Using outer variables in guards
    let threshold = 10;
    let value = Some(15);

    match value {
        Some(x) if x > threshold => println!("{} exceeds threshold", x),
        Some(x) => println!("{} within threshold", x),
        None => println!("No value"),
    }

    // Complex conditions
    let point = (3, 4);
    match point {
        (x, y) if x * x + y * y <= 25 => {
            println!("Inside circle: ({}, {})", x, y);
        }
        (x, y) => {
            println!("Outside circle: ({}, {})", x, y);
        }
    }

    // Guard with destructuring
    enum OptionalPair {
        Some(i32, i32),
        None,
    }

    let opt = OptionalPair::Some(3, 5);
    match opt {
        OptionalPair::Some(x, y) if x > 0 && y > 0 => {
            println!("Both positive: ({}, {})", x, y);
        }
        OptionalPair::Some(x, y) => {
            println!("Not both positive: ({}, {})", x, y);
        }
        OptionalPair::None => println!("None"),
    }

    println!();
}

/// Advanced loops
fn advanced_loops() {
    println!("--- Advanced Loops ---");

    // Loop with break value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop result: {}", result);

    // while let pattern
    let mut stack = vec![1, 2, 3, 4, 5];
    println!("Popping from stack:");
    while let Some(top) = stack.pop() {
        println!("  {}", top);
    }

    // for with pattern matching
    let v = vec![(1, 2), (3, 4), (5, 6)];
    println!("Destructuring in for:");
    for (x, y) in v {
        println!("  x: {}, y: {}", x, y);
    }

    // for with enumerate
    let words = vec!["first", "second", "third"];
    for (i, word) in words.iter().enumerate() {
        println!("  {}: {}", i, word);
    }

    // Reverse iteration
    println!("Reverse:");
    for i in (0..5).rev() {
        println!("  {}", i);
    }

    // Step by
    println!("Step by 2:");
    for i in (0..10).step_by(2) {
        println!("  {}", i);
    }

    // Infinite iterator with take
    println!("Infinite with take:");
    for i in (0..).take(5) {
        println!("  {}", i);
    }

    // Combining iterators
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    println!("Zipped:");
    for (x, y) in a.iter().zip(b.iter()) {
        println!("  ({}, {})", x, y);
    }

    println!();
}

/// Loop labels
fn loop_labels() {
    println!("--- Loop Labels ---");

    // Break to outer loop
    println!("Breaking outer:");
    'outer: for x in 0..3 {
        for y in 0..3 {
            if x == 1 && y == 1 {
                println!("  Breaking at ({}, {})", x, y);
                break 'outer;
            }
            println!("  ({}, {})", x, y);
        }
    }

    // Continue outer loop
    println!("\nContinuing outer:");
    'outer: for x in 0..3 {
        for y in 0..3 {
            if y == 1 {
                continue 'outer;
            }
            println!("  ({}, {})", x, y);
        }
    }

    // Multiple nested loops
    println!("\nMultiple levels:");
    'outer: loop {
        println!("  Outer");
        'middle: loop {
            println!("    Middle");
            'inner: loop {
                println!("      Inner");
                break 'outer;
            }
        }
    }
    println!("  Done");

    // Break with value and label
    let result = 'search: loop {
        for x in 0..10 {
            for y in 0..10 {
                if x * y > 50 {
                    break 'search (x, y);
                }
            }
        }
        break (0, 0);
    };
    println!("\nFound: {:?}", result);

    println!();
}

/// Early returns
fn early_returns() {
    println!("--- Early Returns ---");

    // Guard clauses
    fn process(value: Option<i32>) -> i32 {
        let val = match value {
            Some(v) => v,
            None => return 0,
        };

        if val < 0 {
            return -1;
        }

        if val > 100 {
            return 100;
        }

        val * 2
    }

    println!("process(Some(50)): {}", process(Some(50)));
    println!("process(Some(-5)): {}", process(Some(-5)));
    println!("process(Some(150)): {}", process(Some(150)));
    println!("process(None): {}", process(None));

    // Early return in loop
    fn find_divisor(n: i32) -> Option<i32> {
        for i in 2..n {
            if n % i == 0 {
                return Some(i);
            }
        }
        None
    }

    println!("find_divisor(15): {:?}", find_divisor(15));
    println!("find_divisor(17): {:?}", find_divisor(17));

    // Multiple exit points
    fn categorize(n: i32) -> &'static str {
        if n < 0 {
            return "negative";
        }

        if n == 0 {
            return "zero";
        }

        if n < 10 {
            return "small";
        }

        if n < 100 {
            return "medium";
        }

        "large"
    }

    println!("categorize(-5): {}", categorize(-5));
    println!("categorize(0): {}", categorize(0));
    println!("categorize(5): {}", categorize(5));
    println!("categorize(50): {}", categorize(50));
    println!("categorize(500): {}", categorize(500));

    // ? operator for early return
    fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
        let num: i32 = s.parse()?; // Returns early on error
        Ok(num * 2)
    }

    println!("parse_and_double(\"21\"): {:?}", parse_and_double("21"));
    println!("parse_and_double(\"abc\"): {:?}", parse_and_double("abc"));

    println!();
}
