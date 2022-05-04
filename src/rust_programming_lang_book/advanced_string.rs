// advanced_string: deep view of String as a collection type

pub fn run() {
    string_as_collection();
    basic_concatenation_demo();
    advanced_concatenation_demo();
    correct_way_to_iterate_over_string();
}

fn string_as_collection() { // String is also a collection type
    // Many of the operation available with vec<T> is also available with String
    let mut s = String::new(); // create an empty string
    let hello = "hello";

    s = hello.to_string(); // signature of to_string method use &self 
    // i.e. hello will be valid after this line

    println!("s: {:?}, hello: {:?}", s, hello);
}

fn basic_concatenation_demo() {
    let s1 = String::from("hello");
    let s2 = String::from(" world!");

    // Arithmetic operator's signature is self instead of &self
    // Add's signature: fn add(self, s: &str) -> String {}
    // s1 is moved to the Add function and lose its ownership (invald after the line)
    let s3 = s1 + &s2;

    // println!("s1: {:?}", s1); // Error: s1 is moved and invalid
    println!("s2: {:?}", s2);
    println!("s3: {:?}", s3);
}

fn advanced_concatenation_demo() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s4 = s1 + "-" + &s2 + "-" + &s3; // would work but s1 will be invalid after
    // format! won't take s1's ownership, s1 will be valid after
    let s5 = format!("{}-{}-{}", s1, s2, s3); 

    println!("advanced concatentation: s1 {}", s1);
    println!("advanced concatentation: s2 {}", s2);
    println!("advanced concatentation: s3 {}", s3);
    println!("advanced concatentation: s5 {}", s5);
}

fn correct_way_to_iterate_over_string() {
    // Be explicit about what you want: bytes or chars?
    let s = String::from("नमस्ते");

    println!("Iterate over string by characters:");
    for c in s.chars(){
        println!("{}", c);
    }

    println!("Iterate over string by bytes:");
    for b in s.bytes(){
        println!("{}", b);
    }
}
