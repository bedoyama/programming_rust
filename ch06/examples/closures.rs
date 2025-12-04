// Advanced closure patterns and usage

fn main() {
    println!("=== Advanced Closures ===\n");
    
    closure_basics();
    capturing_environment();
    closure_traits();
    closures_as_parameters();
    returning_closures();
    closures_with_iterators();
}

/// Closure basics
fn closure_basics() {
    println!("--- Closure Basics ---");
    
    // Simple closure
    let add_one = |x| x + 1;
    println!("add_one(5): {}", add_one(5));
    
    // Closure with type annotations
    let add: fn(i32, i32) -> i32 = |a, b| a + b;
    println!("add(2, 3): {}", add(2, 3));
    
    // Multi-line closure
    let calculate = |x: i32| {
        let doubled = x * 2;
        let squared = doubled * doubled;
        println!("  Calculated: {} * 2 = {}, squared = {}", x, doubled, squared);
        squared
    };
    println!("calculate(3): {}", calculate(3));
    
    // Closure without parameters
    let greet = || {
        println!("Hello from closure!");
    };
    greet();
    
    // Closure with no return value
    let print_sum = |a: i32, b: i32| {
        println!("Sum: {}", a + b);
    };
    print_sum(10, 20);
    
    // Type inference
    let multiply = |a, b| a * b;
    println!("multiply(4, 5): {}", multiply(4, 5));
    
    println!();
}

/// Capturing environment
fn capturing_environment() {
    println!("--- Capturing Environment ---");
    
    // Capturing by reference (immutable)
    let x = 10;
    let print_x = || println!("x from closure: {}", x);
    print_x();
    println!("x still accessible: {}", x);
    
    // Capturing by reference (mutable)
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("Count: {}", count);
    };
    increment();
    increment();
    increment();
    println!("Final count: {}", count);
    
    // Move closure
    let text = String::from("hello");
    let consume = move || {
        println!("Moved text: {}", text);
    };
    consume();
    // text is no longer available
    
    // Multiple captures
    let x = 5;
    let y = 10;
    let combine = || x + y;
    println!("combine(): {}", combine());
    
    // Capturing mutable reference
    let mut vec = vec![1, 2, 3];
    {
        let mut add_to_vec = || vec.push(4);
        add_to_vec();
    }
    println!("Vec after closure: {:?}", vec);
    
    // Move with multiple values
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let combine_strings = move || {
        format!("{} {}", s1, s2)
    };
    println!("Combined: {}", combine_strings());
    
    println!();
}

/// Closure traits (Fn, FnMut, FnOnce)
fn closure_traits() {
    println!("--- Closure Traits ---");
    
    // Fn: Can be called multiple times, borrows immutably
    fn call_fn<F>(f: F)
    where
        F: Fn(),
    {
        f();
        f();
        f();
    }
    
    let x = 5;
    let print_x = || println!("Fn: x = {}", x);
    call_fn(print_x);
    
    // FnMut: Can be called multiple times, borrows mutably
    fn call_fn_mut<F>(mut f: F)
    where
        F: FnMut(),
    {
        f();
        f();
        f();
    }
    
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("FnMut: count = {}", count);
    };
    call_fn_mut(&mut increment);
    
    // FnOnce: Can be called once, takes ownership
    fn call_fn_once<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }
    
    let text = String::from("hello");
    let consume = move || {
        println!("FnOnce: consumed {}", text);
    };
    call_fn_once(consume);
    // consume can't be called again
    
    // Trait hierarchy: Fn: FnMut: FnOnce
    // Fn implements FnMut, FnMut implements FnOnce
    
    println!();
}

