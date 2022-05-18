fn iter_example() {
    // iterator iter doesn't take ownership of v1. It returns immutable reference.
    let v1 = vec![String::from("1"), String::from("2"), String::from("3")];
    println!("v1: {:?}", v1);
    let v1_iter = v1.iter();

    for item in v1_iter { // item is a immutable reference to String
        println!("{}", item);
        // item.push_str("!"); // Error: immutable reference can't be changed
    }

    // println!("{:?}", v1_iter); // Error: iterator always gets consumed after called
    println!("v1: {:?}", v1); // v1 still exists
}

fn into_iter_example() {
    // iterator into_iter takes ownership of v1 and return owned value
    let v1 = vec![String::from("1"), String::from("2"), String::from("3")];
    println!("v1: {:?}", v1);
    let v1_iter = v1.into_iter();

    // println!("v1: {:?}", v1); // Error: v1 ownership is taken away by v1_iter

    for mut item in v1_iter { // item is a String
        println!("{}", item);
        item.push_str("!"); // item owns the data in v1 so it can be modified
    }

    // println!("{:?}", v1_iter); // Error: iterator always gets consumed after called
}

fn iter_mut_example() {
    // iterator iter_mut doesn't take ownership of v1. It returns mutable reference.
    let mut v1 = vec![String::from("1"), String::from("2"), String::from("3")];
    println!("v1: {:?}", v1);
    let v1_iter = v1.iter_mut();

    for item in v1_iter { // item is a mutable reference to String
        println!("{}", item);
        item.push_str("!"); // item is mutable reference to element in v1
    }

    println!("v1: {:?}", v1); // v1 is changed through mutable reference in for loop
    // println!("{:?}", v1_iter); // Error: iterator always gets consumed after called
}

fn next_vs_for_on_iterator() {
    // v1_iter needs to be mutable when since calling next method 
    // change the internal state of an iterator
    let v1 = vec![String::from("1"), 
        String::from("2"), String::from("3")];
    let mut v1_iter = v1.iter();

    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());

    // v2_iter doesn't need to be mutable since for loop took 
    // ownership of v2_iter and make it mutable behind the scene
    let v2 = vec![String::from("1"), 
        String::from("2"), String::from("3")];
    let v2_iter = v2.iter();
    for item in v2_iter {
        println!("{}", item);
    }
}

pub fn run() {
    println!("Next method vs for on iterator");
    next_vs_for_on_iterator();

    println!("\nIterator iter example: ");
    iter_example();

    println!("\nIterator into_iter example: ");
    into_iter_example();

    println!("\nIterator iter_mut example: ");
    iter_mut_example()
}