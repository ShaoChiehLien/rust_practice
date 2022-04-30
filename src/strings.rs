// Two types of strings
// Primitive str = It's a record of a string, not the string itself. Immutable fixed-length 
// string somewhere in memory, most commonly appears as &str: a reference to some UTF-8 data.
// String = Growable, heap-allocated data structure - Use when you need to modify or
// own string data (like passing strings to other threads, or building them at runtime).

// Comparison: 
// - If you need an allocation, use String. E.g. s.trim();
// - If you only need a different view of an existing string, use &str. E.g. s.to_uppercase()

pub fn run() {
    // let mut hello = "Hello "; // Won't work, since operation below changes the structure
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('W');

    // Push string
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contain 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    println!("Loop through string by whitespace: ");
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("string {}, len: {}, capacity: {}", s, s.len(), s.capacity());

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}