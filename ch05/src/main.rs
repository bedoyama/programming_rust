fn main() {
    println!("=== Chapter 5: References ===\n");

    references_to_values();
    working_with_references();
    rust_vs_cpp_references();
    assigning_references();
    references_to_references();
    comparing_references();
    references_never_null();
    borrowing_expressions();
    references_to_slices();
    reference_safety();
    borrowing_local_variables();
    function_references();
    returning_references();
    structs_with_references();
    lifetime_parameters();
    sharing_vs_mutation();
}

/// Basic reference concepts
fn references_to_values() {
    println!("--- References to Values ---");

    // Immutable reference
    let x = 10;
    let r = &x;
    println!("x = {}, reference r = {}", x, r);
    println!("Dereferenced: *r = {}", *r);

    // Multiple immutable references are allowed
    let r1 = &x;
    let r2 = &x;
    let r3 = &x;
    println!("Multiple refs: r1={}, r2={}, r3={}", r1, r2, r3);

    // Mutable reference
    let mut y = 20;
    let m = &mut y;
    *m += 10;
    println!("After mutation through ref: y = {}", y);

    // Only one mutable reference at a time
    let mut z = 30;
    {
        let m1 = &mut z;
        *m1 += 5;
    } // m1 goes out of scope
    println!("z after mutable borrow: {}", z);

    // References don't take ownership
    let s = String::from("hello");
    let s_ref = &s;
    println!("String: {}, reference: {}", s, s_ref);
    // s is still valid

    println!();
}

