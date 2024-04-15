// Rust provides no implicit type conversion (coercion) between primitive types. But, explicit type 
// conversion (casting) can be performed using the as keyword.

// Rules for converting between integral types follow C conventions generally, except in cases where C 
// has undefined behavior. The behavior of the following conversions is well-defined in Rust:

#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;
    // FIXME ^ Comment out this line

    let integer = decimal as u8;
    let character = integer as char;
    // Error! There are limitations in conversion rules. A float cannot be directly converted to a char.
    // let character = decimal as char;
    // FIXME ^ Comment out this line

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T, std::T::MAX + 1 is added or subtracted until the value
    // fits into the new type
    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept, while the rest towards the most
    // significant bit (MSB) get truncated.
    println!("1000 as a u8 is: {}", 1000 as u8);

    // -1 + 256 = 255
    println!(" -1 as a u8 is: {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is: {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as first casting to the corresponding
    // unsigned type. If the most significant bit of that value is 1, then the value is negative.

    println!("128 as a i16 is: {}", 128 as i16);

    // 128 as u8 -> 128, whose two's complement in eight bits is: -128
    println!("128 as a i8 is: {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is: {}", 1000 as u8);

    // and the two's complement of 232 is -24
    println!("232 as a i8 is: {}", 232 as i8);

    // Since Rust 1.45, the `as` keyword performs a "saturating cast" when casting from float to int. If the
    // floating point value exceeds the upper bound or is less than the lower bound, the returned value will be
    // equal to the bound crossed.

    // 300.0 is 255
    println!("300.0 as a u8 is: {}", 300.0_f32 as u8);

    // -100.0 as u8 is 0
    println!("-100.0 as a u8 is: {}", -100.0_f32 as u8);

    // casting to a signed integer first
    // -100.0 as i8 is -100
    println!("-100.0 as a i8 is: {}", -100.0_f32 as i8);

    // nan as u8 is 0
    println!("nan as a u8 is: {}", f32::NAN as u8);


    // When casting any value to a floating point type, the fractional part is discarded.
    println!("1 as a f32 is: {}", 1i32 as f32);
    println!("1.9 as a i32 is: {}", 1.9 as i32);

    // This program will not compile as it is trying to assign a float to an integer type
    // let integer = 65.4321;
    // let integer: u8 = integer;
    // println!("Casting: {} -> {}", decimal, integer);

    // This behavior incurs a small runtime cost and can be avoided with unsafe methods, however the results
    // might overflow and return unexpected values. Be sure to extensively test your code when using `as` and
    // be aware of the potential overflow.

    unsafe {
        // 300.0 as u8 is 44
        println!("300.0 as a u8 is: {}", 300.0_f32.to_int_unchecked::<u8>());

        // -100.0 as u8 is 156
        println!("-100.0 as a u8 is: {}", (-100.0_f32).to_int_unchecked::<u8>());

        // nan as u8 is 0
        println!("nan as a u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
    }



    

}