// demonstrating the usage of Vec<T> in rust and computing the sum of the elems 

fn sum_of_vec(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("Sum of the elements in the vector: {}", sum_of_vec(&v));
}

