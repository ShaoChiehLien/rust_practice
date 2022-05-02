// Write a function that takes a string and returns the first word it finds 
// in that string. If the function doesn’t find a space in the string, the 
// whole string must be one word, so the entire string should be returned.

pub fn run() {
    // solution1(); // bad example
    solution2(); // better solution
}

fn solution1() {
    let mut s = String::from("hello world!");

    let word_len = first_word_sol1(&s);

    s.clear();

    // word_len stil have value even though string s has been cleared
    println!("The length of s is {} but s is already cleared!!!!!", word_len);
}

fn first_word_sol1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn solution2() {
    let mut s = String::from("hello world!");

    let word_len = first_word_sol2(&s);

    println!("{:?}", word_len);

    s.clear();

    // word_len can no longer be used after s.clear()
    
    // s.clear() uses mutable reference to prevent the data still being used after clearing it. 
    // - If A is a mutable reference to the data before s.clear() and later used, RUST will 
    // show compile error since no 2 mutable reference on same data should exist at the same time. 
    // - If A is a immutable reference to the data before s.clear() and later used, it will 
    // be immutable reference sandwiching a mutable reference, RUST will show compile error 
    // since mutable reference and immutable reference on the same data shouldn’t exist at the 
    // same time.
}

fn first_word_sol2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
