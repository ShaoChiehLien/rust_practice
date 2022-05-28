#[derive(Debug)]
// Rust will choose the variant that requires the most space to allocate
enum List {
    Cons(i32, Box<List>), // i32 + space to save the pointer
    Nil, // no space required
}

fn box_and_recursive_type() {
    let cons_list = 
        List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
        // List::Cons(1, address_x) is stack allocated
        // List::Cons(2, address_y) is heap allocated
        // List::Cons(3, address_z) is heap allocated
    
    println!("{:?}", cons_list);
}

pub fn run() {
    box_and_recursive_type();
}