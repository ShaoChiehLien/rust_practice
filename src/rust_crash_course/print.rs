pub fn run(){
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting: Use placeholder for number
    println!("{} has {} apples", "Jack", 2);

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Jack", "Taiwan", "code");

    // Named Arguments
    println!("{name} likes to {activity}", name = "Jack", activity = "play basketball");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}