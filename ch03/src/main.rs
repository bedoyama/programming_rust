fn main() {
    println!("=== Chapter 3: Basic Types ===\n");

    fixed_width_numeric_types();
    integer_types();
    checked_arithmetic();
    wrapping_arithmetic();
    saturating_arithmetic();
    overflowing_arithmetic();
    floating_point_types();
    bool_type();
    character_type();
    tuple_type();
    pointer_types();
    array_examples();
    vector_examples();
    slice_examples();
    string_examples();
    type_aliases();
}

/// Fixed-Width Numeric Types
fn fixed_width_numeric_types() {
    println!("--- Fixed-Width Numeric Types ---");

    // Signed integers: i8, i16, i32, i64, i128
    let byte: i8 = -128;
    let short: i16 = 32_767;
    let int: i32 = 2_147_483_647;
    let long: i64 = 9_223_372_036_854_775_807;
    let huge: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;

    println!(
        "i8: {}, i16: {}, i32: {}, i64: {}, i128: {}",
        byte, short, int, long, huge
    );

    // Unsigned integers: u8, u16, u32, u64, u128
    let ubyte: u8 = 255;
    let ushort: u16 = 65_535;
    let uint: u32 = 4_294_967_295;
    let ulong: u64 = 18_446_744_073_709_551_615;

    println!(
        "u8: {}, u16: {}, u32: {}, u64: {}",
        ubyte, ushort, uint, ulong
    );

    // Architecture-dependent sizes
    let arch_signed: isize = -9223372036854775808; // 64-bit on 64-bit systems
    let arch_unsigned: usize = 18446744073709551615; // 64-bit on 64-bit systems

    println!("isize: {}, usize: {}", arch_signed, arch_unsigned);
    println!();
}

/// Integer Types - various representations
fn integer_types() {
    println!("--- Integer Types ---");

    // Different number representations
    let decimal = 1_000_000;
    let hex = 0xff; // 255
    let octal = 0o77; // 63
    let binary = 0b1111_0000; // 240
    let byte_literal = b'A'; // u8: 65

    println!("Decimal: {}", decimal);
    println!("Hex 0xff: {}", hex);
    println!("Octal 0o77: {}", octal);
    println!("Binary 0b1111_0000: {}", binary);
    println!("Byte literal b'A': {}", byte_literal);

    // Type suffixes
    let typed_int = 42i32;
    let typed_uint = 100u8;

    println!("42i32: {}, 100u8: {}", typed_int, typed_uint);
    println!();
}

/// Checked Arithmetic - returns Option<T>
fn checked_arithmetic() {
    println!("--- Checked Arithmetic ---");

    let x: i32 = 1_000_000;
    let y: i32 = 1_000;

    // Checked operations return Option
    match x.checked_mul(y) {
        Some(result) => println!("1_000_000 * 1_000 = {}", result),
        None => println!("Overflow occurred!"),
    }

    // This will overflow
    let max = i32::MAX;
    match max.checked_add(1) {
        Some(result) => println!("i32::MAX + 1 = {}", result),
        None => println!("i32::MAX + 1 = Overflow!"),
    }

    // Checked division
    let a: i32 = 10;
    let b: i32 = 0;
    match a.checked_div(b) {
        Some(result) => println!("10 / 0 = {}", result),
        None => println!("10 / 0 = Division by zero!"),
    }
    println!();
}

/// Wrapping Arithmetic - wraps on overflow
fn wrapping_arithmetic() {
    println!("--- Wrapping Arithmetic ---");

    let max = u8::MAX; // 255
    let wrapped = max.wrapping_add(1);
    println!("u8::MAX (255) + 1 wrapping = {}", wrapped); // 0

    let min = i8::MIN; // -128
    let wrapped_sub = min.wrapping_sub(1);
    println!("i8::MIN (-128) - 1 wrapping = {}", wrapped_sub); // 127

    let wrapped_mul = 200u8.wrapping_mul(2);
    println!("200u8 * 2 wrapping = {}", wrapped_mul); // 144
    println!();
}

/// Saturating Arithmetic - clamps at min/max
fn saturating_arithmetic() {
    println!("--- Saturating Arithmetic ---");

    let max = u8::MAX; // 255
    let saturated = max.saturating_add(10);
    println!("u8::MAX (255) + 10 saturating = {}", saturated); // 255

    let min = i8::MIN; // -128
    let saturated_sub = min.saturating_sub(10);
    println!("i8::MIN (-128) - 10 saturating = {}", saturated_sub); // -128

    let saturated_mul = 200u8.saturating_mul(2);
    println!("200u8 * 2 saturating = {}", saturated_mul); // 255
    println!();
}

