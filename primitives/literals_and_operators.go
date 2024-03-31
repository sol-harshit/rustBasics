/*
 * The current example demonstrates the usage of literals and operators in rust programming language.
 * we will be using the following literals and operators in this example:
 * 1. Integer `1`` literals
 * 2. Floating point `4.12` literals
 * 3. Boolean `true` literals
 * 4. Character `'a'` literals
 * 5. String `"Hello, World!"` literals
 * 6. unit type `()` literals
 * 7. Arithmetic operators `+`, `-`, `*`, `/`, `%`
 * 8. Comparison operators `==`, `!=`, `>`, `<`, `>=`, `<=`
 * 9. Logical operators `&&`, `||`, `!`
 * 10. Bitwise operators `&`, `|`, `^`, `<<`, `>>`
 * 11. Assignment operators `=`, `+=`, `-=`, `*=`, `/=`, `%=`
 * 12. Compound assignment operators `&&=`, `||=`, `&=`, `|=`, `^=`, `<<=`, `>>=`
 * 13. Ternary operator `?`
 * 14. Type cast operator `as`
 */ 
 fn main() {
	// Integer literals
	let integer = 1i32;
	println!("Integer: {}", integer); // prints: Integer: 1

	// Floating point literals
	let float = 4.12f32;
	println!("Float: {}", float); // prints: Float: 4.12

	// Boolean literals
	let boolean = true;
	println!("Boolean: {}", boolean); // prints: Boolean: true

	// Character literals
	let character = 'a';
	println!("Character: {}", character); // prints: Character: a

	// String literals
	let string = "Hello, World!";
	println!("String: {}", string); // prints: String: Hello, World!

	// Unit type literals
	let unit = ();
	println!("Unit: {:?}", unit); // prints: Unit: ()

	// Arithmetic operators | +, -, *, /, % | binary operators
	let sum = 1 + 2;
	println!("Sum: {}", sum); // prints: Sum: 3

	let difference = 1 - 2;
	println!("Difference: {}", difference); // prints: Difference: -1

	let product = 1 * 2;
	println!("Product: {}", product); // prints: Product: 2

	let quotient = 1 / 2;
	println!("Quotient: {}", quotient); // prints: Quotient: 0 (integer division) 

	let quotient_float = 1.0 / 2.0;
	println!("Quotient Float: {}", quotient_float); // prints: Quotient Float: 0.5 (floating point division)

	let remainder = 1 % 2;
	println!("Remainder: {}", remainder); // prints: Remainder: 1

	// Comparison operators | ==, !=, >, <, >=, <= | binary operators

	let equal = 1 == 2;
	println!("Equal: {}", equal); // prints: Equal: false

	let not_equal = 1 != 2;
	println!("Not Equal: {}", not_equal); // prints: Not Equal: true

	let greater_than = 1 > 2;
	println!("Greater Than: {}", greater_than); // prints: Greater Than: false

	let less_than = 1 < 2;
	println!("Less Than: {}", less_than); // prints: Less Than: true

	let greater_than_or_equal = 1 >= 2;
	println!("Greater Than or Equal: {}", greater_than_or_equal); // prints: Greater Than or Equal: false

	let less_than_or_equal = 1 <= 2;
	println!("Less Than or Equal: {}", less_than_or_equal); // prints: Less Than or Equal: true

	// Logical operators | &&, ||, ! | binary and unary operators

	let and = true && false;
	println!("And: {}", and); // prints: And: false

	let or = true || false;
	println!("Or: {}", or); // prints: Or: true

	let not = !true; // unary operator
	println!("Not: {}", not); // prints: Not: false 

	// Bitwise operators | &, |, ^, <<, >> | binary operators

	// below code prints 0 
	// 1 in binary is 0001
	// 2 in binary is 0010
	// Input 1: 0001
	// Input 2: 0010
	// Output&: 0000
	let and = 1 & 2;
	println!("And: {}", and); // prints: And: 0

	// below code prints 3
	// 1 in binary is 0001
	// 2 in binary is 0010
	// Input 1: 0001
	// Input 2: 0010
	// Output|: 0011
	let or = 1 | 2;
	println!("Or: {}", or); // prints: Or: 3

	// below code prints 3
	// 1 in binary is 0001
	// 2 in binary is 0010
	// Input 1: 0001
	// Input 2: 0010
	// Output^: 0011
	let xor = 1 ^ 2;
	println!("Xor: {}", xor); // prints: Xor: 3

	// below code prints 2
	// 1 in binary is 0001
	// Input: 0001
	// Output<<: 0100
	let left_shift = 1 << 2; // left shift by 2 bits
	println!("Left Shift: {}", left_shift); // prints: Left Shift: 4

	// below code prints 0
	// 1 in binary is 0001
	// Input: 0001
	// Output>>: 0000
	let right_shift = 1 >> 2; // right shift by 2 bits
	println!("Right Shift: {}", right_shift); // prints: Right Shift: 0

	// Assignment operators | =, +=, -=, *=, /=, %= | binary operators

	let mut a = 1; // mutable variable because by default variables are immutable in rust
	a += 2; // a = a + 2
	println!("Addition Assignment: {}", a); // prints: Addition Assignment: 3

	let mut s = 1;
	s -= 2; // s = s - 2
	println!("Subtraction Assignment: {}", s); // prints: Subtraction Assignment: -1

	let mut m = 1;
	m *= 2; // m = m * 2
	println!("Multiplication Assignment: {}", m); // prints: Multiplication Assignment: 2

	let mut d = 1;
	d /= 2; // d = d / 2
	println!("Division Assignment: {}", d); // prints: Division Assignment: 0

	let mut r = 1;
	r %= 2; // r = r % 2
	println!("Remainder Assignment: {}", r); // prints: Remainder Assignment: 1

	// Compound assignment operators | &&=, ||=, &=, |=, ^=, <<=, >>= | binary operators

	let mut and = true;
	and &= false; // and = and & false
	println!("And Assignment: {}", and); // prints: And Assignment: false

	let mut or = true;
	or |= false; // or = or | false
	println!("Or Assignment: {}", or); // prints: Or Assignment: true

	let mut xor = true;
	xor ^= false; // xor = xor ^ false
	println!("Xor Assignment: {}", xor); // prints: Xor Assignment: true

	let mut left_shift = 1;
	left_shift <<= 2; // left_shift = left_shift << 2, left shift by 2 bits
	println!("Left Shift Assignment: {}", left_shift); // prints: Left Shift Assignment: 4

	let mut right_shift = 1;
	right_shift >>= 2; // right_shift = right_shift >> 2, right shift by 2 bits
	println!("Right Shift Assignment: {}", right_shift); // prints: Right Shift Assignment: 0

	// Ternary operator | ? | ternary operator

	let a = 1;
	let b = 2;
	let max = if a > b { a } else { b }; // notice the absence of ternary operator in rust
	println!("Max: {}", max); // prints: Max: 2

	// Type cast operator | as | type cast operator

	let a = 1.0;
	let b = a as i32; // type cast from f64 to i32
	println!("Type Cast: {}", b); // prints: Type Cast: 1

	// We can compile above code by running `rustc literals_and_operators.rs -o literals_and_operators && ./literals_and_operators` in terminal
	// output:
	// Integer: 1
	// Float: 4.12
	// Boolean: true
	// Character: a
	// String: Hello, World!
	// Unit: ()
	// Sum: 3
	// Difference: -1
	// Product: 2
	// Quotient: 0
	// Quotient Float: 0.5
	// Remainder: 1
	// Equal: false
	// Not Equal: true
	// Greater Than: false
	// Less Than: true
	// Greater Than or Equal: false
	// Less Than or Equal: true
	// And: false
	// Or: true
	// Not: false
	// And: 0
	// Or: 3
	// Xor: 3
	// Left Shift: 4
	// Right Shift: 0
	// Addition Assignment: 3
	// Subtraction Assignment: -1
	// Multiplication Assignment: 2
	// Division Assignment: 0
	// Remainder Assignment: 1
	// And Assignment: false
	// Or Assignment: true
	// Xor Assignment: true
	// Left Shift Assignment: 4
	// Right Shift Assignment: 0
	// Max: 2
	// Type Cast: 1 
 }


