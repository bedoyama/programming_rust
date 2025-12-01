// Reference safety and common pitfalls

fn main() {
    println!("=== Reference Safety ===\n");

    dangling_prevention();
    lifetime_errors();
    borrowing_conflicts();
    interior_mutability();
    unsafe_references();
}

/// How Rust prevents dangling references
fn dangling_prevention() {
    println!("--- Dangling Reference Prevention ---");

    // ILLEGAL: Can't return reference to local
    // fn dangling() -> &String {
    //     let s = String::from("hello");
    //     &s // ERROR: s will be dropped
    // }

    // CORRECT: Return owned value
    fn not_dangling() -> String {
        let s = String::from("hello");
        s // Ownership moves out
    }

    let result = not_dangling();
    println!("Safe return: {}", result);

    // ILLEGAL: Reference outlives value
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // ERROR: x will be dropped
    // }
    // println!("{}", r); // ERROR: r would be dangling

    // CORRECT: Value lives long enough
    let x = 5;
    let r = &x;
    println!("Valid reference: {}", r);

    // ILLEGAL: Returning reference to parameter that's moved
    // fn try_to_escape(s: String) -> &str {
    //     &s[..] // ERROR: s will be dropped
    // }

    // CORRECT: Borrow parameter
    fn slice_it(s: &String) -> &str {
        &s[..]
    }

    let text = String::from("hello");
    let slice = slice_it(&text);
    println!("Safe slice: {}", slice);

    // Static data is always safe
    fn get_static() -> &'static str {
        "This lives forever"
    }

    println!("Static ref: {}", get_static());

    println!();
}

/// Common lifetime errors and solutions
fn lifetime_errors() {
    println!("--- Lifetime Errors ---");

    // ERROR: Conflicting lifetimes
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() { x } else { y }
    // } // ERROR: Rust can't infer return lifetime

    // CORRECT: Explicit lifetime
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let s1 = String::from("long string");
    let s2 = String::from("short");
    let result = longest(&s1, &s2);
    println!("Longest: {}", result);

    // ERROR: Lifetime too short
    // let result;
    // {
    //     let s2 = String::from("short");
    //     result = longest(&s1, &s2);
    // } // ERROR: s2 dropped but result needs it
    // println!("{}", result);

    // CORRECT: Both parameters live long enough
    let s1 = String::from("first");
    let s2 = String::from("second");
    let result = longest(&s1, &s2);
    println!("Both valid: {}", result);

    // ERROR: Struct with invalid lifetime
    // struct Wrapper<'a> {
    //     data: &'a str,
    // }
    //
    // let wrapper;
    // {
    //     let s = String::from("temp");
    //     wrapper = Wrapper { data: &s };
    // } // ERROR: s dropped
    // println!("{}", wrapper.data);

    // CORRECT: Value outlives struct
    struct Wrapper<'a> {
        data: &'a str,
    }

    let s = String::from("permanent");
    let wrapper = Wrapper { data: &s };
    println!("Wrapper: {}", wrapper.data);

    println!();
}

/// Borrowing conflicts
fn borrowing_conflicts() {
    println!("--- Borrowing Conflicts ---");

    // ERROR: Mutable and immutable borrow
    let mut data = vec![1, 2, 3];

    // let r = &data;
    // data.push(4); // ERROR: can't mutate while borrowed
    // println!("{:?}", r);

    // CORRECT: Separate scopes
    {
        let r = &data;
        println!("Immutable borrow: {:?}", r);
    } // r dropped

    data.push(4); // OK now
    println!("After mutation: {:?}", data);

    // ERROR: Multiple mutable borrows
    // let m1 = &mut data;
    // let m2 = &mut data; // ERROR: already borrowed
    // m1.push(5);
    // m2.push(6);

    // CORRECT: Sequential mutable borrows
    {
        let m1 = &mut data;
        m1.push(5);
    } // m1 dropped

    {
        let m2 = &mut data;
        m2.push(6);
    }

    println!("After sequential borrows: {:?}", data);

    // ERROR: Using value while borrowed
    let mut x = 10;
    // let r = &x;
    // x = 20; // ERROR: can't assign while borrowed
    // println!("{}", r);

    // CORRECT: NLL allows this pattern
    let r = &x;
    println!("Borrowed: {}", r);
    // r no longer used
    x = 20; // OK
    println!("Modified: {}", x);

    // ERROR: Iterator invalidation
    let mut v = vec![1, 2, 3];
    // for item in &v {
    //     v.push(*item); // ERROR: modifying while iterating
    // }

    // CORRECT: Collect modifications first
    let to_add: Vec<_> = v.iter().copied().collect();
    v.extend(to_add);
    println!("After safe iteration: {:?}", v);

    println!();
}

