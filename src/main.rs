use std::env;

mod rust_crash_course;
mod rust_programming_lang_book;

use rust_crash_course::{print, vars, types, strings, tuples, arrays,
                        vectors, conditionals, loops, functions,
                        pointer_ref, structs, enums};

use rust_programming_lang_book::{ownership, borrow, slice, reference,
                        advanced_struct, advanced_string, hash_maps,
                        generic_types};

// Entry Point
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    if command == "print.rs" {
        print::run();
    } else if command == "vars.rs" {
        vars::run();
    } else if command == "types.rs" {
        types::run();
    } else if command == "strings.rs" {
        strings::run();
    } else if command == "tuples.rs" {
        tuples::run();
    } else if command == "arrays.rs" {
        arrays::run();
    } else if command == "vectors.rs" {
        vectors::run();
    } else if command == "conditionals.rs" {
        conditionals::run();
    } else if command == "loops.rs" {
        loops::run();
    } else if command == "functions.rs" {
        functions::run();
    } else if command == "pointer_ref.rs" {
        pointer_ref::run();
    } else if command == "structs.rs" {
        structs::run();
    } else if command == "enums.rs" {
        enums::run();
    } else if command == "ownership.rs" {
        ownership::run();
    } else if command == "borrow.rs" {
        borrow::run();
    } else if command == "slice.rs" {
        slice::run();
    } else if command == "reference.rs" {
        reference::run();
    } else if command == "advanced_struct.rs" {
        advanced_struct::run();
    } else if command == "advanced_string.rs" {
        advanced_string::run();
    } else if command == "hash_maps.rs" {
        hash_maps::run();
    } else if command == "generic_types.rs" {
        generic_types::run();
    } else {
        println!("The script cannot be found!");
    }
}
