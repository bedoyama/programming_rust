fn main() {
    println!("=== Chapter 4: Ownership and Moves ===\n");

    ownership_basics();
    move_semantics();
    more_move_operations();
    moves_and_control_flow();
    moves_and_indexed_content();
    copy_types();
    rc_and_arc();
}

/// Basic ownership principles
fn ownership_basics() {
    println!("--- Ownership Basics ---");

    // Each value has a single owner
    let s1 = String::from("hello");
    println!("s1 owns: {}", s1);

    // When owner goes out of scope, value is dropped
    {
        let s2 = String::from("scoped");
        println!("s2 in scope: {}", s2);
    } // s2 dropped here
    println!("s2 is now out of scope and dropped");

    // Box demonstrates heap ownership
    let boxed = Box::new(100);
    println!("Boxed value: {}", boxed);
    // boxed dropped at end of scope

    // Ownership prevents double-free
    let s3 = String::from("ownership");
    println!("Original owner s3: {}", s3);

    println!();
}

/// Move semantics - ownership transfer
fn move_semantics() {
    println!("--- Move Semantics ---");

    // Move for heap-allocated types
    let s1 = String::from("hello");
    let s2 = s1; // s1 moved to s2
                 // println!("{}", s1); // ERROR: s1 no longer valid
    println!("s2 (moved from s1): {}", s2);

    // Move in function call
    let s3 = String::from("world");
    takes_ownership(s3); // s3 moved into function
                         // println!("{}", s3); // ERROR: s3 no longer valid

    // Return moves ownership back
    let s4 = gives_ownership();
    println!("s4 (returned): {}", s4);

    // Move and return
    let s5 = String::from("move");
    let s6 = takes_and_gives_back(s5);
    // println!("{}", s5); // ERROR: s5 was moved
    println!("s6 (moved and returned): {}", s6);

    // Vector move
    let v1 = vec![1, 2, 3];
    let v2 = v1; // v1 moved to v2
                 // println!("{:?}", v1); // ERROR: v1 no longer valid
    println!("v2 (moved from v1): {:?}", v2);

    println!();
}

fn takes_ownership(s: String) {
    println!("Function takes ownership: {}", s);
} // s dropped here

fn gives_ownership() -> String {
    String::from("transferred")
}

fn takes_and_gives_back(s: String) -> String {
    println!("Function processes: {}", s);
    s // ownership moved back to caller
}

/// More operations that move
fn more_move_operations() {
    println!("--- More Operations That Move ---");

    // Assignment moves
    let s1 = String::from("hello");
    let s2 = s1; // move
    println!("Assignment move: {}", s2);

    // Passing to function moves
    let v1 = vec![1, 2, 3];
    process_vec(v1); // move
                     // Can't use v1 anymore

    // Returning from function moves
    let s3 = create_string();
    println!("Returned: {}", s3);

    // Tuple construction moves
    let name = String::from("Alice");
    let age = 30;
    let person = (name, age); // name moved
                              // println!("{}", name); // ERROR: name moved
    println!("Tuple: {:?}", person);

    // Struct construction moves
    let title = String::from("Rust Book");
    let book = Book { title, pages: 500 }; // title moved
                                           // println!("{}", title); // ERROR: title moved
    println!("Book: {} ({} pages)", book.title, book.pages);

    // Collection push moves
    let mut vec = Vec::new();
    let s = String::from("item");
    vec.push(s); // s moved into vec
                 // println!("{}", s); // ERROR: s moved
    println!("Vec with moved item: {:?}", vec);

    // Collection extend moves
    let strings = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];
    let mut all_strings = Vec::new();
    all_strings.extend(strings); // strings moved
                                 // println!("{:?}", strings); // ERROR: strings moved
    println!("Extended vec: {:?}", all_strings);

    println!();
}

#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
}

fn process_vec(v: Vec<i32>) {
    println!("Processing: {:?}", v);
}

fn create_string() -> String {
    String::from("created")
}

