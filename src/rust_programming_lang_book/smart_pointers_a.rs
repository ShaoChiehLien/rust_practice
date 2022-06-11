use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

/****** Using Box<T> to Point to Data on the Heap ******/
fn basic_box() {
    let x = Box::new(5); // Box allocate a space on heap to store 5
    println!("int32 with Box<T>: {:?}", x); // x owns Box which points to 5
} // both Box(stored on stack) and 5(stored on heap) is deallocated here

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
    
    println!("Cons list with Box<T>: {:?}", cons_list);
}
/**** End of Using Box<T> to Point to Data on the Heap ****/

/** Treating Smart Pointers Like Regular References with the Deref Trait **/
fn basic_pointer() {
    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    // assert_eq!(y, 5); // Error: y is a reference; 5 is a value
}

fn box_as_reference() {
    let x = 5;
    let y = Box::new(x); // copy x to heap and use Box to point to it

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
/** Treating Smart Pointers Like Regular References with the Deref Trait **/

/************ Smart Pointer and Deref Traits ************/
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    // Self refers to whatever type that will implemented the trait, in this
    // case MyBox<T>. Reference: https://stackoverflow.com/questions/32304595/whats-the-difference-between-self-and-self
    // deref(&self) is actually deref(self: &Self). self is just a name
    fn deref(self: &Self) -> &Self::Target {
        &self.0
    }
}

fn define_our_own_smart_pointer() {
    let x = 5;
    // Box is actually just a struct with field allocated in heap
    // When derefereced, it passed the field value, which makes
    // it looks like reference
    let y = MyBox::new(x);

    println!("MyBox acts like a reference that can be deref with *");
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
/******** End of Smart Pointer and Deref Traits ********/

/************** Implicit Deref Coercions **************/
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn ref_checker(name: &str) {
    println!("Ref checker, {}!", name);
}

fn mut_ref_checker(name: &mut String) {
    name.push_str("~~~");
    println!("Mut ref checker, {}!", name);
}

fn basic_implicit_deref_coercions() {
    let m = MyBox::new(String::from("Jack"));
    // 1. Rust turn &MyBox<String> into &String by calling deref since 
    // Deref is implemented for &MyBox<String>
    // 2. Rust turn &String into &str by calling deref since
    // Deref is implemented for &String
    // Above steps was complemented with the nature of deref coercions
    ref_checker(&m);

    // (*m) dereference &MyBox<String> into String
    // & and [..] turns String into a string slice (&str)
    ref_checker(&(*m)[..]);
}

fn advanced_implicit_deref_coercions() {
    let mut m = MyBox::new(String::from("Jack"));
    // &T -> &U
    println!("\n&T -> &U:");
    ref_checker(&m);

    // &mut T -> &mut U
    println!("\n&mut T -> &mut U:");
    mut_ref_checker(&mut m);

    // &mut T -> &U
    println!("\n&mut T -> &U:");
    ref_checker(&m);

    // Error: &T -> &mut U
    println!("\n&T -> &mutU: Error");
    // mut_ref_checker(&m);
    // Can't guarantee only one immutable reference when converting 
    // to mutable reference
}
/********** End of Implicit Deref Coercions **********/

/*********** Smart Pointer and Drop Traits ***********/
struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    // drop traits accept mutable reference
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

fn impl_drop_traits_for_cleanup() {
    let my_stuff = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let other_stuff = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
}

fn my_drop(to_drop: CustomSmartPointer) {}

fn cleanup_with_drop_early() {
    let my_stuff = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let other_stuff = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");

    // my_stuff.drop(); 
    // Error: drop method is called at the end of function
    // Calling it here and at the end of cleanup_with_drop_early again will 
    // result in "double free error"

    drop(my_stuff);
    // 1. drop take ownership of my_stuff
    // 2. When drop function exist, it call my_stuff.drop
    // This result in my_stuff successfully dropped early
    my_drop(other_stuff);
    // The function drop above can be acheived by any function that 
    // take ownership and don't return

    println!("CustomSmartPointer dropped before the end of main");
}
/******** End of Smart Pointer and Drop Traits ********/

/*****  Using RC<T> to Allow Single Value have Multiple Owners *****/
#[derive(Debug)]
enum Sharable_List {
    Cons(i32, Rc<Sharable_List>),
    Nil
}

fn use_rc_to_share_cons_list() {
    // Create a reference-counting pointer 'a' with Rc::new
    let a = Rc::new(Sharable_List::Cons(5, 
                Rc::new(Sharable_List::Cons(10, 
                    Rc::new(Sharable_List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    // Creates another pointer 'b' to the same allocation 'a' is pointing 
    // to with Rc::clone
    let b = Rc::new(Sharable_List::Cons(3, Rc::clone(&a)));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        // Creates another pointer 'c' to the same allocation 'a' is pointing 
        // to with Rc::clone
        let c = Rc::new(Sharable_List::Cons(4, Rc::clone(&a)));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
} // Last Rc pointer (a) to given allocation is destroyed, the value is dropped
/** End of Using RC<T> to Allow Single Value have Multiple Owners **/

pub fn run() {
    println!("1. Using Box<T> to Point to Data on the Heap");
    basic_box();
    box_and_recursive_type();

    println!("\n2. Treating Smart Pointers Like Regular References with the Deref Trait");
    basic_pointer();
    box_as_reference();

    println!("\n3. Smart Pointer and Deref Traits");
    define_our_own_smart_pointer();

    println!("\n3.1 Example of implicit deref coerction");
    basic_implicit_deref_coercions();
    advanced_implicit_deref_coercions();

    println!("\n4. Smart Pointer and Drop Traits");
    impl_drop_traits_for_cleanup();
    cleanup_with_drop_early();

    println!("\n5. Using RC<T> to Allow Single Value have Multiple Owners");
    use_rc_to_share_cons_list();
}