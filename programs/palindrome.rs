//  current program checks if a string is a palindrome.

fn is_palindrome(input: &str) -> bool {
    // chars is a method that returns an iterator over the characters of a string slice.
    // rev is a method that reverses an iterator.
    // collect is a function that can take a collection of items and turn it into a specific type. In this case, we are collecting into a String.
    // ::<String> specifies the type we want to collect into.
    let reversed = input.chars().rev().collect::<String>();
    input == reversed
}

fn main() {
    let string = "Hello, world!";
    println!("Is '{}' a palindrome? {}", string, is_palindrome(string));

    let string = "racecar";
    println!("Is '{}' a palindrome? {}", string, is_palindrome(string));
}