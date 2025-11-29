// Advanced string handling examples

fn main() {
    println!("=== Advanced String Examples ===\n");
    
    string_memory_layout();
    string_manipulation();
    string_searching();
    unicode_handling();
    string_formatting();
}

/// Understanding string memory layout
fn string_memory_layout() {
    println!("--- String Memory Layout ---");
    
    // &str - string slice (pointer + length)
    let str_slice: &str = "Hello";
    println!("&str size: {} bytes", std::mem::size_of::<&str>()); // 16 bytes on 64-bit
    println!("&str points to: {:?}", str_slice);
    
    // String - owned string (pointer + length + capacity)
    let string = String::from("Hello");
    println!("String size: {} bytes", std::mem::size_of::<String>()); // 24 bytes on 64-bit
    println!("String length: {}", string.len());
    println!("String capacity: {}", string.capacity());
    
    // String is stored on the heap
    let mut heap_string = String::with_capacity(100);
    println!("Empty String with capacity 100:");
    println!("  Length: {}", heap_string.len());
    println!("  Capacity: {}", heap_string.capacity());
    
    heap_string.push_str("Hello");
    println!("After adding 'Hello':");
    println!("  Length: {}", heap_string.len());
    println!("  Capacity: {}", heap_string.capacity());
    
    println!();
}

/// String manipulation techniques
fn string_manipulation() {
    println!("--- String Manipulation ---");
    
    let mut s = String::from("Hello");
    
    // Appending
    s.push(' ');
    s.push_str("world");
    s.push('!');
    println!("After appending: {}", s);
    
    // Inserting
    let mut s2 = String::from("Hello world");
    s2.insert(5, ',');
    s2.insert_str(6, " beautiful");
    println!("After inserting: {}", s2);
    
    // Replacing
    let s3 = s2.replace("beautiful", "wonderful");
    println!("After replace: {}", s3);
    
    // Removing
    let mut s4 = String::from("Hello, world!");
    s4.remove(5); // removes comma
    println!("After remove: {}", s4);
    
    // Truncating
    let mut s5 = String::from("Hello, world!");
    s5.truncate(5);
    println!("After truncate: {}", s5);
    
    // Clearing
    let mut s6 = String::from("Hello");
    s6.clear();
    println!("After clear: '{}' (empty: {})", s6, s6.is_empty());
    
    // Trimming
    let s7 = "  whitespace  ".trim();
    println!("Trimmed: '{}'", s7);
    
    let s8 = "###Hello###".trim_matches('#');
    println!("Trim matches: '{}'", s8);
    
    println!();
}

/// String searching and pattern matching
fn string_searching() {
    println!("--- String Searching ---");
    
    let text = "The quick brown fox jumps over the lazy dog";
    
    // Contains
    println!("Contains 'fox'? {}", text.contains("fox"));
    println!("Contains 'cat'? {}", text.contains("cat"));
    
    // Starts with / ends with
    println!("Starts with 'The'? {}", text.starts_with("The"));
    println!("Ends with 'dog'? {}", text.ends_with("dog"));
    
    // Finding
    match text.find("fox") {
        Some(index) => println!("'fox' found at index {}", index),
        None => println!("'fox' not found"),
    }
    
    // Finding from the end
    match text.rfind("the") {
        Some(index) => println!("'the' last found at index {}", index),
        None => println!("'the' not found"),
    }
    
    // Matching patterns
    println!("Matches 'quick'? {}", text.matches("quick").count());
    
    // Splitting
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("Words: {:?}", words);
    
    let parts: Vec<&str> = "one,two,three".split(',').collect();
    println!("Split by comma: {:?}", parts);
    
    // Lines
    let multiline = "line 1\nline 2\nline 3";
    for (i, line) in multiline.lines().enumerate() {
        println!("Line {}: {}", i + 1, line);
    }
    
    println!();
}

/// Unicode handling
fn unicode_handling() {
    println!("--- Unicode Handling ---");
    
    let s = "Hello 世界 🦀";
    
    // Length in bytes vs characters
    println!("String: '{}'", s);
    println!("Byte length: {}", s.len());
    println!("Character count: {}", s.chars().count());
    
    // Iterating over characters
    print!("Characters: ");
    for c in s.chars() {
        print!("'{}' ", c);
    }
    println!();
    
    // Iterating over bytes
    print!("Bytes: ");
    for b in s.bytes() {
        print!("{:02x} ", b);
    }
    println!();
    
    // Character indices
    println!("Character indices:");
    for (i, c) in s.char_indices() {
        println!("  Index {}: '{}'", i, c);
    }
    
    // Safe slicing with char_indices
    let emoji = "👨‍👩‍👧‍👦";
    println!("Complex emoji: {} (bytes: {})", emoji, emoji.len());
    
    // Grapheme clusters (requires external crate)
    println!("Note: For proper grapheme handling, use unicode-segmentation crate");
    
    println!();
}

/// String formatting
fn string_formatting() {
    println!("--- String Formatting ---");
    
    let name = "Alice";
    let age = 30;
    let height = 1.65;
    
    // Basic formatting
    let s1 = format!("Name: {}, Age: {}, Height: {}", name, age, height);
    println!("{}", s1);
    
    // Named parameters
    let s2 = format!("{name} is {age} years old", name = name, age = age);
    println!("{}", s2);
    
    // Positional parameters
    let s3 = format!("{0} is {1} years old. {0} is {2}m tall.", name, age, height);
    println!("{}", s3);
    
    // Padding and alignment
    println!("Left align:  |{:<10}|", "text");
    println!("Right align: |{:>10}|", "text");
    println!("Center:      |{:^10}|", "text");
    println!("Zero pad:    |{:0>10}|", 42);
    
    // Number formatting
    println!("Binary:      {:b}", 42);
    println!("Octal:       {:o}", 42);
    println!("Hex lower:   {:x}", 42);
    println!("Hex upper:   {:X}", 42);
    println!("Float:       {:.2}", 3.14159);
    println!("Scientific:  {:e}", 1000.0);
    
    // Debug formatting
    let vec = vec![1, 2, 3];
    println!("Debug:       {:?}", vec);
    println!("Pretty:      {:#?}", vec);
    
    println!();
}