/// Working with references in practice
fn working_with_references() {
    println!("--- Working with References ---");

    // Passing by reference to avoid moves
    let v = vec![1, 2, 3, 4, 5];
    let sum = sum_vec(&v);
    println!("Sum: {}, vec still available: {:?}", sum, v);

    // Modifying through mutable reference
    let mut data = vec![10, 20, 30];
    add_element(&mut data, 40);
    println!("After modification: {:?}", data);

    // Reference in struct field access
    let point = Point { x: 5, y: 10 };
    let x_ref = &point.x;
    println!("Point.x via reference: {}", x_ref);

    // Implicit dereferencing with methods
    let s = String::from("hello");
    let len = (&s).len(); // Explicit reference
    let len2 = s.len(); // Implicit, Rust auto-borrows
    println!("Length: {} == {}", len, len2);

    // Automatic dereferencing in comparisons
    let a = 42;
    let b = &a;
    let c = &&a;
    println!(
        "Auto-deref in comparison: a == *b == **c: {}",
        a == *b && *b == **c
    );

    println!();
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn sum_vec(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}

fn add_element(v: &mut Vec<i32>, elem: i32) {
    v.push(elem);
}

/// Rust references vs C++ references
fn rust_vs_cpp_references() {
    println!("--- Rust vs C++ References ---");

    // Rust references can be reassigned
    let x = 10;
    let y = 20;
    let mut r = &x;
    println!("Reference to x: {}", r);
    r = &y; // Can reassign reference
    println!("Reference now to y: {}", r);

    // References are first-class values
    let a = 5;
    let ref_array = [&a, &a, &a];
    println!("Array of references: {:?}", ref_array);

    // References can be copied (they implement Copy)
    let b = 100;
    let r1 = &b;
    let r2 = r1; // r1 is copied, both valid
    println!("Both references valid: r1={}, r2={}", r1, r2);

    println!("Key differences from C++:");
    println!("  - Rust refs can be null-checked at compile time");
    println!("  - Rust refs follow borrowing rules");
    println!("  - Rust refs can be reassigned");
    println!("  - Rust has explicit lifetime tracking");

    println!();
}

/// Assigning references
fn assigning_references() {
    println!("--- Assigning References ---");

    let x = 10;
    let y = 20;
    let mut r = &x;

    println!("r points to x: {}", r);
    r = &y; // Change what r refers to
    println!("r now points to y: {}", r);

    // Assigning through mutable reference
    let mut a = 5;
    let mut b = 10;
    let mut r_mut = &mut a;
    *r_mut = 100; // Modify what r_mut points to
    println!("a after *r_mut = 100: {}", a);

    r_mut = &mut b; // Change what r_mut points to
    *r_mut = 200;
    println!("b after *r_mut = 200: {}", b);

    // Reference assignment vs value assignment
    let mut val = 42;
    let r = &mut val;
    *r = 99; // Assigns to what r points to
    println!("val after *r = 99: {}", val);

    println!();
}

/// References to references
fn references_to_references() {
    println!("--- References to References ---");

    let x = 10;
    let r = &x; // &i32
    let rr = &r; // &&i32
    let rrr = &rr; // &&&i32

    println!("x = {}", x);
    println!("*r = {}", *r);
    println!("**rr = {}", **rr);
    println!("***rrr = {}", ***rrr);

    // Automatic dereferencing
    println!("Implicit deref: x == r? {}", x == *r);

    // References to references in functions
    fn takes_ref_to_ref(rr: &&i32) {
        println!("Inside function: **rr = {}", **rr);
    }

    takes_ref_to_ref(&r);

    // Pattern matching with nested refs
    let value = 42;
    let ref1 = &value;
    let ref2 = &ref1;

    match ref2 {
        &&v => println!("Pattern matched value: {}", v),
    }

    println!();
}

/// Comparing references
fn comparing_references() {
    println!("--- Comparing References ---");

    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    // Compare values (automatic dereference)
    println!("Values equal (rx == ry): {}", rx == ry);

    // Compare references themselves (pointer equality)
    println!("Same reference (std::ptr::eq): {}", std::ptr::eq(rx, ry));

    let rx2 = &x;
    println!(
        "rx and rx2 point to same location: {}",
        std::ptr::eq(rx, rx2)
    );

    // Comparing with dereferencing
    println!("*rx == *ry: {}", *rx == *ry);

    // Reference comparison with strings
    let s1 = String::from("hello");
    let s2 = String::from("hello");
    let s3 = &s1;

    println!("String value equality: {}", s1 == s2);
    println!("String ref to original: {}", std::ptr::eq(&s1, s3));
    println!("String ref value eq: {}", s3 == &s2);

    println!();
}

/// References are never null
fn references_never_null() {
    println!("--- References Are Never Null ---");

    // Can't create null reference in safe Rust
    let x = 10;
    let r = &x; // Always valid

    println!("Reference is always valid: {}", r);

    // Use Option for nullable references
    let opt_ref: Option<&i32> = Some(&x);
    match opt_ref {
        Some(r) => println!("Has reference: {}", r),
        None => println!("No reference"),
    }

    let no_ref: Option<&i32> = None;
    match no_ref {
        Some(r) => println!("Has reference: {}", r),
        None => println!("No reference (this prints)"),
    }

    // Function taking optional reference
    fn process_optional(opt: Option<&String>) {
        match opt {
            Some(s) => println!("Got string: {}", s),
            None => println!("Got None"),
        }
    }

    let s = String::from("test");
    process_optional(Some(&s));
    process_optional(None);

    println!("No null pointer exceptions in safe Rust!");

    println!();
}

/// Borrowing references to arbitrary expressions
fn borrowing_expressions() {
    println!("--- Borrowing Arbitrary Expressions ---");

    // Can take reference to any expression
    let r = &(2 + 2);
    println!("Reference to expression: {}", r);

    // Reference to function return value
    fn return_value() -> i32 {
        42
    }
    let r2 = &return_value();
    println!("Reference to function result: {}", r2);

    // Reference to compound expression
    let x = 10;
    let y = 20;
    let r3 = &(x + y);
    println!("Reference to sum: {}", r3);

    // Temporary values are kept alive
    let r4 = &String::from("temporary");
    println!("Reference to temporary: {}", r4);

    // Reference to array element
    let arr = [1, 2, 3, 4, 5];
    let r5 = &arr[2];
    println!("Reference to array element: {}", r5);

    // Reference in method chains
    let v = vec![1, 2, 3];
    let r6 = &v.len();
    println!("Reference to method result: {}", r6);

    println!();
}

/// References to slices and trait objects
fn references_to_slices() {
    println!("--- References to Slices and Trait Objects ---");

    // Slice references (fat pointers)
    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    println!("Slice reference: {:?}", slice);
    println!("Slice length: {}", slice.len());

    // String slices
    let s = String::from("hello world");
    let slice: &str = &s[0..5];
    println!("String slice: {}", slice);

    // Mutable slice
    let mut arr2 = [10, 20, 30, 40, 50];
    let slice_mut: &mut [i32] = &mut arr2[1..4];
    slice_mut[0] = 99;
    println!("Array after mut slice modification: {:?}", arr2);

    // Fat pointer explanation
    println!("\nSlice references are 'fat pointers':");
    println!("  - Pointer to data");
    println!("  - Length of slice");
    println!("  Size: {} bytes", std::mem::size_of::<&[i32]>());

    // Trait objects (also fat pointers)
    trait Drawable {
        fn draw(&self);
    }

    struct Circle {
        radius: f64,
    }
    impl Drawable for Circle {
        fn draw(&self) {
            println!("Drawing circle with radius {}", self.radius);
        }
    }

    let circle = Circle { radius: 5.0 };
    let drawable: &dyn Drawable = &circle;
    drawable.draw();

    println!(
        "Trait object size: {} bytes",
        std::mem::size_of::<&dyn Drawable>()
    );

    println!();
}

/// Reference safety basics
fn reference_safety() {
    println!("--- Reference Safety ---");

    // Valid reference (outlives usage)
    let x = 10;
    let r = &x;
    println!("Valid reference: {}", r);

    // Borrow checker prevents dangling references
    // This won't compile:
    // let r;
    // {
    //     let x = 10;
    //     r = &x;  // ERROR: x doesn't live long enough
    // }
    // println!("{}", r);

    // References must not outlive referent
    // let r = {
    //     let x = 42;
    //     &x // ERROR if we tried to return this
    // };
    // Can't use r here - would be dangling!

    // Correct pattern
    let x = 42;
    let r = {
        &x // x lives long enough
    };
    println!("Valid scoped reference: {}", r);

    println!("Borrow checker ensures:");
    println!("  - No dangling references");
    println!("  - References don't outlive data");
    println!("  - Mutable XOR shared aliasing");

    println!();
}

/// Borrowing local variables
fn borrowing_local_variables() {
    println!("--- Borrowing Local Variables ---");

    // Simple borrow
    let x = 10;
    let r = &x;
    println!("Borrowed local: {}", r);

    // Borrow must not outlive owner
    {
        let y = 20;
        let r = &y;
        println!("Borrow in scope: {}", r);
    } // r goes out of scope with y

    // Multiple borrows
    let mut data = vec![1, 2, 3];
    {
        let r1 = &data;
        let r2 = &data;
        println!("Multiple immutable borrows: {:?}, {:?}", r1, r2);
    } // borrows end

    // Now can mutably borrow
    data.push(4);
    println!("After mutable access: {:?}", data);

    // Borrow splitting
    let mut pair = (String::from("a"), String::from("b"));
    let first = &mut pair.0;
    let second = &mut pair.1;
    first.push_str("1");
    second.push_str("2");
    println!("Borrowed fields separately: {:?}", pair);

    println!();
}

/// Receiving and passing references as function arguments
fn function_references() {
    println!("--- Function References ---");

    // Receiving immutable reference
    fn print_value(val: &i32) {
        println!("Received reference: {}", val);
    }

    let x = 42;
    print_value(&x);
    println!("x still available: {}", x);

    // Receiving mutable reference
    fn increment(val: &mut i32) {
        *val += 1;
    }

    let mut y = 10;
    increment(&mut y);
    println!("After increment: {}", y);

    // Multiple immutable references
    fn sum_refs(a: &i32, b: &i32, c: &i32) -> i32 {
        *a + *b + *c
    }

    let a = 1;
    let b = 2;
    let c = 3;
    let result = sum_refs(&a, &b, &c);
    println!("Sum: {}", result);

    // Mixing owned and borrowed
    fn process_data(owned: String, borrowed: &String) {
        println!("Owned: {}, Borrowed: {}", owned, borrowed);
    }

    let s1 = String::from("moved");
    let s2 = String::from("borrowed");
    process_data(s1, &s2);
    // s1 moved, s2 still available
    println!("s2 still here: {}", s2);

    println!();
}

/// Returning references from functions
fn returning_references() {
    println!("--- Returning References ---");

    // Return reference to parameter (lifetime tied to input)
    fn first_element<'a>(slice: &'a [i32]) -> &'a i32 {
        &slice[0]
    }

    let arr = [10, 20, 30];
    let first = first_element(&arr);
    println!("First element: {}", first);

    // Return reference to longer-lived data
    fn get_static() -> &'static str {
        "static string"
    }

    let s = get_static();
    println!("Static reference: {}", s);

    // Choose between two references
    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }

    let str1 = String::from("short");
    let str2 = String::from("longer string");
    let result = longest(&str1, &str2);
    println!("Longest: {}", result);

    // Can't return reference to local
    // fn dangling() -> &i32 {
    //     let x = 10;
    //     &x  // ERROR: returns reference to local
    // }

    println!();
}

