// Comprehensive operator examples

fn main() {
    println!("=== Operators ===\n");
    
    arithmetic_operators();
    bitwise_operators();
    comparison_operators();
    logical_operators();
    compound_assignment();
    operator_precedence();
    operator_overloading();
}

/// Arithmetic operators
fn arithmetic_operators() {
    println!("--- Arithmetic Operators ---");
    
    // Basic arithmetic
    println!("Addition: 5 + 3 = {}", 5 + 3);
    println!("Subtraction: 5 - 3 = {}", 5 - 3);
    println!("Multiplication: 5 * 3 = {}", 5 * 3);
    println!("Division: 10 / 3 = {}", 10 / 3);
    println!("Remainder: 10 % 3 = {}", 10 % 3);
    
    // Floating point
    println!("\nFloating point:");
    println!("5.0 / 3.0 = {}", 5.0 / 3.0);
    println!("10.5 % 3.2 = {}", 10.5 % 3.2);
    
    // Negation
    let x = 5;
    println!("\nNegation: -{} = {}", x, -x);
    
    // Integer division truncates
    println!("\nInteger division:");
    println!("7 / 2 = {}", 7 / 2);
    println!("-7 / 2 = {}", -7 / 2);
    
    // Overflow behavior (debug mode panics, release wraps)
    println!("\nOverflow (in release mode):");
    let max = std::i32::MAX;
    println!("Max i32: {}", max);
    // println!("Max + 1: {}", max + 1); // Would panic in debug
    println!("Max.wrapping_add(1): {}", max.wrapping_add(1));
    println!("Max.saturating_add(1): {}", max.saturating_add(1));
    
    // Checked arithmetic
    println!("\nChecked arithmetic:");
    match 10i32.checked_div(2) {
        Some(result) => println!("10 / 2 = {}", result),
        None => println!("Division failed"),
    }
    
    match 10i32.checked_div(0) {
        Some(result) => println!("10 / 0 = {}", result),
        None => println!("Cannot divide by zero"),
    }
    
    println!();
}

/// Bitwise operators
fn bitwise_operators() {
    println!("--- Bitwise Operators ---");
    
    let a = 0b1100;  // 12
    let b = 0b1010;  // 10
    
    println!("a = {:04b} ({})", a, a);
    println!("b = {:04b} ({})", b, b);
    println!();
    
    // AND
    println!("AND: {:04b} & {:04b} = {:04b} ({})", a, b, a & b, a & b);
    
    // OR
    println!("OR:  {:04b} | {:04b} = {:04b} ({})", a, b, a | b, a | b);
    
    // XOR
    println!("XOR: {:04b} ^ {:04b} = {:04b} ({})", a, b, a ^ b, a ^ b);
    
    // NOT
    let c: u8 = 0b00001111;
    println!("NOT: !{:08b} = {:08b} ({})", c, !c, !c);
    
    // Left shift
    println!("\nShift left:");
    println!("1 << 0 = {}", 1 << 0);
    println!("1 << 1 = {}", 1 << 1);
    println!("1 << 2 = {}", 1 << 2);
    println!("1 << 3 = {}", 1 << 3);
    
    // Right shift
    println!("\nShift right:");
    println!("16 >> 0 = {}", 16 >> 0);
    println!("16 >> 1 = {}", 16 >> 1);
    println!("16 >> 2 = {}", 16 >> 2);
    println!("16 >> 3 = {}", 16 >> 3);
    
    // Bit manipulation examples
    println!("\nBit manipulation:");
    let mut flags: u8 = 0;
    
    // Set bit
    flags |= 1 << 2;  // Set bit 2
    println!("Set bit 2: {:08b}", flags);
    
    // Clear bit
    flags &= !(1 << 2);  // Clear bit 2
    println!("Clear bit 2: {:08b}", flags);
    
    // Toggle bit
    flags ^= 1 << 3;  // Toggle bit 3
    println!("Toggle bit 3: {:08b}", flags);
    
    // Check bit
    flags = 0b00001000;
    let is_set = (flags & (1 << 3)) != 0;
    println!("Bit 3 is set: {}", is_set);
    
    println!();
}

