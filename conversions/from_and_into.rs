// if you are able to convert type a into type b, you can also convert type b into type a
// hence from and into are inherently linked, and this is why they are often provided together

// From trait allows for a type to define how to create itself from another type, hence providing a very simple mechanism for converting between several types
use std::convert::From;
use std::convert::Into;
#[derive(Debug)]

struct Number {
    value: i32,
}

struct IntoNumber {
    intVal: i32,
}

// impl implements a trait for a type 
// From is a trait that defines how to create a type from another type
// hence, we are implementing the From trait for the Number type

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
// Into is a trait that is the reciprocal of the From trait
// hence, we are implementing the Into trait for the Number type
// the difference between self and Self is that self is a reference to the type, and Self is the type itself
// for php developers, Self is like $this, and self is like $this->
impl Into<IntoNumber> for i32 {
    fn into(self) -> IntoNumber {
        IntoNumber { intVal: self}
    } 
}
 
fn main() {
    // converting str to String 
    // str is a primitive type, and hence does not have the from method implemented for it
    // String is a standard library type, and hence has the from method implemented for it
    // str is like an immutable string literal, and String is a heap-allocated string
    // str is like an array of bytes, and String is a vector of bytes
    // hence, we can convert a str to a String

    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("my_string: {}", my_string);

    let num = Number::from(30);
    println!("My number is {:?}", num.value);

    let int = 5;
    // Try removing the type declaration
    let num: IntoNumber = int.into();
    println!("My number is {:?}", num.intVal);



}