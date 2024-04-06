// there are three types of structs that can be creted using rust programming language
// 1. Tuple Structs
// 2. The classic C structs
// 3. Unit Structs


// An attribute to hide warnings for unused code.
#![allow(dead_code)]


// dervie the fmt::Debug implementation for the struct
#[derive(Debug)]


// In a struct property name and datatype are separated by a colon :

struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit; 

// A struct with two fields 
struct Point {
    x: f32,
    y: f32,
}

// A tuple struct
struct Pair(i32, f32);

// A struct can be used in another struct
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    
    // create a struct with field init shorthand
    let name = String::from("Peter Parker");
    let age = 27;

    let peter = Person {name, age};

    // debug print the struct
    println!("{:?}", peter);

    // Instantiate a Point
    let point: Point = Point { x: 10.3, y: 0.4 };

    // access the fields of the point
    println!("point coordinates: {} {}", point.x, point.y);

    // make a new point by using struct update syntax to use the fields of our other point
    // other one 
    let bottom_right = Point { x: 5.2, ..point};

    // bottom_right.y will be the same as point.y because we used that field from point
    println!("second point: {} {}", bottom_right.x, bottom_right.y);

    // destructure the point using a 'let' binding
    let Point {x: top_edge, y: left_edge} = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // define a tuple
    let Pair(integer, decimal) = pair;
    // accessing tuple to know the difference between a tuple and a tuple struct
    println!("pair contains {:?} and {:?}", integer, decimal);

    let new_tuple = (1, 2, 3);
    println!("Tuple: {:?} {:?} {:?}", new_tuple.0, new_tuple.1, new_tuple.2); 

}


 