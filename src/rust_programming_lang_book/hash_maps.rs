// hash map: a collection type in standard library
use std::collections::HashMap;

pub fn run() {
    println!("Run hashmap_basic");
    hashmap_basic();

    println!("\nRun hashmap_ownership");
    hashmap_ownership();

    println!("\nRun hashmap_iter");
    hashmap_iter();

    println!("\nRun different_way_to_insert");
    different_way_to_insert();

    // create_hashmap_with_vectors()
}

fn hashmap_basic() { // get value with key
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    // get returns Option<&V>, could either be Some or None
    let blue_score = scores.get("Blue");
    println!("{:?}", blue_score); // Some(&10)

    let green_score = scores.get("Green");
    println!("{:?}", green_score); // None
}

fn hashmap_ownership() {
    let name = String::from("Name"); // String: copy trait not implmented
    let value = 12345; // i32: copy trait auto-implemented 

    let mut map = HashMap::new();
    
    // map take the copy of value (i32) and ownership of name (String)
    map.insert(name, value);

    println!("map: {:?}", map);

    // integer automatically implement copy traits so it's still
    // valid after map.insert()
    println!("value: {:?}", value);

    // String's value is moved and hashmap is the owner after
    // map.insert()
    // println!("name: {:?}", name); // Error
}

fn hashmap_iter() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Yellow"), 10);
    scores.insert(String::from("Green"), 10);

    // HashMap can be interate in the same manner as vector
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn different_way_to_insert() {
    println!("\nRun insert_overwriting_value: ");
    insert_overwriting_value();

    println!("\nRun only_insert_if_has_no_value: ");
    only_insert_if_has_no_value();

    println!("\nRun insert_based_on_old_value: ");
    insert_based_on_old_value();
}

fn insert_overwriting_value() { // insert will overwrite value if key exist
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);

    println!("Overwrite insert: {:?}", scores);
}

fn only_insert_if_has_no_value() { // use Entry type and its method
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("scores: {:?}", scores);

    // the Entry returned contains mutable reference of scores, 
    // so we can still change the value of scores even after or_insert
    let green_entry = scores.entry(String::from("Green"));
    println!("green_entry: {:?}", green_entry);
    // Entry's method will insert if key not exists (VacantEntry("Green"))
    // Note that it will return a mutable reference to the value that key holds
    println!("green_entry.or_insert: {:?}", green_entry.or_insert(50));

    let blue_entry = scores.entry(String::from("Blue"));
    println!("blue_entry: {:?}", blue_entry);
    // Entry's method won't insert if key exists (OccupiedEntry("Blue"))
    // Note that it will return a mutable reference to the value that key holds
    println!("blue_entry.or_insert: {:?}", blue_entry.or_insert(50));

    println!("scores: {:?}", scores);
}

fn insert_based_on_old_value() { // Count the words
    let mut count_words = HashMap::new();

    let text = String::from("hello you how are you doing today today");

    // split_whitespace gets immutable reference so text is still
    // valid after the for loop
    for word in text.split_whitespace() { 
        let count = count_words.entry(word).or_insert(0);
        // or_insert() return a reference to the value to the key, i.e. &0
        *count += 1; // dereference count to apply arithmetic operation
    }

    println!("text: {:?}", text); // text is still valid
    println!("count_words: {:?}", count_words);
}

/*
// More to update after read chapter 13 and return to P188
fn create_hashmap_with_vectors() {
    let teams = vec!["Blue", "Red", "Yellow"];
    let initial_scores = vec![10, 50, 30];

    let mut scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores);
}
*/