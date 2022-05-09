// When looking up a method call, the receiver may be automatically dereferenced or borrowed in order to call a method
// The difference is the use of reborrowing:
// https://stackoverflow.com/questions/68936034/mutable-reference-to-a-vector-was-moved-due-to-this-implicit-call-to-into-iter
fn question1(vec: &mut [String]) {
    for item in vec.into_iter() {
        println!("{}", item);
    }
    /* the only difference is into_iter()
    for item in vec {
        println!("{}", item);
    }
    */
    for item in vec {
        println!("{}", item);
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0]; // why item[0] don't need reference
    for &item in list { // why &item needs reference
        if largest < item {
            largest = item;
        }
    }
    largest
}

fn question2() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
}

pub fn run() {
    let mut s = vec![String::from("1"), String::from("2"), String::from("3")];
    question1(&mut s); // not fsmiliar with the concept og reborrwing
    question2();
}