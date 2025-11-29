// Advanced collections examples: arrays, vectors, slices

fn main() {
    println!("=== Advanced Collections Examples ===\n");
    
    array_operations();
    vector_operations();
    slice_operations();
    collection_conversions();
}

/// Advanced array operations
fn array_operations() {
    println!("--- Array Operations ---");
    
    let array = [1, 2, 3, 4, 5];
    
    // Iteration methods
    println!("Array: {:?}", array);
    
    // iter() - immutable iteration
    let sum: i32 = array.iter().sum();
    println!("Sum: {}", sum);
    
    // Map and collect
    let doubled: Vec<i32> = array.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    // Filter
    let evens: Vec<&i32> = array.iter().filter(|x| *x % 2 == 0).collect();
    println!("Evens: {:?}", evens);
    
    // Array methods
    println!("First: {:?}", array.first());
    println!("Last: {:?}", array.last());
    println!("Contains 3? {}", array.contains(&3));
    
    // Sorting (requires mut)
    let mut sortable = [5, 2, 8, 1, 9];
    sortable.sort();
    println!("Sorted: {:?}", sortable);
    
    // Reversing
    let mut reversible = [1, 2, 3, 4, 5];
    reversible.reverse();
    println!("Reversed: {:?}", reversible);
    
    // Splitting
    let (left, right) = array.split_at(2);
    println!("Split at 2: left={:?}, right={:?}", left, right);
    
    // Windows (sliding window)
    print!("Windows of size 3: ");
    for window in array.windows(3) {
        print!("{:?} ", window);
    }
    println!();
    
    // Chunks
    print!("Chunks of size 2: ");
    for chunk in array.chunks(2) {
        print!("{:?} ", chunk);
    }
    println!("\n");
}

/// Advanced vector operations
fn vector_operations() {
    println!("--- Vector Operations ---");
    
    let mut vec = vec![1, 2, 3, 4, 5];
    
    // Push and pop
    vec.push(6);
    println!("After push(6): {:?}", vec);
    
    if let Some(popped) = vec.pop() {
        println!("Popped: {}", popped);
    }
    println!("After pop: {:?}", vec);
    
    // Insert and remove
    vec.insert(2, 99);
    println!("After insert(2, 99): {:?}", vec);
    
    let removed = vec.remove(2);
    println!("Removed at index 2: {}, vec: {:?}", removed, vec);
    
    // Append (moves elements)
    let mut vec2 = vec![10, 20, 30];
    vec.append(&mut vec2);
    println!("After append: {:?}", vec);
    println!("vec2 after append: {:?}", vec2);
    
    // Extend (clones/copies elements)
    let vec3 = vec![100, 200];
    vec.extend(&vec3);
    println!("After extend: {:?}", vec);
    
    // Retain (filter in place)
    vec.retain(|x| x % 2 == 0);
    println!("After retain (evens only): {:?}", vec);
    
    // Deduplication
    let mut dup_vec = vec![1, 2, 2, 3, 3, 3, 4];
    dup_vec.dedup();
    println!("After dedup: {:?}", dup_vec);
    
    // Splitting off
    let mut vec4 = vec![1, 2, 3, 4, 5, 6];
    let vec5 = vec4.split_off(3);
    println!("After split_off(3): vec4={:?}, vec5={:?}", vec4, vec5);
    
    // Drain (remove and iterate)
    let mut vec6 = vec![1, 2, 3, 4, 5];
    let drained: Vec<i32> = vec6.drain(1..4).collect();
    println!("Drained elements: {:?}, remaining: {:?}", drained, vec6);
    
    // Vector with specific capacity
    let mut vec7 = Vec::with_capacity(10);
    println!("Empty vector - len: {}, capacity: {}", vec7.len(), vec7.capacity());
    
    for i in 0..5 {
        vec7.push(i);
    }
    println!("After 5 pushes - len: {}, capacity: {}", vec7.len(), vec7.capacity());
    
    // Shrinking
    vec7.shrink_to_fit();
    println!("After shrink_to_fit - len: {}, capacity: {}", vec7.len(), vec7.capacity());
    
    println!();
}

