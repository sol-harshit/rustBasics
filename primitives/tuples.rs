// Tuples are fixed size collections of values of different types.
// Once declared, they cannot grow or shrink in size.
// They are declared using parentheses and the values are separated by commas.
// Tuples can be used to return multiple values from a function.

// reverse() is a function that takes a tuple of two values and returns a tuple with the values reversed.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // we can use two variables to store the values of the tuple
    let (i32 integer, bool boolean) = pair;

    // we can also return a tuple directly, below is the shortened version of "return (boolean, integer);" 
    (boolean, integer);
}

fn main() {
    // a tuple with different types of values
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, true, false, 'a', "Hello, World!");

    // priting value of first element of the tuple
    println!("First element of the tuple: {}", long_tuple.0);
    
    // priting value of last element of the tuple
    println!("last element of the tuple: {}", long_tuple.12);

    // tuples can be nested inside other tuples
    let nested_tuple = ((1u8, 2u16, 3u32), (4u64, -1i8, -2i16), (-3i32, -4i64, 0.1f32), (0.2f64, true, false), ('a', "Hello, World!"));

    // priting value of first element of the nested tuple
    println!("First element of the nested tuple: {:?}", nested_tuple.0);

    // printing value of first element of the first element of the nested tuple
    println!("First element of the first element of the nested tuple: {}", (nested_tuple.0).0));

    // but very_long_tuples more than 12 elements are not allowed
    // let very_long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, true, false, 'a', "Hello, World!", 13u8);
    // error: expected tuple with 13 elements, found one with 14 elements
    // println!("Print very_long_tuple: {:?}", very_long_tuple);

    // printing the pair tuple 
    let pair = (1, true);
    println!("Pair: {:?}", pair);

    // calling the reverse function with the pair tuple
    println!("Reversed Pair: {:?}", reverse(pair));

    // to create a tuple with one element, a comma is required at the end
    let one_element_tuple = (13,);
    println!("One element tuple: {:?}", one_element_tuple);

    // to create an empty tuple, use empty parentheses
    let empty_tuple = ();
    println!("Empty tuple: {:?}", empty_tuple);

    // above can be ambiguous to an single interger, let's try printing an integer in parantheses, below is not a tuple
    println!("Integer in parentheses: {:?}", (13));

    // tuples can be deconstructed to create bindings
    let tuple = (1, "hello", 4.5, true);

    // deconstructing the tuple
    let (a, b, c, d) = tuple;
    println!("{}, {}, {}, {}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    // transpose the matrix
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}

// activity problem 1 
// create a function that takes a tuple of matrix as tuple type and returns the transpose of the matrix
fn transpose(matrix: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    let (a, b, c, d) = matrix;
    (a, c, b, d)
}