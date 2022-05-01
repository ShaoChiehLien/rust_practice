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
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String { // &self: only a view of itself
        format!("{} {}", self.first_name, self.last_name) // return
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) { // &mut self: need mutatable
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}


pub fn run() {
    // traditional struct
    let mut trad_c = TraditionColor {
        red: 225,
        green: 0,
        blue: 0
    };

    println!("Traditional Struct Color: {} {} {}", trad_c.red, trad_c.green, trad_c.blue);
    trad_c.red = 200;
    println!("Traditional Struct Color: {} {} {}", trad_c.red, trad_c.green, trad_c.blue);

    // typle struct
    let mut tuple_c = TupleColor(255, 0, 0);

    println!("Tuple Struct Color: {} {} {}", tuple_c.0, tuple_c.1, tuple_c.2);
    tuple_c.0 = 200;
    println!("Tuple Struct Color: {} {} {}", tuple_c.0, tuple_c.1, tuple_c.2);

    // struct with function
    let mut p = Person::new("Jack", "Lien");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {}", p.full_name());

    p.set_last_name("Lin");
    println!("Changed to Person {}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
}