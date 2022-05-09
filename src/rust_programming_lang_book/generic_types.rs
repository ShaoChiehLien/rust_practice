/************* Function with generic *************/
// Use generic type parameter bound (T: PartialOrd) to
// make sure all type that uses largest needs to impl
// PartialOrd traits
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];

    for &item in list {
        if largest < item {
            largest = item;
        }
    }

    largest
}

fn func_with_generic() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
/********* End of function with generic *********/

/************** Struct with generic **************/
#[derive(Debug)] // Adding the attribute to derive the Debug trait
struct Point<T, U> { // mutiple generic types <T, U>
    x: T,
    y: U,
}

fn struct_with_generic() {
    let point_a = Point{x: 10, y: 20.11};
    let point_b = Point{x: 10.11, y: 20.22};

    println!("point_a point: {:?}", point_a);
    println!("point_b point: {:?}", point_b);
}
/********** End of struct with generic **********/

/*************** Enum with generic ***************/
#[derive(Debug)] // Adding the attribute to derive the Debug trait
enum Option<T> {
    Some(T),
    None,
}

fn checked_division(dividend: i32, divisor: i32) -> Option<i32>{
    // specify Option type when needed
    if divisor == 0 {
        Option::None
    } else {
        Option::Some(dividend/divisor)
    }
}

fn enum_with_generic() { // example of division
    println!("{:?}", checked_division(4, 2));
    println!("{:?}", checked_division(4, 0));
}
/************ End of enum with generic ************/

/*************** Method with generic ***************/
impl<X1, Y1> Point<X1, Y1> { // implement point for all types
    fn get_x(&self) -> &X1 {
        &self.x
    }

    fn exchange<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<String, i32> { // implement point only for points with <String, i32>
    fn print_point(&self) {
        println!("Mr {} has pointed to item No. {}", &self.x, &self.y);
    }
}

fn method_with_generic() {
    // point a(int, float) will have method get_x() since get_x() is for
    // all types <X1, Y1>
    // point a(int, float) won't have method print_point() since it's only
    // for Point<String, i32>
    let point_a = Point{x: 10, y: 20.11};
    println!("point a: {:?}", point_a);
    println!("point a.x: {:?}", point_a.get_x());
    // point_a.print_point();

    // point b(String, int) will have method get_x() since get_x() is for
    // all types <X1, Y1>
    // point b(String, int) will have method print_point() since it's
    // for Point<String, i32>
    let point_b = Point{x: String::from("Jack"), y: 10};
    println!("point b: {:?}", point_b);
    println!("point b.x: {:?}", point_b.get_x());
    point_b.print_point();

    // Generic method inside generic struct
    // declare some generic parameters with impl and some with method
    let p1 = Point{x: 1, y: String::from("hi")};
    let p2 = Point{x: String::from("hello"), y: 2.2};

    let p3 = p1.exchange(p2);

    println!("Exhange result: {:?}", p3);
}
/************ End of method with generic ************/

pub fn run() {
    println!("Run func_with_generic");
    func_with_generic();

    println!("\nRun struct_with_generic");
    struct_with_generic();

    println!("\nRun enum_with_generic");
    enum_with_generic();

    println!("\nRun method_with_generic");
    method_with_generic();
}