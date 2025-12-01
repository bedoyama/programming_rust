// Advanced lifetime examples

fn main() {
    println!("=== Advanced Lifetimes ===\n");

    lifetime_basics();
    lifetime_elision();
    lifetime_constraints();
    struct_lifetimes();
    static_lifetime();
}

/// Understanding lifetime annotations
fn lifetime_basics() {
    println!("--- Lifetime Basics ---");

    // Explicit lifetime annotation
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let s1 = String::from("long string");
    let s2 = String::from("short");
    let result = longest(&s1, &s2);
    println!("Longest: {}", result);

    // Lifetime shows relationship between input and output
    fn first_word<'a>(s: &'a str) -> &'a str {
        s.split_whitespace().next().unwrap_or("")
    }

    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("First word: {}", word);

    // Multiple lifetime parameters
    fn combine<'a, 'b>(x: &'a str, y: &'b str) -> String {
        format!("{} {}", x, y)
    }

    let part1 = String::from("hello");
    let part2 = String::from("world");
    let combined = combine(&part1, &part2);
    println!("Combined: {}", combined);

    // Lifetime bounds
    fn print_if_long<'a>(x: &'a str, y: &'a str)
    where
        'a: 'static,
    {
        println!("{} {}", x, y);
    }

    // Works with static strings
    print_if_long("static", "strings");

    println!();
}

/// Lifetime elision rules
fn lifetime_elision() {
    println!("--- Lifetime Elision ---");

    // Rule 1: Each parameter gets its own lifetime
    // Explicit:
    fn rule1_explicit<'a>(x: &'a i32) -> &'a i32 {
        x
    }
    // Elided (compiler infers):
    fn rule1_elided(x: &i32) -> &i32 {
        x
    }

    let num = 42;
    println!("Rule 1 - explicit: {}", rule1_explicit(&num));
    println!("Rule 1 - elided: {}", rule1_elided(&num));

    // Rule 2: If one input lifetime, assigned to all outputs
    // Explicit:
    fn rule2_explicit<'a>(s: &'a str) -> &'a str {
        s
    }
    // Elided:
    fn rule2_elided(s: &str) -> &str {
        s
    }

    let text = "hello";
    println!("Rule 2 - explicit: {}", rule2_explicit(text));
    println!("Rule 2 - elided: {}", rule2_elided(text));

    // Rule 3: Method with &self or &mut self
    struct Data<'a> {
        value: &'a str,
    }

    impl<'a> Data<'a> {
        // Explicit:
        fn get_explicit<'b>(&'b self) -> &'a str {
            self.value
        }

        // Elided (returns lifetime of field, not &self):
        fn get_elided(&self) -> &'a str {
            self.value
        }
    }

    let s = String::from("data");
    let data = Data { value: &s };
    println!("Rule 3 - explicit: {}", data.get_explicit());
    println!("Rule 3 - elided: {}", data.get_elided());

    // When elision doesn't work
    // This needs explicit lifetimes:
    fn needs_explicit<'a>(x: &'a str, y: &str) -> &'a str {
        x // Returns first parameter
    }

    println!("Needs explicit: {}", needs_explicit("first", "second"));

    println!();
}

/// Lifetime constraints and bounds
fn lifetime_constraints() {
    println!("--- Lifetime Constraints ---");

    // Lifetime subtyping: 'long: 'short
    // 'long outlives 'short

    fn select_first<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
        x // 'b must outlive 'a
    }

    let s1 = String::from("first");
    let s2 = String::from("second");
    println!("Selected: {}", select_first(&s1, &s2));

    // Generic with lifetime bound
    fn print_ref<'a, T>(value: &'a T)
    where
        T: std::fmt::Display + 'a,
    {
        println!("Value: {}", value);
    }

    let num = 42;
    print_ref(&num);

    // Struct with lifetime constraints
    struct Wrapper<'a, T: 'a> {
        value: &'a T,
    }

    impl<'a, T: 'a + std::fmt::Display> Wrapper<'a, T> {
        fn print(&self) {
            println!("Wrapped: {}", self.value);
        }
    }

    let x = 100;
    let wrapper = Wrapper { value: &x };
    wrapper.print();

    // Multiple lifetime parameters with constraints
    fn complex<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
    where
        'b: 'a, // 'b outlives 'a
    {
        x
    }

    println!("Complex: {}", complex("alpha", "beta"));

    println!();
}

/// Structs with lifetimes
fn struct_lifetimes() {
    println!("--- Struct Lifetimes ---");

    // Simple struct with lifetime
    #[derive(Debug)]
    struct Book<'a> {
        title: &'a str,
        author: &'a str,
    }

    let title = String::from("Programming Rust");
    let author = String::from("Blandy & Orendorff");
    let book = Book {
        title: &title,
        author: &author,
    };
    println!("Book: {:?}", book);

    // Multiple lifetimes
    #[derive(Debug)]
    struct Citation<'a, 'b> {
        quote: &'a str,
        source: &'b str,
    }

    let quote = String::from("To be or not to be");
    let source = String::from("Hamlet");
    let citation = Citation {
        quote: &quote,
        source: &source,
    };
    println!("Citation: {:?}", citation);

    // Struct with methods
    struct Parser<'a> {
        input: &'a str,
        position: usize,
    }

    impl<'a> Parser<'a> {
        fn new(input: &'a str) -> Self {
            Parser { input, position: 0 }
        }

        fn current(&self) -> Option<&'a str> {
            if self.position < self.input.len() {
                Some(&self.input[self.position..])
            } else {
                None
            }
        }

        fn advance(&mut self) {
            self.position += 1;
        }
    }

    let text = String::from("hello");
    let mut parser = Parser::new(&text);
    if let Some(current) = parser.current() {
        println!("Parser current: {}", current);
    }
    parser.advance();
    if let Some(current) = parser.current() {
        println!("After advance: {}", current);
    }

    // Nested structs with lifetimes
    struct Container<'a> {
        data: &'a str,
    }

    struct Wrapper<'a> {
        container: Container<'a>,
    }

    let data = String::from("nested data");
    let container = Container { data: &data };
    let wrapper = Wrapper { container };
    println!("Nested: {}", wrapper.container.data);

    println!();
}

/// Static lifetime
fn static_lifetime() {
    println!("--- Static Lifetime ---");

    // String literals have 'static lifetime
    let s: &'static str = "I have a static lifetime";
    println!("Static string: {}", s);

    // Can be used anywhere
    fn takes_static(s: &'static str) {
        println!("Received static: {}", s);
    }

    takes_static("static string");

    // Static variables
    static GLOBAL: &str = "I'm global";
    println!("Global static: {}", GLOBAL);

    // Returning static references
    fn get_constant() -> &'static str {
        "constant value"
    }

    println!("Constant: {}", get_constant());

    // Leaking to create 'static
    fn create_static_string() -> &'static str {
        let s = String::from("leaked");
        Box::leak(s.into_boxed_str())
    }

    let static_ref = create_static_string();
    println!("Leaked static: {}", static_ref);

    // 'static doesn't mean immutable
    static mut COUNTER: i32 = 0;

    unsafe {
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }

    // 'static in trait bounds
    fn process<T: 'static>(value: T) {
        // T must not contain any references with shorter lifetimes
        println!("Processing static-bounded type");
    }

    let owned = String::from("owned");
    process(owned); // Owned types are 'static

    println!("\n'static means:");
    println!("  - Lives for entire program duration");
    println!("  - String literals are 'static");
    println!("  - No borrowed references (or all 'static)");
    println!("  - Can be leaked to heap");

    println!();
}
