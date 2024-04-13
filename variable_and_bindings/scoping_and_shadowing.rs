// variable bindings have a scope, and are constrained to live in a block. 

fn main() {
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        // This binding *shadows* the outer one
        // here 5_f32 is a float number where f32 is the type of the number and 5.0 is the value of the number
        let long_lived_binding = 5.31_f32;

        // here {:.3} is used to print the float number with 3 decimal points
        println!("inner long: {:.3}", long_lived_binding);
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);

    // This binding also *shadows* the previous binding
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}