/// Moves and control flow
fn moves_and_control_flow() {
    println!("--- Moves and Control Flow ---");

    // Move in if expression
    let x = String::from("conditional");
    let y = if true {
        x // x moved here
    } else {
        String::from("else")
    };
    // println!("{}", x); // ERROR: x moved
    println!("Conditional result: {}", y);

    // Move in match
    let option = Some(String::from("value"));
    match option {
        Some(s) => {
            println!("Matched and moved: {}", s);
            // s is moved here, option is now None in concept
        }
        None => println!("None"),
    }
    // option is partially moved

    // Non-moving match with reference
    let option2 = Some(String::from("reference"));
    match &option2 {
        Some(s) => println!("Matched by reference: {}", s),
        None => println!("None"),
    }
    // option2 still valid because we matched by reference
    println!("option2 still valid: {:?}", option2);

    // Loop with move
    let strings = vec![String::from("a"), String::from("b"), String::from("c")];

    for s in strings {
        // strings moved in iteration
        println!("Loop item (moved): {}", s);
    }
    // println!("{:?}", strings); // ERROR: strings moved

    // Loop without move (using reference)
    let strings2 = vec![String::from("x"), String::from("y"), String::from("z")];

    for s in &strings2 {
        // iterate by reference
        println!("Loop item (borrowed): {}", s);
    }
    println!("strings2 still valid: {:?}", strings2);

    // While with move
    let mut stack = vec![String::from("first"), String::from("second")];

    while let Some(item) = stack.pop() {
        println!("Popped (moved): {}", item);
    }

    println!();
}

/// Moves and indexed content
fn moves_and_indexed_content() {
    println!("--- Moves and Indexed Content ---");

    // Cannot move out of vector by indexing
    let mut v = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];

    // let s = v[0]; // ERROR: cannot move out of index

    // Solution 1: Use reference
    let s_ref = &v[0];
    println!("Reference to element: {}", s_ref);

    // Solution 2: Clone
    let s_clone = v[0].clone();
    println!("Cloned element: {}", s_clone);
    println!("Original still there: {}", v[0]);

    // Solution 3: Replace with something else
    let s_take = std::mem::replace(&mut v[0], String::from("replaced"));
    println!("Taken element: {}", s_take);
    println!("Vector now: {:?}", v);

    // Solution 4: Use remove (shifts elements)
    let removed = v.remove(1);
    println!("Removed element: {}", removed);
    println!("Vector after remove: {:?}", v);

    // Solution 5: Use swap_remove (faster, doesn't preserve order)
    let mut v2 = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
    ];
    let swapped = v2.swap_remove(1); // swaps with last, then removes
    println!("Swap removed: {}", swapped);
    println!("Vector after swap_remove: {:?}", v2);

    // Solution 6: Take ownership of entire vector
    let v3 = vec![String::from("x"), String::from("y")];
    for s in v3 {
        // moves each element
        println!("Consuming iteration: {}", s);
    }

    // Moving out of Option
    let mut opt = Some(String::from("optional"));
    let taken = opt.take(); // Replaces with None, returns Some(value)
    println!("Taken from Option: {:?}", taken);
    println!("Option now: {:?}", opt);

    println!();
}

