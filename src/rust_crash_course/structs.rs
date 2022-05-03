// Struct - Used to create custom data types

// Traditional Struct
struct TraditionColor {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct TupleColor(u8, u8, u8);

// Struct with function
#[derive(Debug)] // Adding the attribute to derive the Debug trait
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Associated Functions: normally used for construction
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Method: Get full name
    fn full_name(&self) -> String { // &self: only a view of itself
        format!("{} {}", self.first_name, self.last_name) // return
    }

    // Method: Set last name
    fn set_last_name(&mut self, last: &str) { // &mut self: need mutatable
        self.last_name = last.to_string();
    }

    // Method: Consume the instance and return a tuple
    // This method uses "self" which will get the ownership of the instance. 
    // The instance itself will be invalid if ownership is not returned
    fn struct_to_tuple(self) -> (String, String) { // self: get the ownership
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    traditional_struct_vs_tuple_struct();
    struct_with_method_demo();
}

fn traditional_struct_vs_tuple_struct() {
    println!("Compare traditional struct and tuple struct");

    // traditional struct
    let mut trad_c = TraditionColor {
        red: 225,
        green: 0,
        blue: 0
    };

    println!("Traditional Struct Color: {} {} {}", trad_c.red, trad_c.green, trad_c.blue);
    trad_c.red = 200;
    println!("Traditional Struct Color: {} {} {}", trad_c.red, trad_c.green, trad_c.blue);

    // tuple struct
    let mut tuple_c = TupleColor(255, 0, 0);

    println!("Tuple Struct Color: {} {} {}", tuple_c.0, tuple_c.1, tuple_c.2);
    tuple_c.0 = 200;
    println!("Tuple Struct Color: {} {} {}", tuple_c.0, tuple_c.1, tuple_c.2);
}

fn struct_with_method_demo() {
    println!("Struct with method demo");

    // Use :: syntax to call associated function
    let mut p = Person::new("Jack", "Lien"); 

    // Use . syntax followed by () to call method
    println!("Person {} {}", p.first_name, p.last_name);
    // Use . syntax without () to call field
    println!("Person {}", p.full_name());

    // Use . syntax followed by () to call method
    p.set_last_name("Lin");
    println!("Changed to Person {}", p.full_name());

    // p's ownership is moved to struct_to_tuple since struct_to_tuple accept self
    let tuple_p = p.struct_to_tuple();
    println!("{:?}", tuple_p);
    // println!("{:?}", p); // Error: p is invalid since its ownership is moved
}