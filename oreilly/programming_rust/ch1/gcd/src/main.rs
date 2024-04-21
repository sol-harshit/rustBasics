fn main() {
    println!("Hello, world!");
    let n = 14;
    let m = 21;
    println!("The greatest common divisor of {} and {} is {}", n, m, gcd(n, m));
}

// the below gcd function is a simple implementation of Euclid's algorithm
// Euclid's algorithm is an efficient way to find the greatest common divisor of two numbers
// it takes two numbers and keep dividing the smaller number by the larger number until the remainder is zero
// the last non-zero remainder is the greatest common divisor of the two numbers

// the function takes two arguments n and m of type u64 and returns a u64
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(m!=0 && m!=0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m%n;
    }
    n
}

// writing tests for the gcd function
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}
