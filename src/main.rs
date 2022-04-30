use std::env;

mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_ref;
mod structs;
mod enums;

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
    } else {
        println!("The script cannot be found!");
    }
}
