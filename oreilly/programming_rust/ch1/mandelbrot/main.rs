use num::Complex;
use std::str::FromStr;


fn main() {
    let c: (f64, f64) = parse_pair("1.25,0.0625", ',').expect("error parsing complex f64 number");
    println!("complex number: {:?}", c);

    let d: (i32, i32) = parse_pair("1,20", ',').expect("error parsing complex i32 number");
    println!("complex number: {:?}", d);
}

// The function from_str() is used to convert a string into a number. It is a generic function that
// can convert a string into any type that implements the FromStr trait.
// The FromStr trait is a trait that is used to convert a string into a number. It is implemented for
// all the primitive types like i32, f64, etc.
// The from_str() function returns a Result type which is an enum with two variants Ok and Err.
// The Ok variant is used to return the value if the conversion is successful and the Err variant is
// used to return an error if the conversion fails.
fn parse_pair<T:FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None, 
        Some(index) => {
            // &s[..index] is a substring of s from the beginning to index - 1
            // &s[index + 1..] is a substring of s from index + 1 to the end
            match(T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/**
 * we can get a decent approximation of the Mandelbrot set if we limit the number of iterations
 * The return type of this function is Option<usize> which is an enum with two variants Some and None.
 */
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex{re: 0., im: 0.};
    for i in 0..limit { // loops from 0 to limit - 1, limit is exclusive
        if z.norm_sqr() > 4. { // the norm_sqr() method returns the square returns the square of the z's
            // distance from the origin
            return Some(i); // Some is a variant of the Option enum, and it is used to return a value
        }
        z *= z + c;
    }
    None // None is a variant of the Option enum, and it is used to return no value, 
    // since option is an enum, it can be used in a match expression with the Some and None variants
}

/**
 * complex_square_add_loop
 * This function takes a complex number c and starts from z = 0. It keeps squaring z and adding c to
 * it. This function will loop forever. 
 * In Rust Complex<f64> is a struct with two fields re and im.
 * The Complex<f64> struct is defined in the num crate.
 * struct Complex<T> {
 *      // real part
 *      re: T,
 *      // imaginary part
 *      im: T,
 *  }
 * The Mandelbrot set is a set of complex numbers c for which the function f(z) = z^2 + c does not
 * fly off to infinity when iterated from z = 0.
 * 
 * Since complex have two components re and im, they will be treated as x & y coordinates of a point
 * on the cartesian plane & color of the point is black if c is in the Mandelbrot set and white otherwise.
 */
fn complex_square_add_loop(c: Complex<f64>) {
    // the complex number z have two components re and im, re is the real part and im is the imaginary
    let mut z = Complex {re: 0., im: 0.};
    loop {
        z *= z + c; 
    }
}

/**
 * This function starts from x = 0 and keeps squaring x and adding c to it. So it will keep growing
 * and will never reach zero. This function will loop forever.
 */

fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x *= x + c;
    }
}


/**
 * This function will loop forever because the value of x will always be positive and will never
 * reach zero.
 * but the compiler will not allow this code to compile because the function will never return a
 * value.
 */
fn square_loop(mut x: f64) {
    loop {
        x *= x;
    }
}
