// Rc and Arc: Shared ownership patterns

fn main() {
    println!("=== Rc and Arc: Shared Ownership ===\n");
    
    rc_basics();
    rc_patterns();
    arc_basics();
    arc_threading();
    weak_references();
    interior_mutability();
}

/// Basic Rc usage
fn rc_basics() {
    println!("--- Rc Basics ---");
    
    use std::rc::Rc;
    
    // Creating an Rc
    let value = Rc::new(String::from("shared"));
    println!("Created Rc: {}", value);
    println!("Reference count: {}", Rc::strong_count(&value));
    
    // Cloning Rc increments count
    let value2 = Rc::clone(&value);
    println!("After clone, count: {}", Rc::strong_count(&value));
    
    let value3 = value.clone(); // Also works (calls Rc::clone)
    println!("After another clone, count: {}", Rc::strong_count(&value));
    
    // All references point to same data
    println!("value: {}", value);
    println!("value2: {}", value2);
    println!("value3: {}", value3);
    
    // Dropping decrements count
    drop(value3);
    println!("After drop, count: {}", Rc::strong_count(&value));
    
    // Automatic cleanup when count reaches 0
    {
        let temp = Rc::clone(&value);
        println!("In scope, count: {}", Rc::strong_count(&value));
    }
    println!("After scope, count: {}", Rc::strong_count(&value));
    
    println!();
}

/// Common Rc patterns
fn rc_patterns() {
    println!("--- Rc Patterns ---");
    
    use std::rc::Rc;
    
    // Pattern 1: Shared data structure
    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Rc<Node>>,
    }
    
    let node3 = Rc::new(Node { value: 3, next: None });
    let node2 = Rc::new(Node { value: 2, next: Some(Rc::clone(&node3)) });
    let node1 = Rc::new(Node { value: 1, next: Some(Rc::clone(&node2)) });
    
    println!("Linked list: {:?}", node1);
    println!("node3 ref count: {}", Rc::strong_count(&node3));
    
    // Pattern 2: Shared configuration
    struct Config {
        host: String,
        port: u16,
    }
    
    let config = Rc::new(Config {
        host: String::from("localhost"),
        port: 8080,
    });
    
    let server1 = Rc::clone(&config);
    let server2 = Rc::clone(&config);
    
    println!("Server 1: {}:{}", server1.host, server1.port);
    println!("Server 2: {}:{}", server2.host, server2.port);
    println!("Config shared by {} servers", Rc::strong_count(&config));
    
    // Pattern 3: Multiple owners
    let data = Rc::new(vec![1, 2, 3, 4, 5]);
    
    let process1 = Rc::clone(&data);
    let process2 = Rc::clone(&data);
    let process3 = Rc::clone(&data);
    
    println!("Process 1 sum: {}", process1.iter().sum::<i32>());
    println!("Process 2 max: {:?}", process2.iter().max());
    println!("Process 3 len: {}", process3.len());
    
    println!();
}

/// Basic Arc usage
fn arc_basics() {
    println!("--- Arc Basics ---");
    
    use std::sync::Arc;
    
    // Arc is like Rc but thread-safe
    let value = Arc::new(String::from("thread-safe"));
    println!("Created Arc: {}", value);
    println!("Reference count: {}", Arc::strong_count(&value));
    
    // Cloning Arc
    let value2 = Arc::clone(&value);
    let value3 = Arc::clone(&value);
    
    println!("After clones, count: {}", Arc::strong_count(&value));
    
    // All references valid
    println!("value: {}", value);
    println!("value2: {}", value2);
    println!("value3: {}", value3);
    
    // Arc uses atomic operations (slightly slower than Rc)
    println!("\nArc vs Rc:");
    println!("  Arc: Thread-safe, atomic ref counting");
    println!("  Rc:  Single-threaded, faster");
    
    println!();
}

