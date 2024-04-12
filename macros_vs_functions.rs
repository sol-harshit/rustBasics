/**
 * add_two_integers function
 * 
 */
fn add_two_integers(a: i32, b: i32) -> i32 {
    a + b
}

/**
 * add_two_numbers macro to add two numbers
 * 
 */
macro_rules! add_two_numbers {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

/**
 * main function
 * 
 */
fn main() {
    let a = 10;
    let b = 20;
    let result = add_two_integers(a, b);
    println!("Result: {}", result);

    let result = add_two_numbers!(a, b);
    println!("Result: {}", result);
}