/// Overflowing Arithmetic - returns (result, bool)
fn overflowing_arithmetic() {
    println!("--- Overflowing Arithmetic ---");

    let max = u8::MAX; // 255
    let (result, overflowed) = max.overflowing_add(10);
    println!(
        "u8::MAX (255) + 10 = {}, overflowed: {}",
        result, overflowed
    );

    let (result, overflowed) = 100u8.overflowing_add(50);
    println!("100u8 + 50 = {}, overflowed: {}", result, overflowed);

    let (result, overflowed) = 200u8.overflowing_mul(2);
    println!("200u8 * 2 = {}, overflowed: {}", result, overflowed);
    println!();
}

/// Floating-Point Types
fn floating_point_types() {
    println!("--- Floating-Point Types ---");

    let f32_num: f32 = 3.14159;
    let f64_num: f64 = 2.718281828459045;

    println!("f32: {}, f64: {}", f32_num, f64_num);

    // Scientific notation
    let scientific = 6.022e23;
    println!("Avogadro's number: {}", scientific);

    // Special values
    println!("Infinity: {}", f64::INFINITY);
    println!("Negative infinity: {}", f64::NEG_INFINITY);
    println!("NaN: {}", f64::NAN);

    // Checking for special values
    let nan = f64::NAN;
    println!("Is NaN? {}", nan.is_nan());
    println!("Is finite? {}", f32_num.is_finite());
    println!();
}

/// The bool Type
fn bool_type() {
    println!("--- bool Type ---");

    let is_true: bool = true;
    let is_false = false;

    println!("true: {}, false: {}", is_true, is_false);

    // Boolean operations
    println!("true && false = {}", is_true && is_false);
    println!("true || false = {}", is_true || is_false);
    println!("!true = {}", !is_true);

    // Comparisons return bool
    let x = 10;
    let y = 20;
    println!("{} < {} = {}", x, y, x < y);
    println!("{} == {} = {}", x, y, x == y);

    // bool as integer (explicit cast)
    println!("true as u8 = {}", is_true as u8); // 1
    println!("false as u8 = {}", is_false as u8); // 0
    println!();
}

/// Characters
fn character_type() {
    println!("--- Characters ---");

    let letter: char = 'A';
    let emoji: char = '😊';
    let japanese: char = 'あ';
    let unicode: char = '\u{1F60E}'; // 😎

    println!(
        "Letter: {}, Emoji: {}, Japanese: {}, Unicode: {}",
        letter, emoji, japanese, unicode
    );

    // char is 4 bytes (32-bit Unicode scalar value)
    println!("Size of char: {} bytes", std::mem::size_of::<char>());

    // Character methods
    println!("Is 'A' alphabetic? {}", letter.is_alphabetic());
    println!("Is '5' numeric? {}", '5'.is_numeric());
    println!("To uppercase: {}", 'a'.to_uppercase());
    println!();
}

/// Tuples
fn tuple_type() {
    println!("--- Tuples ---");

    // Creating tuples
    let tuple: (i32, f64, char) = (42, 3.14, 'x');
    println!("Tuple: {:?}", tuple);

    // Accessing tuple elements
    println!(
        "First: {}, Second: {}, Third: {}",
        tuple.0, tuple.1, tuple.2
    );

    // Destructuring
    let (x, y, z) = tuple;
    println!("Destructured: x={}, y={}, z={}", x, y, z);

    // Nested tuples
    let nested = ((1, 2), (3, 4));
    println!("Nested tuple: {:?}", nested);

    // Unit type - empty tuple
    let unit: () = ();
    println!("Unit type: {:?}", unit);

    // Function returning multiple values
    let (min, max) = find_min_max(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("Min: {}, Max: {}", min, max);
    println!();
}

fn find_min_max(values: &[i32]) -> (i32, i32) {
    let mut min = values[0];
    let mut max = values[0];
    for &val in values {
        if val < min {
            min = val;
        }
        if val > max {
            max = val;
        }
    }
    (min, max)
}

/// Pointer Types
fn pointer_types() {
    println!("--- Pointer Types ---");

    // References (borrowed pointers)
    let x = 42;
    let reference = &x;
    println!("Value: {}, Reference: {}", x, reference);
    println!("Dereferenced: {}", *reference);

    // Mutable references
    let mut y = 10;
    let mut_ref = &mut y;
    *mut_ref += 5;
    println!("Modified through mutable reference: {}", y);

    // Box - heap allocation
    let boxed = Box::new(100);
    println!("Boxed value: {}", boxed);
    println!("Box on heap, size: {} bytes", std::mem::size_of_val(&boxed));

    // Raw pointers (unsafe)
    let raw_ptr: *const i32 = &x;
    unsafe {
        println!("Raw pointer value: {}", *raw_ptr);
    }

    println!();
}

/// Arrays
fn array_examples() {
    println!("--- Arrays ---");

    // Fixed-size arrays
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);

    // Array with repeated values
    let zeros = [0; 10];
    println!("Ten zeros: {:?}", zeros);

    // Accessing elements
    println!("First element: {}", array[0]);
    println!("Last element: {}", array[4]);

    // Array length
    println!("Length: {}", array.len());

    // Iterating
    print!("Elements: ");
    for elem in &array {
        print!("{} ", elem);
    }
    println!("\n");
}