/// Copy types - exception to moves
fn copy_types() {
    println!("--- Copy Types ---");

    // Simple types implement Copy
    let x = 42;
    let y = x; // Copy, not move
    println!("x: {}, y: {} (both valid)", x, y);

    // Numeric types are Copy
    let a: i32 = 10;
    let b = a;
    println!("i32 copies: a={}, b={}", a, b);

    let c: f64 = 3.14;
    let d = c;
    println!("f64 copies: c={}, d={}", c, d);

    // bool and char are Copy
    let flag = true;
    let flag2 = flag;
    println!("bool copies: flag={}, flag2={}", flag, flag2);

    let ch = 'A';
    let ch2 = ch;
    println!("char copies: ch={}, ch2={}", ch, ch2);

    // Tuples of Copy types are Copy
    let tuple = (10, 20);
    let tuple2 = tuple;
    println!("Tuple copies: {:?}, {:?}", tuple, tuple2);

    // Arrays of Copy types are Copy
    let arr = [1, 2, 3];
    let arr2 = arr;
    println!("Array copies: {:?}, {:?}", arr, arr2);

    // References are Copy
    let s = String::from("hello");
    let r1 = &s;
    let r2 = r1; // Copy reference, not String
    println!("Reference copies: r1={}, r2={}", r1, r2);

    // Function pointers are Copy
    let f1: fn(i32) -> i32 = double;
    let f2 = f1;
    println!(
        "Function pointer copies: {}(5)={}, {}(5)={}",
        "f1",
        f1(5),
        "f2",
        f2(5)
    );

    // Custom Copy type
    #[derive(Copy, Clone, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // Copy, not move
    println!("Custom Copy type: p1={:?}, p2={:?}", p1, p2);

    // Types that can't be Copy
    // - String (owns heap data)
    // - Vec<T> (owns heap data)
    // - Box<T> (owns heap data)
    // - Any type with Drop implementation

    println!();
}

fn double(x: i32) -> i32 {
    x * 2
}

/// Rc and Arc: Shared ownership
fn rc_and_arc() {
    println!("--- Rc and Arc: Shared Ownership ---");

    use std::rc::Rc;
    use std::sync::Arc;
    use std::thread;

    // Rc: Reference counted (single-threaded)
    println!("-- Rc (Reference Counted) --");

    let s = Rc::new(String::from("shared"));
    println!("Created Rc, ref count: {}", Rc::strong_count(&s));

    let s2 = Rc::clone(&s); // Increment reference count
    println!("After clone, ref count: {}", Rc::strong_count(&s));

    let s3 = Rc::clone(&s);
    println!("After another clone, ref count: {}", Rc::strong_count(&s));

    println!("All references: s={}, s2={}, s3={}", s, s2, s3);

    {
        let s4 = Rc::clone(&s);
        println!("In scope, ref count: {}", Rc::strong_count(&s));
    } // s4 dropped, count decremented

    println!("After scope, ref count: {}", Rc::strong_count(&s));

    // Using Rc with collections
    let shared_vec = Rc::new(vec![1, 2, 3, 4, 5]);
    let owner1 = Rc::clone(&shared_vec);
    let owner2 = Rc::clone(&shared_vec);

    println!("Shared vec through Rc:");
    println!("  owner1: {:?}", owner1);
    println!("  owner2: {:?}", owner2);
    println!("  ref count: {}", Rc::strong_count(&shared_vec));

    // Arc: Atomic reference counted (thread-safe)
    println!("\n-- Arc (Atomic Reference Counted) --");

    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    println!("Created Arc, ref count: {}", Arc::strong_count(&data));

    let data2 = Arc::clone(&data);
    let data3 = Arc::clone(&data);

    println!("After clones, ref count: {}", Arc::strong_count(&data));

    // Use in threads
    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Thread {}: {:?}", i, data_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("After threads, ref count: {}", Arc::strong_count(&data));

    // Rc vs Arc
    println!("\n-- Rc vs Arc --");
    println!("Rc: Single-threaded, faster");
    println!("Arc: Thread-safe, slightly slower (atomic operations)");
    println!("Both provide shared ownership without mutation");

    // Weak references
    println!("\n-- Weak References --");

    let strong = Rc::new(String::from("strong"));
    let weak = Rc::downgrade(&strong);

    println!("Strong count: {}", Rc::strong_count(&strong));
    println!("Weak count: {}", Rc::weak_count(&strong));

    // Upgrade weak to strong
    match weak.upgrade() {
        Some(s) => println!("Weak upgraded: {}", s),
        None => println!("Value was dropped"),
    }

    drop(strong); // Drop all strong references

    match weak.upgrade() {
        Some(s) => println!("Weak upgraded: {}", s),
        None => println!("Value was dropped (weak now invalid)"),
    }

    println!();
}
