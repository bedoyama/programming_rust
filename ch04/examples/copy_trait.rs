// Copy trait implementation and behavior

fn main() {
    println!("=== Copy Trait Deep Dive ===\n");
    
    copy_semantics();
    implementing_copy();
    copy_vs_clone();
    copy_limitations();
}

/// Understanding Copy semantics
fn copy_semantics() {
    println!("--- Copy Semantics ---");
    
    // Primitive types are Copy
    let x = 5;
    let y = x;
    println!("Both valid - x: {}, y: {}", x, y);
    
    // Copy happens on assignment
    let a = 10;
    let b = a;
    let c = a; // Can copy multiple times
    println!("a: {}, b: {}, c: {}", a, b, c);
    
    // Copy in function calls
    let num = 42;
    takes_copy(num);
    println!("num still valid after function: {}", num);
    
    // Copy in return
    let result = returns_copy();
    println!("Returned copy: {}", result);
    
    // Arrays of Copy types
    let arr = [1, 2, 3, 4, 5];
    let arr2 = arr;
    println!("Both arrays valid: {:?}, {:?}", arr, arr2);
    
    // Tuples of Copy types
    let tuple = (1, 2.5, 'x');
    let tuple2 = tuple;
    println!("Both tuples valid: {:?}, {:?}", tuple, tuple2);
    
    // References are Copy
    let string = String::from("hello");
    let ref1 = &string;
    let ref2 = ref1;
    println!("Both refs valid: {}, {}", ref1, ref2);
    
    println!();
}

fn takes_copy(x: i32) {
    println!("Function received copy: {}", x);
}

fn returns_copy() -> i32 {
    100
}

/// Implementing Copy for custom types
fn implementing_copy() {
    println!("--- Implementing Copy ---");
    
    // Simple struct with Copy
    #[derive(Copy, Clone, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // Copy
    println!("p1: {:?}, p2: {:?}", p1, p2);
    
    // Tuple struct with Copy
    #[derive(Copy, Clone, Debug)]
    struct Color(u8, u8, u8);
    
    let c1 = Color(255, 0, 0);
    let c2 = c1;
    println!("c1: {:?}, c2: {:?}", c1, c2);
    
    // Enum with Copy
    #[derive(Copy, Clone, Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }
    
    let d1 = Direction::North;
    let d2 = d1;
    println!("d1: {:?}, d2: {:?}", d1, d2);
    
    // Struct with generic Copy type
    #[derive(Copy, Clone, Debug)]
    struct Pair<T: Copy> {
        first: T,
        second: T,
    }
    
    let pair1 = Pair { first: 1, second: 2 };
    let pair2 = pair1;
    println!("pair1: {:?}, pair2: {:?}", pair1, pair2);
    
    // Complex Copy type
    #[derive(Copy, Clone, Debug)]
    struct Pixel {
        position: Point,
        color: Color,
    }
    
    let px1 = Pixel {
        position: Point { x: 0, y: 0 },
        color: Color(255, 255, 255),
    };
    let px2 = px1;
    println!("px1: {:?}, px2: {:?}", px1, px2);
    
    println!();
}

/// Copy vs Clone
fn copy_vs_clone() {
    println!("--- Copy vs Clone ---");
    
    // Copy is implicit
    let x = 42;
    let y = x; // Automatic copy
    println!("Copy (implicit): x={}, y={}", x, y);
    
    // Clone is explicit
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Explicit clone
    println!("Clone (explicit): s1={}, s2={}", s1, s2);
    
    // Copy types also have clone
    let a = 10;
    let b = a.clone(); // Explicit, but same as copy
    println!("Copy type cloned: a={}, b={}", a, b);
    
    // Copy is always a cheap bitwise copy
    #[derive(Copy, Clone, Debug)]
    struct Small {
        value: i32,
    }
    
    let small1 = Small { value: 42 };
    let small2 = small1; // Cheap copy
    println!("Cheap copy: {:?}", small2);
    
    // Clone can be expensive
    #[derive(Clone, Debug)]
    struct Large {
        data: Vec<i32>,
    }
    
    let large1 = Large { data: vec![1; 1000] };
    let large2 = large1.clone(); // Expensive: copies 1000 elements
    println!("Expensive clone: {} elements", large2.data.len());
    
    // Copy marker trait
    println!("\nCopy is a marker trait:");
    println!("  - No methods to implement");
    println!("  - Requires Clone");
    println!("  - All fields must be Copy");
    println!("  - Enables implicit copying");
    
    println!();
}

/// Limitations of Copy
fn copy_limitations() {
    println!("--- Copy Limitations ---");
    
    // Cannot implement Copy if:
    
    // 1. Type owns heap data
    // struct NotCopy1 {
    //     data: String, // String is not Copy
    // }
    
    // 2. Type has Drop implementation
    // struct NotCopy2 {
    //     value: i32,
    // }
    // impl Drop for NotCopy2 {
    //     fn drop(&mut self) {}
    // }
    // impl Copy for NotCopy2 {} // ERROR
    
    // 3. Type contains non-Copy fields
    #[derive(Clone, Debug)]
    struct NotCopy3 {
        name: String,  // Not Copy
        age: i32,      // Copy
    }
    
    let nc = NotCopy3 {
        name: String::from("Alice"),
        age: 30,
    };
    
    // Must clone, not copy
    let nc2 = nc.clone();
    // let nc3 = nc; // ERROR: nc was moved
    println!("Cloned non-Copy struct: {:?}", nc2);
    
    // Mixed Copy/non-Copy fields
    #[derive(Clone, Debug)]
    struct Mixed {
        id: i32,           // Copy
        name: String,      // Not Copy
        active: bool,      // Copy
    }
    
    let m1 = Mixed {
        id: 1,
        name: String::from("Test"),
        active: true,
    };
    
    // Whole struct must be cloned
    let m2 = m1.clone();
    println!("Mixed struct: {:?}", m2);
    
    // Copy requirement is transitive
    println!("\nCopy requirements:");
    println!("  ✓ All primitive types");
    println!("  ✓ References (&T)");
    println!("  ✓ Raw pointers (*const T, *mut T)");
    println!("  ✓ Function pointers");
    println!("  ✓ Tuples/arrays of Copy types");
    println!("  ✗ String, Vec, Box, Rc, Arc");
    println!("  ✗ Any type with custom Drop");
    
    // Size matters for Copy
    #[derive(Copy, Clone, Debug)]
    struct Tiny {
        byte: u8,
    }
    
    #[derive(Copy, Clone, Debug)]
    struct Large {
        data: [u8; 1024],
    }
    
    let tiny = Tiny { byte: 1 };
    let large = Large { data: [0; 1024] };
    
    println!("\nSize considerations:");
    println!("  Tiny: {} bytes (cheap to copy)", std::mem::size_of::<Tiny>());
    println!("  Large: {} bytes (still copied by value!)", std::mem::size_of::<Large>());
    
    let tiny2 = tiny;
    let large2 = large;
    println!("Both copied: tiny={:?}, large has {} bytes", tiny2, large2.data.len());
    
    println!();
}
