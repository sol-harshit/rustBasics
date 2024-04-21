// The current programs reverse a given string.
// str and String are two different types. str is an unsized type that is a view into a string slice, 
// while String is a growable, mutable, owned, UTF-8 encoded string type.
fn reverse(input: &str) -> String {
    // chars is a method that returns an iterator over the characters of a string slice.
    // rev is a method that reverses an iterator.
    // collect is a function that can take a collection of items and turn it into a specific type. In this case, we are collecting into a String.
    input.chars().rev().collect()
}

fn main() {

    let string = "Hello, world!";
    println!("The reverse of '{}' is '{}'", string, reverse(string));    

}

