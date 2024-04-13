// Globals are declared with the `const` keyword.
// Constants must be explicitly typed.
// Constants can be declared in any scope, including the global scope.
// Constants are always immutable.
// Constants can only be set to a constant expression, not the result of a function call or any other value that could only be computed at runtime.
// Constants are valid for the entire time a program runs within the scope they were declared in.

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // THRESHOLD = 5; // error: cannot assign to this expression
}