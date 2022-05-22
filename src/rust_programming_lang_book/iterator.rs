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

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn iterator_adaptor_example() {
    // shoes own the vec
    let mut shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    println!("shoes: {:?}", shoes);

    // iter_mut iterate mutable reference of shoes, any modification made
    // with affect shoes as well
    let v2: Vec<_> = shoes.iter_mut().map(|x| {x.size += 2; x}).collect();
    println!("v2: {:?}", v2); // v2's element is a reference to shoes vector
    println!("shoes: {:?}", shoes);

    // into_iter consume shoes and return owned element of shoes
    let v3: Vec<_> = shoes.into_iter().filter(|s| s.style.eq(&String::from("sandal"))).collect();
    println!("v3: {:?}", v3); // v3's element is an owned element
    // println!("shoes: {:?}", shoes); // Error: shoes is consumed by the into_iter call
}

/************** Create own Iterators **************/
#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    // Iterator that calls next needs to be mutable so the internal
    // state could keep track where it is
    // return an option with type of Item (u32)
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn call_own_iterator() {
    for i in Counter::new().zip(Counter::new().skip(1)){
        println!("{:?}", i);
    }
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1)) // (1, 2), (2, 3), (3, 4), (4, 5)
        .map(|(a, b)| a * b) // 2, 6, 12, 20
        .filter(|x| x % 3 == 0) // 6, 12
        .sum(); // 18
    println!("sum: {:?}", sum);
}
/********* End of Creating own Iterators *********/

pub fn run() {
    println!("Next method vs for on iterator");
    next_vs_for_on_iterator();

    println!("\nIterator iter example: ");
    iter_example();

    println!("\nIterator into_iter example: ");
    into_iter_example();

    println!("\nIterator iter_mut example: ");
    iter_mut_example();

    println!("\nIterator adaptor example: ");
    iterator_adaptor_example();

    println!("\nCreating our own iterator example: ");
    call_own_iterator();
}