use std::thread;
use std::time::Duration;
use std::collections::HashMap;

/********* Shared function for ex1 & ex2 *********/
fn expensive_calculation(intensity: u32) -> u32 {
    println!("calculate slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity*2
}
/************ End of Shared function ************/

/********************* Example 1 *********************/
fn example1_generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        // Run two times of expensive_calculation, double the time
        println!("Today, do {} pushups!", expensive_calculation(intensity)); 
        println!("Next, do {} situps!", expensive_calculation(intensity));
        println!("Next, do {} pullups!", expensive_calculation(intensity%7));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_calculation(intensity));   
        }
    } 
}

fn example1() {
    let mut simulated_user_specified_value = 10;
    let mut simulated_random_number = 7;

    println!("Case1: specified_value = {}, random_number = {}", 
        simulated_user_specified_value, simulated_random_number);
 
    example1_generate_workout(
        simulated_user_specified_value, simulated_random_number);

    simulated_user_specified_value = 50;
    simulated_random_number = 3;

    println!("Case2: specified_value = {}, random_number = {}", 
        simulated_user_specified_value, simulated_random_number);
    
    example1_generate_workout(
        simulated_user_specified_value, simulated_random_number);

    println!("Summary:\n\
            1. Run two times of expensive_calculation even intensity < 25 \
                and input is the same, which double the time\n\
            2. Hard to maintain (i.e. when we want to use other function instead \
                of expensive_calculation, we need to change every call to \
                expensive_calculation in exampleX_generate_workout)\n");
}
/***************** End of Example 1 *****************/

/********************* Example 2 *********************/
fn example2_generate_workout(intensity: u32, random_number: u32) {
    // Didn't run two times of expensive_calculation when intensity < 25
    // and input is the same
    // But always run expensive_calculation even its not needed 
    // (intensity > 25 && random_number == 3)
    let expensive_result_push_sit = expensive_calculation(intensity);
    let expensive_result_pull = expensive_calculation(intensity%7);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result_push_sit); 
        println!("Next, do {} situps!", expensive_result_push_sit);
        println!("Next, do {} pullups!", expensive_result_pull);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result_push_sit);   
        }
    } 
}

fn example2() {
    let mut simulated_user_specified_value = 10;
    let mut simulated_random_number = 7;

    println!("Case1: specified_value = {}, random_number = {}", 
            simulated_user_specified_value, simulated_random_number);
    example2_generate_workout(
        simulated_user_specified_value, simulated_random_number);

    simulated_user_specified_value = 50;
    simulated_random_number = 3;

    println!("Case2: specified_value = {}, random_number = {}", 
            simulated_user_specified_value, simulated_random_number);
    example2_generate_workout(
        simulated_user_specified_value, simulated_random_number);

    println!("Summary:\n\
        1. Didn't run two times of expensive_calculation when intensity < 25 \
            and input is the same \n\
        2. Easier to maintain (i.e. when we want to use other function instead \
            of expensive_calculation, we only need to change one line in \
            exampleX_generate_workout)\n\
        3. But always run expensive_calculation even it's not needed (intensity \
            > 25 && random_number == 3)\n");
}
/***************** End of Example 2 *****************/

/********************* Example 3 *********************/
fn example3_generate_workout(intensity: u32, random_number: u32) {
    // Use closure to save the definition of expensive_calculation
    let expensive_closure = |num| {
        println!("calculate slowly...");
        thread::sleep(Duration::from_secs(2));
        num*2
    };

    if intensity < 25 {
        // The definition is executed only when it is called
        println!("Today, do {} pushups!", expensive_closure(intensity)); 
        println!("Next, do {} situps!", expensive_closure(intensity));
        println!("Next, do {} pullups!", expensive_closure(intensity%7));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));   
        }
    } 
}

fn example3() {
    let mut simulated_user_specified_value = 10;
    let mut simulated_random_number = 7;

    println!("Case1: specified_value = {}, random_number = {}", 
            simulated_user_specified_value, simulated_random_number);
    example3_generate_workout(
        simulated_user_specified_value, simulated_random_number);

    simulated_user_specified_value = 50;
    simulated_random_number = 3;

    println!("Case2: specified_value = {}, random_number = {}", 
            simulated_user_specified_value, simulated_random_number);
    example3_generate_workout(
        simulated_user_specified_value, simulated_random_number);

    println!("Summary:\n\
        1. Still run two times of expensive_calculation when intensity < 25 \
            and input is the same, which double the time\n\
        2. Easier to maintain (i.e. when we want to use other function instead \
            of expensive_closure, we only need to change one line in \
            exampleX_generate_workout)\n\
        3. Only execute the expensive code when needed\n");
}
/***************** End of Example 3 *****************/

/********************* Example 4 *********************/
struct Cacher<T>
where T: Fn(u32) -> u32
{
    calculation: T, // T is any closure that accepts u32 and return u32
    value: HashMap<u32, u32>
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32
// trait bound is part of the struct identity, so it needs to be mentioned in impl
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher{
            calculation, // Same as calculation: calculation
            value: HashMap::new()
        }
    }

    fn value(&mut self, arg: u32) -> &mut u32 {
        // check if key exists, if not run the closure with key, 
        // save the value and return the value
        // or_insert will call (self.calculation)(arg) no matter if arg
        // exist, so we use or_insert_with instead
        // See https://stackoverflow.com/questions/60109843/entryor-insert-executes-despite-a-value-already-existing
        self.value.entry(arg).or_insert_with(|| (self.calculation)(arg))
    }
}

fn example4_generate_workout(intensity: u32, random_number: u32) {
    // Use closure to save the definition of expensive_calculation
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculate slowly...");
        thread::sleep(Duration::from_secs(2));
        num*2
    });

    if intensity < 25 {
        // The definition is executed only when it is called
        println!("Today, do {} pushups!", expensive_closure.value(intensity)); 
        println!("Next, do {} situps!", expensive_closure.value(intensity));
        println!("Next, do {} pullups!", expensive_closure.value(intensity%7));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", 
                expensive_closure.value(intensity));   
        }
    }
}

fn example4() {
    let mut simulated_user_specified_value = 10;
    let mut simulated_random_number = 7;

    println!("Case1: specified_value = {}, random_number = {}", 
            simulated_user_specified_value, simulated_random_number);
    example4_generate_workout(
        simulated_user_specified_value, simulated_random_number);

    simulated_user_specified_value = 50;
    simulated_random_number = 3;

    println!("Case2: specified_value = {}, random_number = {}", 
            simulated_user_specified_value, simulated_random_number);
    example4_generate_workout(
        simulated_user_specified_value, simulated_random_number);
    
    println!("Summary:\n\
        1. Run expensive_closure only when it's needed and only run the closure \
            when the key doesn't exist. i.e. expensive operation don't repeat \n\
        2. Easier to maintain (i.e. when we want to use other function instead \
            of expensive_closure, we only need to change one line in \
            exampleX_generate_workout)\n");

}
/***************** End of Example 3 *****************/

pub fn run() {
    println!("Example 1");
    example1();

    println!("Example 2");
    example2();

    println!("Example 3: Use closure to solve the problem in example 1 & 2");
    example3();

    println!("Example 4: Use a struct for closure");
    example4();
}