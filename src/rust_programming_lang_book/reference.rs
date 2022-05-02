pub fn run() {
    println!("good example 1");
    good_example1();

    println!("good example 2");
    good_example2();

    println!("good example 3");
    good_example3();
}

fn good_example1() {
    // reference: https://users.rust-lang.org/t/reference-to-a-reference/44753/2
    // Important!!! Difference between &s2 = &s1 and s2 = &s1
    let s = String::from("hello");
    let s1 = &s;  // mutable variable s1 "immutable borrow" s (immutable reference)

    let s2 = &s1;
    // &s2 = &s1: s2 and s1 are on the same address, they refere to the same data.
    // s2 = &s1: s2 and s1 are on a different address, but they refer to the same data.

    println!("s: {:?}", s);
    println!("s1's value {:?}, s2's value {:?}", s1, s2);
    println!("s1's address {:p}, s2's address {:p}", s1, s2);
}

fn good_example2() {
    // Two immutable reference: can exist at the same time
    // reference: Either one mutable reference or any number of immutable references.
    let s = String::from("hello");

    // "mut s1" means s1 is a mutable variable
    // "&s" means s1 is immutable borrowing s
    let mut s1 = &s; // // mutable variable s1 "immutable borrows" s (immutable reference)
    let mut s2 = &s;

    println!("{:?}", s1);
    println!("{:?}", s2);
}

fn good_example3() {
    // One mutable and one immutable reference, but mutable reference's (s1) lifetime end
    // before immutable reference (s2) is activate, so it won't trigger error
    // reference: Either one mutable reference or any number of immutable references.
    let mut s = String::from("hello");
    
    let mut s1 = &mut s; // mutable variable s1 "mutable borrows" s (mutable reference)
    println!("{:?}", s1); // end of s1's lifetime

    let mut s2 = &s; // mutable variable s2 "immutable borrows" s (immutable reference)
    println!("{:?}", s2); // end of s2's lifetime
}

/*
fn bad_exapmple1() {
    // bad example: println! use immutable borrow 
    let mut s = String::from("hello");
    let mut s1 = &mut s; // mutable variable s1 "mutable borrows" s (mutable reference)

    println!("s: {:?}", s); // immutable borrow occurs here (println! use immutable borrow)
    println!("s1: {:?}", s1); // end of mutable variable s1's lifetime
}
*/

/*
fn bad_exapmple2() {
    // One mutable and one immutable reference, and they overlap
    let mut s = String::from("hello");
    
    let mut s1 = &mut s; // mutable variable s1 "mutable borrows" s (mutable reference)
    let mut s2 = &s; // mutable variable s2 "immutable borrows" s (immutable reference)

    println!("{:?}", s1);
    println!("{:?}", s2);
}
*/