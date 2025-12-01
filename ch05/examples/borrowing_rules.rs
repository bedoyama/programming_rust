// Borrowing rules and patterns

fn main() {
    println!("=== Borrowing Rules and Patterns ===\n");

    immutable_borrowing();
    mutable_borrowing();
    borrow_splitting();
    non_lexical_lifetimes();
    reborrowing();
    common_patterns();
}

/// Immutable borrowing patterns
fn immutable_borrowing() {
    println!("--- Immutable Borrowing ---");

    let data = vec![1, 2, 3, 4, 5];

    // Multiple immutable borrows simultaneously
    let r1 = &data;
    let r2 = &data;
    let r3 = &data;

    println!("Multiple borrows: {:?}, {:?}, {:?}", r1, r2, r3);

    // Borrowing in nested scopes
    let outer = &data;
    {
        let inner = &data;
        println!("Nested borrow: {:?}", inner);
    }
    println!("Outer borrow still valid: {:?}", outer);

    // Borrowing for iteration
    for item in &data {
        println!("Item: {}", item);
    }
    println!("Data still accessible: {:?}", data);

    // Borrowing struct fields
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 10, y: 20 };
    let x_ref = &point.x;
    let y_ref = &point.y;
    println!("Point refs: x={}, y={}", x_ref, y_ref);

    // Immutable borrow doesn't prevent reading
    let value = 42;
    let ref1 = &value;
    let ref2 = &value;
    println!("Value: {}, Ref1: {}, Ref2: {}", value, ref1, ref2);

    println!();
}

/// Mutable borrowing patterns
fn mutable_borrowing() {
    println!("--- Mutable Borrowing ---");

    // Only one mutable borrow at a time
    let mut data = vec![1, 2, 3];

    {
        let m = &mut data;
        m.push(4);
        println!("Mutable borrow: {:?}", m);
    } // m goes out of scope

    // Can borrow again after previous borrow ends
    {
        let m2 = &mut data;
        m2.push(5);
        println!("Second mutable borrow: {:?}", m2);
    }

    // Can't read original while mutably borrowed
    let mut x = 10;
    {
        let m = &mut x;
        *m += 5;
        // println!("{}", x); // ERROR: can't use x while borrowed
        println!("Through mut ref: {}", m);
    }
    println!("After borrow: {}", x);

    // Mutable borrow for iteration
    let mut nums = vec![1, 2, 3];
    for item in &mut nums {
        *item *= 2;
    }
    println!("After doubling: {:?}", nums);

    // Mutable borrow of struct fields
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let mut rect = Rectangle {
        width: 10,
        height: 20,
    };

    {
        let w = &mut rect.width;
        *w = 30;
    }
    println!("Rectangle: {}x{}", rect.width, rect.height);

    println!();
}

/// Borrow splitting
fn borrow_splitting() {
    println!("--- Borrow Splitting ---");

    // Can borrow different fields mutably at same time
    struct Person {
        name: String,
        age: u32,
    }

    let mut person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let name_ref = &mut person.name;
    let age_ref = &mut person.age;

    name_ref.push_str(" Smith");
    *age_ref += 1;

    println!("Person: {} is {} years old", person.name, person.age);

    // Slicing splits array borrows
    let mut arr = [1, 2, 3, 4, 5, 6];
    let (left, right) = arr.split_at_mut(3);

    left[0] = 10;
    right[0] = 40;

    println!("Split array: {:?}", arr);

    // Multiple mutable slices
    let mut data = vec![1, 2, 3, 4, 5];
    let (first_half, second_half) = data.split_at_mut(2);

    first_half[0] = 100;
    second_half[0] = 300;

    println!("Modified halves: {:?}", data);

    // Borrowing different vec elements
    let mut v = vec![String::from("a"), String::from("b"), String::from("c")];

    // Can't get two mutable references to same vec normally
    // But can split:
    let (left, right) = v.split_at_mut(1);
    left[0].push_str("1");
    right[0].push_str("2");

    println!("Vec after split: {:?}", v);

    println!();
}