/// Structs containing references
fn structs_with_references() {
    println!("--- Structs Containing References ---");

    // Struct with lifetime parameter
    #[derive(Debug)]
    struct StringRef<'a> {
        text: &'a str,
    }

    let s = String::from("hello");
    let string_ref = StringRef { text: &s };
    println!("Struct with reference: {:?}", string_ref);

    // Struct with multiple references
    #[derive(Debug)]
    struct Pair<'a, 'b> {
        first: &'a str,
        second: &'b str,
    }

    let s1 = String::from("first");
    let s2 = String::from("second");
    let pair = Pair {
        first: &s1,
        second: &s2,
    };
    println!("Pair: {:?}", pair);

    // Struct method with references
    struct Data<'a> {
        value: &'a i32,
    }

    impl<'a> Data<'a> {
        fn get_value(&self) -> &'a i32 {
            self.value
        }
    }

    let x = 42;
    let data = Data { value: &x };
    println!("Value from method: {}", data.get_value());

    println!();
}

/// Distinct lifetime parameters
fn lifetime_parameters() {
    println!("--- Lifetime Parameters ---");

    // Single lifetime
    fn first<'a>(slice: &'a [i32]) -> &'a i32 {
        &slice[0]
    }

    let arr = [1, 2, 3];
    println!("First: {}", first(&arr));

    // Multiple lifetimes
    fn select<'a, 'b>(flag: bool, a: &'a str, b: &'b str) -> &'a str {
        if flag {
            a
        } else {
            a
        } // Always returns 'a
    }

    let s1 = String::from("first");
    let s2 = String::from("second");
    println!("Selected: {}", select(true, &s1, &s2));

    // Lifetime in structs
    struct Context<'a> {
        name: &'a str,
    }

    impl<'a> Context<'a> {
        fn new(name: &'a str) -> Self {
            Context { name }
        }

        fn get_name(&self) -> &'a str {
            self.name
        }
    }

    let name = String::from("MyContext");
    let ctx = Context::new(&name);
    println!("Context name: {}", ctx.get_name());

    // Lifetime elision (compiler infers)
    fn implicit(s: &str) -> &str {
        // Lifetimes inferred
        s
    }

    println!("Implicit lifetime: {}", implicit("test"));

    println!();
}

