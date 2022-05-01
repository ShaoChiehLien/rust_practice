// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Jack";
    let mut age = 23; // Add "mut" so the variable is mutable (value can be reassigned)

    println!("My name is {0} and I am {1}", name, age);

    // name = "Tom"; // name is immutable variable so it can't be reassigned 
    age = 24;

    println!("My name is {0} and I am {1}", name, age);

    // Define constant
    const ID: i32 = 100; // Need to specify a type (i32: integer 32 bit)
    println!("ID: {}", ID);

    // Assigned multiple vars
    let (my_name, my_age) = ("Jack", 24);
    println!("{} is {}", my_name, my_age);
}