pub fn run() {
    println!("Demo 1: ownership");
    ownership();

    println!("Demo 2: reference and borrow");
    ref_borrow();
}

fn ownership() {
    // gives_ownership moves its return value into s1 (s1 now hold the data of some_string)
    let s1 = gives_ownership();

    // s2 comes into scope
    let s2 = String::from("hello");

    // s2 moves into takes_and_gives_back. takes_and_gives_back moves its return value 
    // into s3
    let s3 = takes_and_gives_back(s2);

    println!("s1: {:?}", s1);
    // println!("s2: {:?}", s2); // s2 is invalid since it's moved
    println!("s3: {:?}", s3);

    // s3 goes out of scope and is dropped; s2 was moved, so nothing happens; s1 goes out of 
    // scope and is dropped
}

fn gives_ownership() -> String {
    let some_string = String::from("gives_ownership"); // some_string comes into scope
    
    some_string // some_string is returned and moves out to the calling funciton
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    
    a_string // some_string is returned and moves out to the calling funciton
}

fn ref_borrow() {
    // Could pass in String itself (s1) or reference (&s1)
    // Pass in s1 would lead to lost of ownership, 
    // i.e. s1 would be invalid after this line; Pass in &s1 wouldn’t cause this.
    let s1 = String::from("hello");
    let len = calculate_lenth(&s1);

    println!("The length of '{}' is {} (s1 is still valid)", s1, len);
}

// signature of function use &: accept reference
fn calculate_lenth(s: &String) -> usize {
    s.len()
}

/* Uncomment to see the error
// `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
fn change(s: &String) {
    s.push_str(" World!")
}
*/