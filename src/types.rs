/*
Primitive Types
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 
(number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the type of 
// all variables at compile time, however, the compiler can usually infer what type
// we want to use based on the value and how we use it.

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 4545454545454545;

    println!("{:?}", (x, y, z));

    // Find max size
    println!("Max i32: {}", std::i32::MAX); // std is the standard library
    println!("Max i64: {}", std::i64::MAX); // std is the standard library

    // Boolean
    let is_active = true; // The convention for variable is underscore, not camelcase
    let is_active_specifiy: bool = true; // specify variable type

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    println!("{:?}", (is_active, is_active_specifiy, is_greater));

    let a1: char = 'a'; // character
    let face = '\u{1F600}'; // emoji are unicode as well

    println!("{:?}", (a1, face));
}