/// Arc with threading
fn arc_threading() {
    println!("--- Arc with Threading ---");
    
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;
    
    // Share data across threads
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            let sum: i32 = data_clone.iter().sum();
            println!("Thread {} computed sum: {}", i, sum);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Main thread still has data: {:?}", data);
    println!("Final ref count: {}", Arc::strong_count(&data));
    
    // Shared configuration across threads
    #[derive(Debug)]
    struct AppConfig {
        max_connections: usize,
        timeout_secs: u64,
    }
    
    let config = Arc::new(AppConfig {
        max_connections: 100,
        timeout_secs: 30,
    });
    
    let mut workers = vec![];
    
    for i in 0..5 {
        let cfg = Arc::clone(&config);
        let worker = thread::spawn(move || {
            println!("Worker {}: max_conn={}, timeout={}s", 
                     i, cfg.max_connections, cfg.timeout_secs);
        });
        workers.push(worker);
    }
    
    for worker in workers {
        worker.join().unwrap();
    }
    
    println!();
}

/// Weak references
fn weak_references() {
    println!("--- Weak References ---");
    
    use std::rc::{Rc, Weak};
    
    // Creating weak references
    let strong = Rc::new(String::from("strong reference"));
    let weak: Weak<String> = Rc::downgrade(&strong);
    
    println!("Strong count: {}", Rc::strong_count(&strong));
    println!("Weak count: {}", Rc::weak_count(&strong));
    
    // Upgrading weak to strong
    match weak.upgrade() {
        Some(s) => println!("Weak upgraded successfully: {}", s),
        None => println!("Value was dropped"),
    }
    
    // Weak doesn't prevent dropping
    {
        let strong2 = Rc::clone(&strong);
        println!("In scope - strong count: {}", Rc::strong_count(&strong2));
    }
    
    // Drop all strong references
    drop(strong);
    
    // Weak upgrade fails after all strong refs dropped
    match weak.upgrade() {
        Some(s) => println!("Upgraded: {}", s),
        None => println!("Value was dropped (weak now invalid)"),
    }
    
    // Use case: Breaking cycles
    #[derive(Debug)]
    struct Parent {
        children: Vec<Rc<Child>>,
    }
    
    #[derive(Debug)]
    struct Child {
        parent: Weak<Parent>,
        value: i32,
    }
    
    let parent = Rc::new(Parent { children: vec![] });
    let child = Rc::new(Child {
        parent: Rc::downgrade(&parent),
        value: 42,
    });
    
    println!("Child value: {}", child.value);
    println!("Parent strong count: {}", Rc::strong_count(&parent));
    println!("Parent weak count: {}", Rc::weak_count(&parent));
    
    // Child can access parent
    if let Some(p) = child.parent.upgrade() {
        println!("Child accessed parent with {} children", p.children.len());
    }
    
    println!();
}

/// Interior mutability with Rc/Arc
fn interior_mutability() {
    println!("--- Interior Mutability ---");
    
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::sync::{Arc, Mutex};
    
    // Rc with RefCell for single-threaded mutation
    println!("-- Rc + RefCell --");
    
    let data = Rc::new(RefCell::new(vec![1, 2, 3]));
    
    let data2 = Rc::clone(&data);
    let data3 = Rc::clone(&data);
    
    // Mutate through shared reference
    data2.borrow_mut().push(4);
    println!("After mutation: {:?}", data.borrow());
    
    data3.borrow_mut().push(5);
    println!("After another mutation: {:?}", data.borrow());
    
    // All references see the changes
    println!("data: {:?}", data.borrow());
    println!("data2: {:?}", data2.borrow());
    println!("data3: {:?}", data3.borrow());
    
    // Arc with Mutex for multi-threaded mutation
    println!("\n-- Arc + Mutex --");
    
    use std::thread;
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Counter after 10 threads: {}", *counter.lock().unwrap());
    
    // Shared mutable data structure
    let shared_map = Arc::new(Mutex::new(std::collections::HashMap::new()));
    let mut workers = vec![];
    
    for i in 0..3 {
        let map = Arc::clone(&shared_map);
        let worker = thread::spawn(move || {
            let mut m = map.lock().unwrap();
            m.insert(format!("key{}", i), i * 10);
        });
        workers.push(worker);
    }
    
    for worker in workers {
        worker.join().unwrap();
    }
    
    println!("Shared map: {:?}", shared_map.lock().unwrap());
    
    println!();
}