/// Interior mutability patterns
fn interior_mutability() {
    println!("--- Interior Mutability ---");

    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    // Cell: Copy types only
    let x = Cell::new(5);
    let r1 = &x;
    let r2 = &x;

    r1.set(10);
    r2.set(20);

    println!("Cell value: {}", x.get());

    // RefCell: Runtime borrow checking
    let data = RefCell::new(vec![1, 2, 3]);

    // Multiple immutable borrows OK
    {
        let r1 = data.borrow();
        let r2 = data.borrow();
        println!("RefCell borrows: {:?}, {:?}", r1, r2);
    } // Borrows dropped

    // Mutable borrow
    {
        let mut m = data.borrow_mut();
        m.push(4);
        println!("After mut borrow: {:?}", m);
    }

    // ERROR: Conflicting borrows panic at runtime
    // let r = data.borrow();
    // let m = data.borrow_mut(); // PANIC!
    // println!("{:?}", r);

    // CORRECT: Drop previous borrow
    {
        let r = data.borrow();
        println!("Immutable: {:?}", r);
    } // r dropped

    {
        let mut m = data.borrow_mut();
        m.push(5);
    }

    // Rc + RefCell pattern
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let root = Rc::new(Node {
        value: 1,
        children: RefCell::new(vec![]),
    });

    let child = Rc::new(Node {
        value: 2,
        children: RefCell::new(vec![]),
    });

    // Mutate through shared reference
    root.children.borrow_mut().push(Rc::clone(&child));

    println!("Node with children: {:?}", root);

    println!();
}

/// Unsafe reference operations
fn unsafe_references() {
    println!("--- Unsafe References ---");

    // Raw pointers
    let mut x = 10;
    let r1 = &x as *const i32;
    let r2 = &mut x as *mut i32;

    println!("Raw pointers created: {:?}, {:?}", r1, r2);

    // Dereferencing is unsafe
    unsafe {
        println!("Via const pointer: {}", *r1);
        *r2 = 20;
        println!("Via mut pointer: {}", *r2);
    }

    // Can create dangling raw pointers (but can't safely use them)
    let dangling = {
        let temp = 42;
        &temp as *const i32
    }; // temp dropped

    println!("Dangling pointer exists: {:?}", dangling);
    // unsafe { println!("{}", *dangling); } // UB: Undefined behavior!

    // Mutable aliases with raw pointers
    let mut data = vec![1, 2, 3];
    let ptr1 = data.as_mut_ptr();
    let ptr2 = data.as_mut_ptr();

    unsafe {
        *ptr1 = 10;
        *ptr2 = 20; // Overwrites ptr1's write

        println!("First element: {}", data[0]);
    }

    // Converting between reference and raw pointer
    let s = String::from("hello");
    let ptr = &s as *const String;

    unsafe {
        let back_to_ref = &*ptr;
        println!("Via raw pointer: {}", back_to_ref);
    }

    // Slice from raw parts
    let v = vec![1, 2, 3, 4, 5];
    let ptr = v.as_ptr();
    let len = v.len();

    unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        println!("Slice from raw: {:?}", slice);
    }

    println!("\nRaw pointers bypass Rust's safety guarantees.");
    println!("Use only when necessary and with extreme care!");

    println!();
}