/// Closures as function parameters
fn closures_as_parameters() {
    println!("--- Closures as Parameters ---");
    
    // Generic closure parameter
    fn apply<F>(f: F, value: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(value)
    }
    
    let double = |x| x * 2;
    let square = |x| x * x;
    
    println!("apply(double, 5): {}", apply(double, 5));
    println!("apply(square, 5): {}", apply(square, 5));
    
    // Multiple closures
    fn combine<F, G>(f: F, g: G, value: i32) -> i32
    where
        F: Fn(i32) -> i32,
        G: Fn(i32) -> i32,
    {
        g(f(value))
    }
    
    println!("combine(double, square, 3): {}", combine(double, square, 3));
    
    // Closure with different types
    fn process<F>(f: F, numbers: Vec<i32>)
    where
        F: Fn(i32) -> i32,
    {
        for num in numbers {
            println!("  {} -> {}", num, f(num));
        }
    }
    
    println!("Processing with add_one:");
    process(|x| x + 1, vec![1, 2, 3, 4, 5]);
    
    // Mutable closure parameter
    fn modify<F>(mut f: F, times: usize)
    where
        F: FnMut(),
    {
        for _ in 0..times {
            f();
        }
    }
    
    let mut counter = 0;
    modify(
        || {
            counter += 1;
            println!("  Counter: {}", counter);
        },
        3,
    );
    
    // FnOnce parameter
    fn call_once<F, T>(f: F) -> T
    where
        F: FnOnce() -> T,
    {
        f()
    }
    
    let result = call_once(|| 42);
    println!("call_once result: {}", result);
    
    println!();
}

/// Returning closures
fn returning_closures() {
    println!("--- Returning Closures ---");
    
    // Return impl Fn
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }
    
    let add_five = make_adder(5);
    let add_ten = make_adder(10);
    
    println!("add_five(3): {}", add_five(3));
    println!("add_ten(3): {}", add_ten(3));
    
    // Return different closures (using Box)
    fn make_operator(op: char) -> Box<dyn Fn(i32, i32) -> i32> {
        match op {
            '+' => Box::new(|a, b| a + b),
            '-' => Box::new(|a, b| a - b),
            '*' => Box::new(|a, b| a * b),
            '/' => Box::new(|a, b| a / b),
            _ => Box::new(|a, _| a),
        }
    }
    
    let add = make_operator('+');
    let multiply = make_operator('*');
    
    println!("add(5, 3): {}", add(5, 3));
    println!("multiply(5, 3): {}", multiply(5, 3));
    
    // Closure factory
    fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor
    }
    
    let double = make_multiplier(2);
    let triple = make_multiplier(3);
    
    println!("double(7): {}", double(7));
    println!("triple(7): {}", triple(7));
    
    // Return FnMut
    fn make_counter() -> impl FnMut() -> i32 {
        let mut count = 0;
        move || {
            count += 1;
            count
        }
    }
    
    let mut counter = make_counter();
    println!("counter(): {}", counter());
    println!("counter(): {}", counter());
    println!("counter(): {}", counter());
    
    println!();
}

/// Closures with iterators
fn closures_with_iterators() {
    println!("--- Closures with Iterators ---");
    
    // map
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    // filter
    let evens: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Evens: {:?}", evens);
    
    // fold
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);
    
    // for_each
    print!("For each: ");
    numbers.iter().for_each(|x| print!("{} ", x));
    println!();
    
    // Chain operations
    let result: Vec<_> = numbers
        .iter()
        .filter(|&&x| x % 2 != 0)
        .map(|x| x * x)
        .collect();
    println!("Odd squares: {:?}", result);
    
    // find
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("First even: {:?}", first_even);
    
    // any and all
    let has_five = numbers.iter().any(|&x| x == 5);
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("Has 5: {}, All positive: {}", has_five, all_positive);
    
    // partition
    let (evens, odds): (Vec<&i32>, Vec<&i32>) = numbers.iter().partition(|&&x| x % 2 == 0);
    println!("Partitioned - evens: {:?}, odds: {:?}", evens, odds);
    
    // take_while
    let taken: Vec<_> = numbers.iter().take_while(|&&x| x < 4).collect();
    println!("Take while < 4: {:?}", taken);
    
    // Closure capturing in iterator
    let threshold = 3;
    let above_threshold: Vec<_> = numbers
        .iter()
        .filter(|&&x| x > threshold)
        .collect();
    println!("Above {}: {:?}", threshold, above_threshold);
    
    // Complex transformation
    let words = vec!["hello", "world", "rust"];
    let uppercase_lengths: Vec<_> = words
        .iter()
        .map(|s| s.to_uppercase())
        .map(|s| (s.clone(), s.len()))
        .collect();
    println!("Uppercase with lengths: {:?}", uppercase_lengths);
    
    // Mutable closure in iterator
    let mut total = 0;
    numbers.iter().for_each(|x| {
        total += x;
    });
    println!("Total: {}", total);
    
    println!();
}
