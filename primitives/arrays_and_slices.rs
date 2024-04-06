// arrays are fixed size and contiguous in memory, the size of the array must be known at compile time
// syntax for the array is [T; N] where T is the type of the elements and N is the number of elements

// slices are a reference to a contiguous sequence of elements in a collection and size is not known at compile time
// syntax for the slice is &[T] where T is the type of the elements

// arrays and slices are similar in many ways, but there are some differences between them
// 1. arrays have a fixed size, slices can be of any size
// 2. arrays are allocated on the stack, slices are references to a sequence of elements in a collection
// 3. arrays can be used to initialize a slice, but slices cannot be used to initialize an array

//std::mem is used to get the size of the array
use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("slice has {} elements", slice.len());
}

fn main() {
    // fixed size array where i32 is the type of the elements and 5 is the number of elements
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // all elements of an array can be initialized to the same value
    // here 0 is the value and 500 is the number of elements
    // 0 means all elements of the array will be initialized to 1 and 500 is the number of elements
    // by default the elements will be initialized to 0 if no value is provided
    let ys: [i32; 500] = [1; 500];

    // indexing starts with 0
    println!("first element of the array {}", xs[0]);
    println!("second element of the array {}", xs[1]);

    // len() is used to get the size of the array
    println!("array size: {}", xs.len());

    // std::mem::size_of_val is used to get the size of the array
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slices can be used to borrow a section of an array
    // here 2 is the starting index and 3 is the ending index
    // the ending index is exclusive
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);


    let empty_array: [i32; 0] = [];
    assert_eq!(&empty_array, &[]);
    println!("empty array is equal to empty slice");

    // same but more verbose
    assert_eq!(&empty_array, &[][..]); 


    // arrays can be safely accessed with the `get` method, which returns an `Option`
    // This method can be matched as shown below or used with expect() if you want 
    // the program to exit with an error rather than happily continue

    for i in 0..xs.len() {
        match xs.get(i) {
            Some(value) => println!("{}: {}", i, value),
            None => println!("{}: out of bounds", i),
        }
    }

    // out of bounds on an array will panic and will cause a compile time error
    // println!("{}", xs[5]);

    // out of bounds on a slice will cause a runtime error
    //println!("{}", &xs[1..5][4]);

    


}