/// Sharing vs mutation rules
fn sharing_vs_mutation() {
    println!("--- Sharing vs Mutation ---");

    // Rule: Many readers OR one writer, not both

    let mut data = vec![1, 2, 3];

    // Multiple immutable borrows OK
    {
        let r1 = &data;
        let r2 = &data;
        let r3 = &data;
        println!("Multiple readers: {:?}, {:?}, {:?}", r1, r2, r3);
    } // All immutable borrows end

    // One mutable borrow OK
    {
        let m = &mut data;
        m.push(4);
        println!("One writer: {:?}", m);
    } // Mutable borrow ends

    // Can't mix immutable and mutable
    // {
    //     let r = &data;
    //     let m = &mut data;  // ERROR: can't borrow as mutable
    //     println!("{:?}, {:?}", r, m);
    // }

    // Sequential borrows OK
    let r = &data;
    println!("Read: {:?}", r);
    // r is last used here

    let m = &mut data;
    m.push(5);
    println!("Write: {:?}", m);

    // Interior mutability (special case)
    use std::cell::RefCell;

    let cell = RefCell::new(vec![1, 2, 3]);

    // Can have immutable reference to RefCell but mutate contents
    let ref1 = &cell;
    ref1.borrow_mut().push(4);

    let ref2 = &cell;
    println!("RefCell contents: {:?}", ref2.borrow());

    println!("\nBorrowing rules:");
    println!("  ✓ Any number of immutable references");
    println!("  ✓ Exactly one mutable reference");
    println!("  ✗ Immutable and mutable at same time");
    println!("  ✓ References must not outlive data");

    println!();
}