/// Advanced slice operations
fn slice_operations() {
    println!("--- Slice Operations ---");
    
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Different slice ranges
    println!("Full slice: {:?}", &array[..]);
    println!("From index 3: {:?}", &array[3..]);
    println!("Up to index 5: {:?}", &array[..5]);
    println!("Range 2..7: {:?}", &array[2..7]);
    println!("Range 2..=7: {:?}", &array[2..=7]); // inclusive
    
    // Slice methods
    let slice = &array[2..8];
    println!("Slice: {:?}", slice);
    println!("First: {:?}", slice.first());
    println!("Last: {:?}", slice.last());
    println!("Length: {}", slice.len());
    println!("Is empty? {}", slice.is_empty());
    
    // Splitting slices
    let (left, right) = slice.split_at(3);
    println!("Split at 3: left={:?}, right={:?}", left, right);
    
    // Finding elements
    if let Some(pos) = slice.iter().position(|&x| x == 5) {
        println!("Found 5 at position {}", pos);
    }
    
    // Binary search (requires sorted slice)
    let sorted = [1, 3, 5, 7, 9, 11, 13, 15];
    match sorted.binary_search(&7) {
        Ok(index) => println!("Found 7 at index {}", index),
        Err(index) => println!("7 not found, would be at index {}", index),
    }
    
    // Slice patterns (destructuring)
    match slice {
        [] => println!("Empty slice"),
        [single] => println!("Single element: {}", single),
        [first, .., last] => println!("First: {}, Last: {}", first, last),
    }
    
    // Working with mutable slices
    let mut mut_array = [1, 2, 3, 4, 5];
    let mut_slice = &mut mut_array[1..4];
    for elem in mut_slice {
        *elem *= 10;
    }
    println!("Modified array: {:?}", mut_array);
    
    // Sorting a slice
    let mut unsorted = [5, 2, 8, 1, 9, 3];
    unsorted.sort();
    println!("Sorted: {:?}", unsorted);
    
    // Sorting by key
    let mut pairs = [(3, "three"), (1, "one"), (2, "two")];
    pairs.sort_by_key(|&(n, _)| n);
    println!("Sorted pairs: {:?}", pairs);
    
    println!();
}

/// Conversions between collection types
fn collection_conversions() {
    println!("--- Collection Conversions ---");
    
    // Array to slice
    let array = [1, 2, 3, 4, 5];
    let slice: &[i32] = &array;
    println!("Array to slice: {:?}", slice);
    
    // Array to Vec
    let vec1: Vec<i32> = array.to_vec();
    println!("Array to Vec (to_vec): {:?}", vec1);
    
    let vec2: Vec<i32> = array.iter().copied().collect();
    println!("Array to Vec (collect): {:?}", vec2);
    
    // Vec to array (if size known at compile time)
    let vec = vec![1, 2, 3, 4, 5];
    let array_from_vec: [i32; 5] = vec.try_into().unwrap();
    println!("Vec to array: {:?}", array_from_vec);
    
    // Vec to slice
    let vec = vec![10, 20, 30];
    let slice_from_vec: &[i32] = &vec;
    println!("Vec to slice: {:?}", slice_from_vec);
    
    // Slice to Vec
    let slice = &[7, 8, 9];
    let vec_from_slice = slice.to_vec();
    println!("Slice to Vec: {:?}", vec_from_slice);
    
    // String conversions
    let string = String::from("hello");
    let str_slice: &str = &string;
    let string_again = str_slice.to_string();
    println!("String <-> &str: {} -> {} -> {}", string, str_slice, string_again);
    
    // Bytes to string
    let bytes = vec![72, 101, 108, 108, 111];
    let from_bytes = String::from_utf8(bytes).unwrap();
    println!("Bytes to String: {}", from_bytes);
    
    // String to bytes
    let string = String::from("World");
    let to_bytes = string.into_bytes();
    println!("String to bytes: {:?}", to_bytes);
    
    println!();
}
