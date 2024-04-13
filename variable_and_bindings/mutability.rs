// variable bindings are immutable by default, but this can be overridden using the `mut` modifier.

fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error! The type of a variable can't be changed.
    // _immutable_binding += 1;
    // FIXME ^ Comment out this line
}