/// Comparison operators
fn comparison_operators() {
    println!("--- Comparison Operators ---");
    
    // Numeric comparisons
    println!("5 == 5: {}", 5 == 5);
    println!("5 != 3: {}", 5 != 3);
    println!("5 > 3: {}", 5 > 3);
    println!("5 < 3: {}", 5 < 3);
    println!("5 >= 5: {}", 5 >= 5);
    println!("5 <= 3: {}", 5 <= 3);
    
    // String comparisons
    println!("\nString comparisons:");
    let s1 = "hello";
    let s2 = "world";
    println!("\"{}\" == \"{}\": {}", s1, s1, s1 == s1);
    println!("\"{}\" < \"{}\": {}", s1, s2, s1 < s2);
    
    // Tuple comparisons (lexicographic)
    println!("\nTuple comparisons:");
    println!("(1, 2) < (1, 3): {}", (1, 2) < (1, 3));
    println!("(1, 2) < (2, 0): {}", (1, 2) < (2, 0));
    
    // Option comparisons
    println!("\nOption comparisons:");
    println!("Some(5) == Some(5): {}", Some(5) == Some(5));
    println!("Some(5) > Some(3): {}", Some(5) > Some(3));
    println!("Some(5) > None: {}", Some(5) > None::<i32>);
    
    // Floating point special values
    println!("\nFloating point:");
    let nan = f64::NAN;
    println!("NaN == NaN: {}", nan == nan);  // false!
    println!("NaN != NaN: {}", nan != nan);  // true!
    
    // Comparison chaining doesn't work
    // println!("1 < 2 < 3: {}", 1 < 2 < 3);  // ERROR
    println!("1 < 2 && 2 < 3: {}", 1 < 2 && 2 < 3);  // Correct
    
    println!();
}

/// Logical operators
fn logical_operators() {
    println!("--- Logical Operators ---");
    
    // Boolean AND
    println!("true && true: {}", true && true);
    println!("true && false: {}", true && false);
    println!("false && true: {}", false && true);
    println!("false && false: {}", false && false);
    
    // Boolean OR
    println!("\nOR:");
    println!("true || true: {}", true || true);
    println!("true || false: {}", true || false);
    println!("false || true: {}", false || true);
    println!("false || false: {}", false || false);
    
    // Boolean NOT
    println!("\nNOT:");
    println!("!true: {}", !true);
    println!("!false: {}", !false);
    
    // Short-circuit evaluation
    println!("\nShort-circuit evaluation:");
    
    fn expensive_true() -> bool {
        println!("  expensive_true called");
        true
    }
    
    fn expensive_false() -> bool {
        println!("  expensive_false called");
        false
    }
    
    println!("true || expensive_true():");
    let _ = true || expensive_true();  // expensive_true not called
    
    println!("\nfalse && expensive_false():");
    let _ = false && expensive_false();  // expensive_false not called
    
    println!("\nfalse || expensive_true():");
    let _ = false || expensive_true();  // expensive_true IS called
    
    println!("\ntrue && expensive_true():");
    let _ = true && expensive_true();  // expensive_true IS called
    
    // Complex conditions
    println!("\nComplex conditions:");
    let x = 5;
    let y = 10;
    println!("x > 0 && y > 0: {}", x > 0 && y > 0);
    println!("x < 0 || y > 5: {}", x < 0 || y > 5);
    println!("!(x > 10): {}", !(x > 10));
    
    println!();
}

