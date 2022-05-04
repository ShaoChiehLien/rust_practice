// Vectors - Resizable arrays (must store values of the same type)

use std::mem;

pub fn run() {
    basic_vector();
    vector_iteration();
}

fn basic_vector () {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("Vector: {:?}", numbers);

    // Re-assgin value
    numbers[2] = 20;
    println!("Vector: {:?}", numbers);

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are heap allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    // Reference: https://stackoverflow.com/questions/57848114/what-are-the-differences-between-and-vec
    let mut slice: &[i32] = &numbers; // slice is a fat pointer that stores pointer and length
    println!("Slice: {:?}", slice);

    slice = &numbers[1..3];
    println!("Slice: {:?}", slice);
}

fn vector_iteration () {
    let mut numbers = vec![1, 2, 3, 4, 5]; // RUST can infer Vec<i32> from the data

    // Loop through vector values with iter
    for x in numbers.iter() {
        println!("Loop through vector values with iter: {}", x);
    }

    // Loop through vector values with reference
    for x in &numbers {
        println!("Loop through vector values with reference: {}", x);
    }

    // Loop and mutate value with iter_mut
    for x in numbers.iter_mut() { // iter_mut returns mutable reference of numbers
        // Arithmetic operator's signature is self instead of &self, so we dereference x (type &mut)
        *x *= 2; // multiply each number by 2
    }
    println!("Loop and mutate value with iter_mut: {:?}", numbers);

    // Loop and mutate value with mutable reference
    for x in &mut numbers {// &mut returns mutable reference of numbers
        // AArithmetic operator's signature is self instead of &self, so we dereference x (type &mut)
        *x *= 2; // multiply each number by 2
    }
    println!("Loop and mutate value with mutable reference: {:?}", numbers);
}