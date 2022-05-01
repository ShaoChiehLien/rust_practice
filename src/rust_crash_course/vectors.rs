// Vectors - Resizable arrays

use std::mem;

pub fn run() {
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

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    // Reference: https://stackoverflow.com/questions/57848114/what-are-the-differences-between-and-vec
    let mut slice: &[i32] = &numbers; // slice is a fat pointer that stores pointer and length
    println!("Slice: {:?}", slice);

    slice = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate value
    for x in numbers.iter_mut() {
        // iter_mut returns mutable reference of numbers, so it needs to be dereference
        *x *= 2; // multiply each number by 2
    }

    println!("Numbers Vec: {:?}", numbers);
}