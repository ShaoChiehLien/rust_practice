
#[derive(Debug)] // Adding the attribute to derive the Debug trait
struct Point<T, U> { // mutiple generic types <T, U>
    x: T,
    y: U,
}

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

#[derive(Debug)] // Adding the attribute to derive the Debug trait
enum Option<T> {
    Some(T),
    None,
}

pub fn run() {
    println!("Run struct_with_generic");
    struct_with_generic();

    println!("Run enum_with_generic");
    enum_with_generic();

    println!("Run method_with_generic");
    method_with_generic();
}

fn struct_with_generic() {
    let point_a = Point{x: 10, y: 20.11};
    let point_b = Point{x: 10.11, y: 20.22};

    println!("point_a point: {:?}", point_a);
    println!("point_b point: {:?}", point_b);
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