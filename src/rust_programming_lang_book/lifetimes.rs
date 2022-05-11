use std::fmt::Display;

/***************** Idea of Lifetime *****************/
/*
fn dangling_reference() {
    // a' is the lifetime of r, b' is the lifetime of x
    let r;                   // --------------+-- a'
    {                        //               |
        let x = 5;           // ----+-- b'    |
        r = &x;              //     |         |
    }                        // ----+         |
    println!("r: {}", r);    // --------------+
    // Error: r has lifetime of a' but refers to memory who only
    // has lifetime of b'
}
*/

fn sol_to_dangling_reference() {
    // Reference's lifetime end at the point it is last used.
    // Value's lifetime end at the point it is out of scope.
    // 'b is the life time of x, 'a is the life time of r
    // r is refereing to memory (x) that lives longer that itself
    // which pass the borrow checker
    let x = 5;              // -------------+-- 'b
                            //              |
    let r = &x;             // ---+-- 'a    |   
                            //    |         |
    println!("r: {}", r);   // ---+         |
}                           // -------------+
/************** End of idea of Lifetime **************/

/*************** Lifetime in function ***************/
// Error: compiler not sure the relationship between the reference 
// returned and the parameter passed in, i.e. should returned 
// reference has lifetime of x or y? Rust doesn't care how the
// function is implemented, it needs to explicitly know the answer
/*
fn longest(x: &str, y: &str) -> &str{
    if x.len() < y.len() {
        y
    } else {
        x
    }
}
*/

// Add lifetime to specify that the &str returned equal to
// the shorter lifetime between x and y 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() < y.len() {
        y
    } else {
        x
    }
}

fn lifetime_in_func_case_1() {
    println!("Show the lifetime of returned reference");
    let s1 = String::from("very very very long string");

    {
        let s2 = String::from("short string");
        let result = longest(s1.as_str(), s2.as_str());

        println!("The longest string is '{}'", result);
        // result has lifetime of s2 since we specify the 
        // shorter one between s1 and s2. The result is still
        // valid inside this block
    }
}

/*
fn lifetime_in_func_case_2() {
    println!("Show the lifetime of returned reference");
    let s1 = String::from("very very very long string");
    let result;

    {
        let s2 = String::from("short string");
        result = longest(s1.as_str(), s2.as_str());
        // Error: s2 doesn't live long enough
    }
    // If s2 is assigned to result, at this point it is deallocated
    // In C++ it would become a odd behavior that when s1 is longer
    // than s2, it works perfectly since s2 is not needed after calling
    // longest but when s2 is longer than s1, it will become unpredictable
    println!("The longest string is {}", result);
}
*/

fn lifetime_in_function() {
    println!("Case 1: ");
    lifetime_in_func_case_1();
    println!("Case 2: Error, uncomment to see detail");
    // lifetime_in_func_case_2(); // Error
}
/************ End of lifetime in function ************/

/**************** Lifetime in struct ****************/
// It means an instance of ImportantExcerpt can't outlive the reference
// it holds in part field
struct ImportantExcerpt<'a>{
    part: &'a str,
}

fn lifetime_in_struct() {
    // Borrow check will make sure i doesn't outlive novel 
    let novel = String::from("Call me Jack. Some years ago..."); // -----------------+-- lifetime 
    let first_sentence =                                         //                  |   of novel
        novel.split('.').next().expect("Could not find a '.'");  //                  |
    let i = ImportantExcerpt{                                    // -+- life time    |
        part: first_sentence,                                    //  |  of instance  |
    };                                                           // -+               |
    println!("first sentence: {}", i.part);                      // -----------------+
}
/************* End of lifetime in struct *************/

/************** Lifetime elision rule ***************/
// First rule: each intput parameter gets its own lifetime parameter
// Same as the functions with lifetime signature below 
fn first_rule(s: &str) -> i32{
    println!("{}", s);
    12345
}

/* first rule: first_rule with lifetime signature
fn first_rule<'a>(s: &'a str) -> i32{
    println!("First rule: {}", s);
    12345
}
*/

// Second rule: if there is exactly one input lifetime parameter, that lifetime 
// is assigned to all output lifetime parameters
// Same as the functions with lifetime signature below 
fn first_and_second_rule(s: &str) -> &str {
    s
}

/* second rule: first_and_second_rule with lifetime signature
fn first_and_second_rule<'a>(s: &'a str) -> &'a str {
    s
}
*/

// Third rule: if there are multiple input lifetime parameters, but one 
// of them is &self, lifetime of self is assigned to all output lifetime parameters
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part 
    }
}

/* third rule: announce_and_return_part with lifetime signature
// Lifetime name of structs always need to be declared after impl, cause they 
// are part of the struct's type
impl<'a> ImportantExcerpt<'a> {
    // lifetime of 'b is only needed in this method, so we declare it here
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part 
    }
}
*/

fn elision_rules() {
    println!("First rule");
    println!("{}", first_rule("Each parameter gets its own lifetime parameter"));

    println!("\nSecond rule");
    println!("{}", first_and_second_rule("If there is exactly one input lifetime\
        parameter, that lifetime is assigned to all output lifetime parameters"));
    
    println!("\nThird rule");
    let novel = String::from("Call me Jack. Some years ago...");
    let first_sentence =
        novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt{
        part: first_sentence,
    };
    println!("{}", i.announce_and_return_part("first sentence of novel"));
}
/*************** End of elision rule ****************/

/****** Combine generic type, traits bounds, and lifetime ******/
// Use 'a to specify the relationship between x, y, and output parameter
// Use generic type (T) and trait bounds (where clause) to specify the 
// type that is allowed in this function
fn longest_with_announcement<'a, T>(
    x: &'a str, y: &'a str, announce: T) -> &'a str
    where T: Display {
        println!("Announcement!: {}", announce);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn combine_all_together() {
    let novel = String::from("Call me Jack. Some years ago...");
    let first_sentence =
        novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt{
        part: first_sentence,
    };
    println!("{}", longest_with_announcement(
        "aaaa", "bbbbbbbbb", String::from("The longest string is ")));
}
/** End of combine generic type, traits bounds, and lifetime **/


pub fn run() {
    println!("Show the basic idea of lifetimes by dangling reference");
    // dangling_reference(); // Error
    sol_to_dangling_reference();

    println!("\nSpecify reference lifetime in function");
    lifetime_in_function();

    println!("\nSpecify reference lifetime in stuct");
    lifetime_in_struct();

    println!("\nElision rules");
    elision_rules();

    println!("\nCombine generic type, traits bounds, and lifetime");
    combine_all_together();
}