#[derive(Debug)] // Adding the attribute to derive the Debug trait
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

pub fn run() {
    init_shorthand_syntax();
    create_inst_from_other_inst();
}

// field init shorthand syntax
fn init_shorthand_syntax() {
    let email = String::from("jack@gmail.com");
    let username = String::from("jack");

    // Rust will automatically match variable given in User with the struct's
    // fieldname to initialize the instance. Order doesn't matter
    let user1 = User {
        email, // "email: email" would also work, but that's a bit redundant
        active: true,
        username,
        sign_in_count: 2,
    };
    println!("Show order doesn't matter in creating instance of a struct");
    println!("{:?} {:?}", user1.email, user1.username);
}

// Creating Instances From Other Instances With Struct Update Syntax
fn create_inst_from_other_inst() {
    println!("Create instance from other instances with heap data inside");
    let user1 = User {
        email: String::from("user1@gmail.com"),
        active: true,
        username: String::from("user1"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("user2@gmail.com"),
        ..user1
    };

    // Can no longer use user1 after creating user2 with it because the 
    // user1.username (heap data String) was moved into user2.username
    // println!("user1: {:?} {:?}", user1.email, user1.username); // Error
    println!("user2: {:?} {:?}", user2.email, user2.username);

    println!("Create instance from other instaces with only stack data inside");
    let user3 = User {
        email: String::from("user3@gmail.com"),
        active: false,
        username: String::from("user3"),
        sign_in_count: 3,
    };

    let user4 = User {
        email: String::from("user4@gmail.com"),
        username: String::from("user4"),
        ..user3
    };

    // active (bool) and sign_in_count (int) are stack data, it implement 
    // copy trait automatically so there is no transfer of ownership 
    // (user3.active & user3.sign_in_count are still valid after creating
    // user4 with it)
    println!("user3: {:?}", user3);  // using debug format {:?} to print
    // println! print to stdout

    dbg!(&user4); // note that dbg takes ownership, so it's better to pass 
    // in reference
    // dbg! print to stderr
}