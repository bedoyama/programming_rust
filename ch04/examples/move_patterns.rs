// Advanced move patterns and borrowing strategies

fn main() {
    println!("=== Advanced Move Patterns ===\n");

    borrowing_vs_moving();
    partial_moves();
    move_closures();
    avoiding_moves();
}

/// Borrowing vs moving strategies
fn borrowing_vs_moving() {
    println!("--- Borrowing vs Moving ---");

    let data = vec![1, 2, 3, 4, 5];

    // Strategy 1: Borrow (read-only)
    print_vec(&data);
    println!("Data still available: {:?}", data);

    // Strategy 2: Mutable borrow (modify)
    let mut data2 = vec![1, 2, 3];
    modify_vec(&mut data2);
    println!("After modification: {:?}", data2);

    // Strategy 3: Move (consume)
    let data3 = vec![1, 2, 3];
    let sum = consume_vec(data3);
    println!("Consumed and returned sum: {}", sum);
    // data3 no longer available

    // Strategy 4: Clone (when you need both)
    let data4 = vec![1, 2, 3];
    let data5 = data4.clone();
    println!("Original: {:?}, Clone: {:?}", data4, data5);

    // Strategy 5: Return ownership
    let data6 = vec![1, 2, 3];
    let data7 = process_and_return(data6);
    println!("Returned: {:?}", data7);

    println!();
}

fn print_vec(v: &Vec<i32>) {
    println!("Printing (borrowed): {:?}", v);
}

fn modify_vec(v: &mut Vec<i32>) {
    v.push(100);
    println!("Modified (mut borrowed): {:?}", v);
}

fn consume_vec(v: Vec<i32>) -> i32 {
    v.iter().sum()
}

fn process_and_return(mut v: Vec<i32>) -> Vec<i32> {
    v.push(99);
    v
}

/// Partial moves from structs
fn partial_moves() {
    println!("--- Partial Moves ---");

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        email: String,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };

    // Can move Copy fields
    let age = person.age; // Copy, not move
    println!("Age (copied): {}", age);
    // person.age still accessible

    // Moving non-Copy field partially moves struct
    let name = person.name; // Moves name out
    println!("Name (moved): {}", name);

    // Can still access Copy fields
    println!("Age still accessible: {}", person.age);

    // Can still access non-moved fields
    println!("Email still accessible: {}", person.email);

    // Can't access moved field
    // println!("{}", person.name); // ERROR: name was moved

    // Can't use struct as a whole
    // println!("{:?}", person); // ERROR: partially moved

    // Solution: Borrow instead of move
    let person2 = Person {
        name: String::from("Bob"),
        age: 25,
        email: String::from("bob@example.com"),
    };

    let name_ref = &person2.name; // Borrow, not move
    println!("Name (borrowed): {}", name_ref);
    println!("Person still valid: {:?}", person2);

    println!();
}

/// Move closures
fn move_closures() {
    println!("--- Move Closures ---");

    // Regular closure borrows
    let data = vec![1, 2, 3];
    let closure = || {
        println!("Borrowed in closure: {:?}", data);
    };
    closure();
    println!("Data still available: {:?}", data);

    // Move closure takes ownership
    let data2 = vec![4, 5, 6];
    let move_closure = move || {
        println!("Moved into closure: {:?}", data2);
    };
    move_closure();
    // println!("{:?}", data2); // ERROR: data2 was moved

    // Move closure with threads
    use std::thread;

    let data3 = vec![7, 8, 9];
    let handle = thread::spawn(move || {
        println!("Thread owns: {:?}", data3);
    });
    handle.join().unwrap();
    // data3 moved into thread

    // Multiple values in move closure
    let x = String::from("hello");
    let y = String::from("world");
    let closure2 = move || {
        println!("{} {}", x, y);
    };
    closure2();
    // Both x and y moved

    // Copy types in move closures
    let num = 42;
    let closure3 = move || {
        println!("Number: {}", num);
    };
    closure3();
    println!("Number still available (Copy): {}", num);

    println!();
}

/// Techniques to avoid unnecessary moves
fn avoiding_moves() {
    println!("--- Avoiding Moves ---");

    // 1. Use references in collections
    let s1 = String::from("one");
    let s2 = String::from("two");
    let s3 = String::from("three");

    let vec_refs = vec![&s1, &s2, &s3];
    println!("Vec of refs: {:?}", vec_refs);
    println!("Originals still valid: {}, {}, {}", s1, s2, s3);

    // 2. Use as_ref() to avoid moving Option
    let opt = Some(String::from("value"));
    if let Some(s) = opt.as_ref() {
        println!("Option value (borrowed): {}", s);
    }
    println!("Option still valid: {:?}", opt);

    // 3. Use iter() instead of into_iter()
    let vec = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    println!("Original: {:?}", vec);
    println!("Doubled: {:?}", doubled);

    // 4. Clone when necessary (explicit cost)
    let original = vec![String::from("a"), String::from("b")];
    let copy = original.clone();
    println!("Original: {:?}", original);
    println!("Copy: {:?}", copy);

    // 5. Use Rc/Arc for shared ownership
    use std::rc::Rc;

    let shared = Rc::new(vec![1, 2, 3]);
    let ref1 = Rc::clone(&shared);
    let ref2 = Rc::clone(&shared);

    println!("Shared data: {:?}", shared);
    println!("Reference 1: {:?}", ref1);
    println!("Reference 2: {:?}", ref2);

    // 6. Use std::mem::take for Option<T>
    let mut opt_data = Some(String::from("data"));
    let taken = opt_data.take();
    println!("Taken: {:?}", taken);
    println!("Option now None: {:?}", opt_data);

    // 7. Use std::mem::replace
    let mut s = String::from("old");
    let old = std::mem::replace(&mut s, String::from("new"));
    println!("Old: {}, New: {}", old, s);

    // 8. Use std::mem::swap
    let mut a = String::from("A");
    let mut b = String::from("B");
    std::mem::swap(&mut a, &mut b);
    println!("After swap: a={}, b={}", a, b);

    println!();
}