/// Vectors
fn vector_examples() {
    println!("--- Vectors ---");

    // Creating vectors
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vector: {:?}", vec);

    // vec! macro
    let vec2 = vec![10, 20, 30, 40, 50];
    println!("Vec from macro: {:?}", vec2);

    // Accessing elements
    println!("First: {}", vec2[0]);
    println!("Third: {}", vec2[2]);

    // Safe access with get()
    match vec2.get(10) {
        Some(value) => println!("Value at index 10: {}", value),
        None => println!("Index 10 out of bounds"),
    }

    // Vector operations
    println!("Length: {}", vec2.len());
    println!("Capacity: {}", vec2.capacity());

    // Iterating
    print!("Elements: ");
    for elem in &vec2 {
        print!("{} ", elem);
    }
    println!();

    // Vector with initial capacity
    let mut vec3 = Vec::with_capacity(10);
    println!(
        "Empty vec with capacity 10: len={}, cap={}",
        vec3.len(),
        vec3.capacity()
    );
    vec3.push(1);
    println!("After push: len={}, cap={}", vec3.len(), vec3.capacity());
    println!();
}

/// Slices
fn slice_examples() {
    println!("--- Slices ---");

    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Creating slices
    let slice = &array[2..5]; // elements 3, 4, 5
    println!("Slice [2..5]: {:?}", slice);

    let slice_from = &array[5..]; // from index 5 to end
    println!("Slice [5..]: {:?}", slice_from);

    let slice_to = &array[..3]; // from start to index 3
    println!("Slice [..3]: {:?}", slice_to);

    let full_slice = &array[..]; // entire array as slice
    println!("Full slice: {:?}", full_slice);

    // Slice from vector
    let vec = vec![10, 20, 30, 40, 50];
    let vec_slice = &vec[1..4];
    println!("Vector slice [1..4]: {:?}", vec_slice);

    // Passing slices to functions
    println!("Sum of array slice: {}", sum_slice(&array[0..5]));
    println!();
}

fn sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

/// String Types
fn string_examples() {
    println!("--- String Types ---");

    // String literals
    let literal = "Hello, world!";
    println!("String literal: {}", literal);
    println!(
        "Type: &str, size: {} bytes",
        std::mem::size_of_val(&literal)
    );

    // Raw strings
    let raw = r"C:\Users\name\file.txt";
    println!("Raw string: {}", raw);

    let raw_multiline = r#"
        This is a raw string
        with "quotes" and
        multiple lines
    "#;
    println!("Raw multiline:{}", raw_multiline);

    // Byte strings
    let byte_string: &[u8] = b"Hello";
    println!("Byte string: {:?}", byte_string);

    // String type (owned, growable)
    let mut string = String::from("Hello");
    string.push_str(", world!");
    println!("String: {}", string);

    // String methods
    println!("Length: {}", string.len());
    println!("Is empty? {}", string.is_empty());
    println!("Contains 'world'? {}", string.contains("world"));

    // String concatenation
    let s1 = String::from("Hello");
    let s2 = String::from(" world");
    let s3 = s1 + &s2; // s1 is moved
    println!("Concatenated: {}", s3);

    // format! macro
    let formatted = format!("{} + {} = {}", 10, 20, 10 + 20);
    println!("Formatted: {}", formatted);

    // Conversion between String and &str
    let string_to_str: &str = &string;
    let str_to_string: String = literal.to_string();
    println!(
        "Conversions - &str: {}, String: {}",
        string_to_str, str_to_string
    );

    // String slicing (careful with UTF-8!)
    let hello = "Hello";
    let slice = &hello[0..4];
    println!("String slice: {}", slice);

    // Iterating over characters
    print!("Characters: ");
    for c in "Rust".chars() {
        print!("{} ", c);
    }
    println!();

    // Iterating over bytes
    print!("Bytes: ");
    for b in "Hi".bytes() {
        print!("{} ", b);
    }
    println!("\n");
}

/// Type Aliases
fn type_aliases() {
    println!("--- Type Aliases ---");

    // Creating type aliases
    type Kilometers = i32;
    type Meters = i32;

    let distance_km: Kilometers = 5;
    let distance_m: Meters = 5000;

    println!("Distance in km: {}", distance_km);
    println!("Distance in m: {}", distance_m);

    // Note: type aliases don't create new types, just new names
    // This compiles (not type-safe):
    let sum = distance_km + distance_m;
    println!("Sum (no type safety): {}", sum);

    // Complex type aliases
    type ResultStr = Result<String, std::io::Error>;
    type Point = (f64, f64);
    type Matrix = Vec<Vec<f64>>;

    let point: Point = (3.5, 7.2);
    println!("Point: ({}, {})", point.0, point.1);

    let matrix: Matrix = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    println!("Matrix: {:?}", matrix);

    println!();
}