/// Non-lexical lifetimes (NLL)
fn non_lexical_lifetimes() {
    println!("--- Non-Lexical Lifetimes ---");

    // Before NLL, this wouldn't compile
    let mut data = vec![1, 2, 3];

    let r = &data;
    println!("Immutable borrow: {:?}", r);
    // r's lifetime ends here (last use)

    data.push(4); // OK! r is no longer used
    println!("After push: {:?}", data);

    // Another example
    let mut x = 5;
    let r1 = &x;
    let r2 = &x;
    println!("r1: {}, r2: {}", r1, r2);
    // r1 and r2 no longer used

    x = 10; // OK to mutate now
    println!("x mutated: {}", x);

    // Conditional borrows
    let mut value = String::from("hello");

    if value.len() > 0 {
        let first_char = &value[0..1];
        println!("First char: {}", first_char);
    } // first_char's lifetime ends

    value.push_str(" world"); // OK
    println!("Modified: {}", value);

    // Loop with borrows
    let mut numbers = vec![1, 2, 3, 4, 5];

    for i in 0..numbers.len() {
        let item = &numbers[i];
        println!("Item {}: {}", i, item);
        // item's lifetime ends here
    }

    numbers.push(6); // OK, no outstanding borrows
    println!("After loop: {:?}", numbers);

    println!("\nNLL allows borrows to end at last use,");
    println!("not at end of scope (more flexible)");

    println!();
}

/// Reborrowing
fn reborrowing() {
    println!("--- Reborrowing ---");

    // Immutable reborrow
    let x = 10;
    let r1 = &x;
    let r2 = &*r1; // Reborrow
    println!("Original: {}, Reborrow: {}", r1, r2);

    // Mutable reborrow
    let mut y = 20;
    let m1 = &mut y;

    {
        let m2 = &mut *m1; // Reborrow
        *m2 = 30;
    } // m2's lifetime ends

    *m1 = 40; // Can still use m1
    println!("After reborrow: {}", y);

    // Reborrow in function call
    fn increment(val: &mut i32) {
        *val += 1;
    }

    let mut z = 5;
    let mz = &mut z;

    increment(&mut *mz); // Reborrow for function
    *mz += 1; // Can still use mz

    println!("After reborrows: {}", z);

    // Reborrow with slice
    let mut arr = [1, 2, 3, 4, 5];
    let slice = &mut arr[..];

    {
        let subslice = &mut slice[1..3];
        subslice[0] = 20;
    }

    slice[0] = 10;
    println!("After slice reborrow: {:?}", arr);

    // Automatic reborrowing in methods
    struct Counter {
        count: i32,
    }

    impl Counter {
        fn increment(&mut self) {
            self.count += 1;
        }

        fn get(&self) -> i32 {
            self.count
        }
    }

    let mut counter = Counter { count: 0 };
    let c_ref = &mut counter;

    c_ref.increment(); // Automatic reborrow
    c_ref.increment(); // Can call multiple times

    println!("Counter: {}", c_ref.get());

    println!();
}

/// Common borrowing patterns
fn common_patterns() {
    println!("--- Common Patterns ---");

    // Pattern 1: Temporary borrow for read
    let mut data = vec![1, 2, 3];

    {
        let len = data.len(); // Immutable borrow
        println!("Length: {}", len);
    } // Borrow ends

    data.push(4); // Can mutate now

    // Pattern 2: Borrow for processing, then modify
    let mut text = String::from("hello");

    let uppercase = text.to_uppercase(); // Borrows temporarily
    println!("Uppercase: {}", uppercase);

    text.push_str(" world"); // Original can be modified

    // Pattern 3: Split processing
    fn process_parts(data: &mut [i32]) {
        if data.len() > 1 {
            let (first, rest) = data.split_at_mut(1);
            first[0] *= 2;
            for item in rest {
                *item += 1;
            }
        }
    }

    let mut nums = [1, 2, 3, 4, 5];
    process_parts(&mut nums);
    println!("Processed: {:?}", nums);

    // Pattern 4: Builder pattern with mut ref
    struct Builder {
        text: String,
    }

    impl Builder {
        fn new() -> Self {
            Builder {
                text: String::new(),
            }
        }

        fn add(&mut self, s: &str) -> &mut Self {
            self.text.push_str(s);
            self // Return mut ref for chaining
        }

        fn build(self) -> String {
            self.text
        }
    }

    let mut builder = Builder::new();
    builder.add("Hello").add(" ").add("World");
    let result = builder.build();

    println!("Built: {}", result);

    // Pattern 5: Option with references
    fn find_first_even(nums: &[i32]) -> Option<&i32> {
        nums.iter().find(|&&n| n % 2 == 0)
    }

    let numbers = [1, 3, 4, 7, 8];
    if let Some(even) = find_first_even(&numbers) {
        println!("First even: {}", even);
    }

    // Pattern 6: Avoiding unnecessary clones
    fn process_string(s: &str) -> usize {
        s.len() // Just borrow, don't clone
    }

    let my_string = String::from("test");
    let len = process_string(&my_string); // Efficient
    println!("String length: {}", len);

    println!();
}
