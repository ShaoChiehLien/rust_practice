// Reference borrowing: borrow the data to view or modify, but the 
// ownership still belongs to others

pub fn run() {
    reference_borrowing();
    mutable_reference_borrowing();
}

fn reference_borrowing() { // simply view the data
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);
}

fn mutable_reference_borrowing() { // view and modify the data
    let mut s1 = String::from("hello");
    
    change(&mut s1);

    println!("The changed string is '{}'", s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) { 
    // mut reference: borrow the data and change it while others still owns it
    s.push_str(" world!");
}