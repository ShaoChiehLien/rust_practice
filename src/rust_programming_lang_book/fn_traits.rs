/***** Tool to check if FnX traits is implemented *****/
struct CheckFn<T, U>
where T: Fn() -> U
{
    closure: T,
}

struct CheckFnOnce<T, U>
where T: FnOnce() -> U
{
    closure: T,
}

struct CheckFnMut<T, U>
where T: FnMut() -> U
{
    closure: T,
}

/* 
// Example of FnX traits checker: 
// expect error since this closure only implement FnOnce
let closure_x = move || {
    println!("{:?}", x);
    x
};
let check_fn_mut = CheckFn {
    closure: closure_x
};
let check_fn_mut = CheckFnOnce {
    closure: closure_x
};
let x = CheckFnMut {
    closure: closure_x
};
*/
/** End of tool to check if FnX traits is implemented **/

/********************* Example 1 *********************/
fn example1() {
    println!("Closure (implements FnOnce) with move");
    let x = vec![1, 2, 3];
    // The closure needs to own x to operate: 
    // the closure only implements `FnOnce`
    let my_fn_once = move || { // move: my_fn_once owns x
        println!("my_fn_once: {:?}", x);
        x
    };
    my_fn_once();
    // my_fn_once(); // Error: This closure can't be used twice, since the 
    // closure lose the ownership of x after the first call my_fn_once()
    // println!("x: {:?}", x); // Error: The move sytax makes the closure
    // take the ownership of the value x is holding

    println!("Closure (implements FnMut & FnOnce) with move");
    let mut y = vec![1, 2, 3];
    // The closure needs mutable reference from y to operate: 
    // the closure implements `FnMut`, `FnOnce`
    let mut my_fn_mut = move || { // move: my_fn_mut owns y
        println!("my_fn_mut: {:?}", y);
        y.push(4);
    };
    my_fn_mut();
    my_fn_mut();
    // This closure can be used more than once since the closure still own y
    // println!("y: {:?}", y); // Error: The move sytax makes the closure
    // take the ownership of the value y is holding

    println!("Closure (implements Fn & FnMut & FnOnce) with move");
    let z = vec![1, 2, 3];
    // The closure only needs immutable reference from z: 
    // the closure implements `Fn`, `FnMut`, `FnOnce`
    let my_fn = move || { // move: my_fn owns z
        println!("my_fn: {:?}", z);
        z.len()
    };
    my_fn();
    my_fn();
    // This closure can be used more than once since the closure still own z
    // println!("z: {:?}", z); // Error: The move sytax makes the closure
    // take the ownership of the value z is holding

}
/***************** End of Example 1 *****************/

/********************* Example 2 *********************/
fn example2() {
    println!("Closure (implements FnOnce) without move");
    let x = vec![1, 2, 3];
    // The closure needs to own x to operate: 
    // the closure only implements `FnOnce`
    let my_fn_once = || { // my_fn_once owns x
        println!("my_fn_once: {:?}", x);
        x
    };
    my_fn_once();
    // my_fn_once(); // Error: This closure can't be used twice, since the 
    // closure lose the ownership of x after the first call my_fn_once()
    // println!("x: {:?}", x); // Error: The closure would just get the  
    // ownership of x since the body of closure needs it to operate

    println!("Closure (implements FnMut & FnOnce) without move");
    let mut y = vec![1, 2, 3];
    // The closure needs mutable reference from y to operate: 
    // the closure implements `FnMut`, `FnOnce`
    let mut my_fn_mut = || { // my_fn_mut owns y
        println!("my_fn_mut: {:?}", y);
        y.push(4);
    };
    my_fn_mut();
    my_fn_mut();
    // This closure can be used more than once since the closure
    // use the reference to y instead of owning it 
    println!("y: {:?}", y); 
    // The FnMut would just get the mutable reference of y since 
    // the body of closure only needs a mutable reference

    println!("Closure (implements Fn & FnMut & FnOnce) without move");
    let z = vec![1, 2, 3];
    // The closure only needs immutable reference from z: 
    // the closure implements `Fn`, `FnMut`, `FnOnce`
    let my_fn = || { // my_fn owns s
        println!("my_fn: {:?}", z);
        z.len()
    };
    my_fn();
    my_fn();
    // This closure can be used more than once since the closure
    // use the reference to z instead of owning it 
    println!("z: {:?}", z);
    // The Fn would just get the immutable reference of z since 
    // the body of closure only needs a immutable reference

}
/***************** End of Example 2 *****************/

pub fn run() { 
    println!("Example 1: Difference between closure that implements \
        FnOnce, FnMut, and Fn with move");
    example1();

    println!("\nExample 2: Difference between closure that implements \
        FnOnce, FnMut, and Fn without move");
    example2();
}