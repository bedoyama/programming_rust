// Advanced examples for numeric operations

fn main() {
    println!("=== Advanced Numeric Examples ===\n");
    
    overflow_behavior();
    numeric_conversions();
    bitwise_operations();
    method_chaining();
}

/// Demonstrating overflow behavior in debug vs release
fn overflow_behavior() {
    println!("--- Overflow Behavior ---");
    
    // In debug mode, this would panic
    // In release mode, it wraps
    let x: u8 = 255;
    println!("Starting value: {}", x);
    
    // Safe alternatives:
    println!("Checked add: {:?}", x.checked_add(1));
    println!("Saturating add: {}", x.saturating_add(1));
    println!("Wrapping add: {}", x.wrapping_add(1));
    println!("Overflowing add: {:?}", x.overflowing_add(1));
    println!();
}

/// Type conversions and casting
fn numeric_conversions() {
    println!("--- Numeric Conversions ---");
    
    // Explicit casting with 'as'
    let x: i32 = 1000;
    let y: i16 = x as i16;
    let z: u8 = 255;
    let w: i32 = z as i32;
    
    println!("i32 {} as i16: {}", x, y);
    println!("u8 {} as i32: {}", z, w);
    
    // Truncation
    let large: u32 = 65_537;
    let truncated: u8 = large as u8; // loses high bits
    println!("u32 {} as u8 (truncated): {}", large, truncated);
    
    // Float to int (truncates decimal)
    let float = 3.99;
    let int = float as i32;
    println!("f64 {} as i32: {}", float, int);
    
    // From/Into traits for fallible conversions
    let value: i32 = 100;
    let converted: Result<i8, _> = value.try_into();
    match converted {
        Ok(v) => println!("Converted {} to i8: {}", value, v),
        Err(_) => println!("Conversion failed"),
    }
    
    println!();
}

/// Bitwise operations
fn bitwise_operations() {
    println!("--- Bitwise Operations ---");
    
    let a: u8 = 0b1010_1010; // 170
    let b: u8 = 0b1100_1100; // 204
    
    println!("a = {:08b} ({})", a, a);
    println!("b = {:08b} ({})", b, b);
    println!();
    
    println!("a & b  = {:08b} ({})", a & b, a & b);   // AND
    println!("a | b  = {:08b} ({})", a | b, a | b);   // OR
    println!("a ^ b  = {:08b} ({})", a ^ b, a ^ b);   // XOR
    println!("!a     = {:08b} ({})", !a, !a);         // NOT
    println!("a << 2 = {:08b} ({})", a << 2, a << 2); // Left shift
    println!("a >> 2 = {:08b} ({})", a >> 2, a >> 2); // Right shift
    
    println!();
}

/// Method chaining with numeric types
fn method_chaining() {
    println!("--- Method Chaining ---");
    
    let result = 10_i32
        .saturating_mul(100)
        .saturating_add(50)
        .saturating_sub(25)
        .max(0)
        .min(1000);
    
    println!("Chained operations result: {}", result);
    
    // Working with ranges
    let sum: i32 = (1..=10).sum();
    println!("Sum of 1..=10: {}", sum);
    
    let product: i32 = (1..=5).product();
    println!("Product of 1..=5: {}", product);
    
    println!();
}
