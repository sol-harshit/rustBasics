// below example talks about printing in Rust programming language.
// print! is a macro that prints the string to the console without adding a newline at the end to (io::stdout)
// println! is a macro that prints the string to the console and adds a newline at the end
// format! is a macro that returns a formatted string
// eprint! is a macro that prints the string to the standard error without adding a newline at the end to (io::stderr)
// eprintln! is a macro that prints the string to the standard error and adds a newline at the end

fn main() {
    print!("This ");
    print!("is ");
    print!("a ");
    print!("single ");
    print!("line ");
    print!("print ");
    print!("statement ");
    print!("without ");
    print!("a ");
    print!("newline ");
    print!("at ");
    print!("the ");
    print!("end");
    println!();

    println!("This is a single line print statement with a newline at the end");

    let name = "Alice"; // let helps us declare a variable same as 'name := "Alice"' in Go
    let age = 30;

    // {} is a placeholder for the variables that are passed to the println! macro and outputs will be in string format
    println!("Hello, my name is {} and I am {} years old", name, age);

    // trying above with format! macro
    let formatted_string = format!("Hello, my name is {} and I am {} years old", name, age);
    println!("{}", formatted_string);

    eprint!("This ");
    eprint!("is ");
    eprint!("a ");
    eprint!("single ");
    eprint!("line ");
    eprint!("print ");
    eprint!("statement ");
    eprint!("without ");
    eprint!("a ");
    eprint!("newline ");
    eprint!("at ");
    eprint!("the ");
    eprint!("end");
    eprintln!();

    eprintln!("This is a single line print statement with a newline at the end");

    // printing the positional arguments
    // notice how we can specify the positional arguments in the println! macro every 0 will be replaced by the first argument, 1 by the second argument and so on
    println!("{0} did a transacion to {1}, and {1} received {2} from {0}", "Alice", "Bob", "1000");

    // printing the named arguments
    // notice how we can specify the named arguments in the println! macro
    println!("{sender} did a transacion to {receiver}, and {receiver} received {amount} from {sender}", sender="Alice", receiver="Bob", amount="1000");

    // printing the mixed arguments
    // notice how we can specify the mixed arguments in the println! macro
    println!("{0} did a transacion to {receiver}, and {receiver} received {amount} from {0}", "Alice", receiver="Bob", amount="1000");

    // printing the debug arguments
    // notice how we can specify the debug arguments in the println! macro
    println!("{:?}", ("Alice", 30));

    // printing the different types of arguments
    // notice how we can specify the different types of arguments in the println! macro
    // we need to use the following format specifiers:
    // {:b} for binary
    // {:o} for octal
    // {:x} for hexadecimal
    // {:X} for hexadecimal with uppercase letters
    // {:e} for scientific notation
    // {:E} for scientific notation with uppercase letters
    // {:p} for pointer
    // {:?} for debug
    println!("{:b} {:o} {:x} {:X} {:e} {:E} {:p}", 10, 10, 10, 10, 10.0, 10.0, &10);

    // right align the output
    // notice how we can specify the right align the output in the println! macro
    println!("{:>10}", "Alice");

    // padding the output
    // notice how we can specify the padding the output in the println! macro
    println!("{:10}", "Alice");

    // pad the output with a specific character
    // notice how we can specify the pad the output with a specific character in the println! macro
    println!("{:0>10}", 1);

    // you can directly use the variable names in the println! macro
    // notice how we can directly use the variable names in the println! macro
    let name = "Alice";
    println!("{name}");


    // to compile and run this program, use the following command:
    // rustc printing.rs -o printing.out && ./printing.out
    // expected output:
    // This is a single line print statement without a newline at the end
    // This is a single line print statement with a newline at the end
    // Hello, my name is Alice and I am 30 years old
    // Hello, my name is Alice and I am 30 years old
    // This is a single line print statement without a newline at the end
    // This is a single line print statement with a newline at the end
    // Alice did a transacion to Bob, and Bob received 1000 from Alice
    // Alice did a transacion to Bob, and Bob received 1000 from Alice
    // ("Alice", 30)
    // 1010 a 10 A 1.0e1 1.0E1 0x7fff5b3b3b68
    //      Alice
    // Alice
    // 0000000001
    // Alice

}