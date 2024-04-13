// it is possible to declare a variable first and then bind it to a value later
fn main() {
    // declare a variable binding 
    let a_binding;
    // starting a code block 
    {
        let x = 2;

        // initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding); // prints "a binding: 4" since a_binding is declared outside the block and initialized inside the block

    let another_binding;

    // Error! use of uninitialized binding
    // println!("another binding: {}", another_binding); // error[E0381]: borrow of possibly-uninitialized variable: `another_binding`
    // FIXME ^ Comment out this line

    another_binding = 1;
    println!("another binding: {}", another_binding); // prints "another binding: 1" since another_binding is declared and initialized

} 