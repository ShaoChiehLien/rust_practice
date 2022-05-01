// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);

    // Re-assgin value
    numbers[2] = 20;
    println!("Array: {:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Arrary Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    // Reference: https://stackoverflow.com/questions/57848114/what-are-the-differences-between-and-vec
    let mut slice: &[i32] = &numbers; // slice is a fat pointer that stores pointer and length
    println!("Slice: {:?}", slice);

    slice = &numbers[1..3];
    println!("Slice: {:?}", slice);
}