/// Compound assignment operators
fn compound_assignment() {
    println!("--- Compound Assignment ---");
    
    // Arithmetic compound assignment
    let mut x = 10;
    println!("Starting x: {}", x);
    
    x += 5;
    println!("After x += 5: {}", x);
    
    x -= 3;
    println!("After x -= 3: {}", x);
    
    x *= 2;
    println!("After x *= 2: {}", x);
    
    x /= 4;
    println!("After x /= 4: {}", x);
    
    x %= 5;
    println!("After x %= 5: {}", x);
    
    // Bitwise compound assignment
    println!("\nBitwise compound:");
    let mut flags: u8 = 0b1100;
    println!("Starting: {:08b}", flags);
    
    flags |= 0b0011;
    println!("After |= 0b0011: {:08b}", flags);
    
    flags &= 0b1010;
    println!("After &= 0b1010: {:08b}", flags);
    
    flags ^= 0b0101;
    println!("After ^= 0b0101: {:08b}", flags);
    
    flags <<= 2;
    println!("After <<= 2: {:08b}", flags);
    
    flags >>= 1;
    println!("After >>= 1: {:08b}", flags);
    
    // String compound
    let mut text = String::from("Hello");
    text += " World";
    println!("\nString after +=: {}", text);
    
    println!();
}

/// Operator precedence
fn operator_precedence() {
    println!("--- Operator Precedence ---");
    
    // Precedence examples
    println!("2 + 3 * 4 = {} (not {})", 2 + 3 * 4, (2 + 3) * 4);
    println!("10 - 5 - 2 = {} (left-to-right)", 10 - 5 - 2);
    println!("2 * 3 + 4 * 5 = {}", 2 * 3 + 4 * 5);
    
    // Explicit parentheses
    println!("\nWith parentheses:");
    println!("(2 + 3) * 4 = {}", (2 + 3) * 4);
    println!("2 + (3 * 4) = {}", 2 + (3 * 4));
    
    // Complex expressions
    println!("\nComplex:");
    println!("5 + 3 * 2 - 1 = {}", 5 + 3 * 2 - 1);
    println!("(5 + 3) * (2 - 1) = {}", (5 + 3) * (2 - 1));
    
    // Comparison precedence
    println!("\nComparison:");
    println!("5 + 3 > 2 * 3 = {}", 5 + 3 > 2 * 3);
    println!("5 > 3 && 2 < 4 = {}", 5 > 3 && 2 < 4);
    
    // Bitwise precedence
    println!("\nBitwise:");
    println!("0b1100 & 0b1010 | 0b0011 = {:04b}", 0b1100 & 0b1010 | 0b0011);
    println!("(0b1100 & 0b1010) | 0b0011 = {:04b}", (0b1100 & 0b1010) | 0b0011);
    
    println!();
}

/// Operator overloading
fn operator_overloading() {
    println!("--- Operator Overloading ---");
    
    use std::ops::{Add, Sub, Mul, Neg};
    
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    // Implement Add
    impl Add for Point {
        type Output = Point;
        
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    
    // Implement Sub
    impl Sub for Point {
        type Output = Point;
        
        fn sub(self, other: Point) -> Point {
            Point {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }
    
    // Implement Mul (scalar)
    impl Mul<i32> for Point {
        type Output = Point;
        
        fn mul(self, scalar: i32) -> Point {
            Point {
                x: self.x * scalar,
                y: self.y * scalar,
            }
        }
    }
    
    // Implement Neg
    impl Neg for Point {
        type Output = Point;
        
        fn neg(self) -> Point {
            Point {
                x: -self.x,
                y: -self.y,
            }
        }
    }
    
    let p1 = Point { x: 3, y: 4 };
    let p2 = Point { x: 1, y: 2 };
    
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    
    let sum = p1 + p2;
    println!("p1 + p2: {:?}", sum);
    
    let diff = p1 - p2;
    println!("p1 - p2: {:?}", diff);
    
    let scaled = p1 * 3;
    println!("p1 * 3: {:?}", scaled);
    
    let negated = -p1;
    println!("-p1: {:?}", negated);
    
    // Chaining operations
    let result = (p1 + p2) * 2 - p1;
    println!("(p1 + p2) * 2 - p1: {:?}", result);
    
    println!();
}
